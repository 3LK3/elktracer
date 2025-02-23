use std::{
    error::Error,
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
};

use ash::{ext::debug_utils, khr::surface, vk, Entry};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;

use elktracer_core::profile_scope;

#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

const REQUIRED_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

pub struct VulkanContext {
    pub instance: ash::Instance,
    pub surface: surface::Instance,
    pub surface_khr: vk::SurfaceKHR,
    pub physical_device: vk::PhysicalDevice,
    graphics_queue_index: u32,
    present_queue_index: u32,
    pub device: ash::Device,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
    pub command_pool: vk::CommandPool,
    //
    #[cfg(all(debug_assertions))]
    debug_utils: debug_utils::Instance,
    #[cfg(all(debug_assertions))]
    debug_utils_messenger: vk::DebugUtilsMessengerEXT,
}

pub struct QueueFamiliesIndices {
    pub graphics: u32,
    pub present: u32,
}

impl VulkanContext {
    pub fn initialize(
        window: &Window,
        name: &str,
    ) -> Result<Self, Box<dyn Error>> {
        profile_scope!("Initializing VulkanContext");

        let entry = Entry::linked();

        // log::trace!("Creating vulkan instance");
        let (instance, debug_utils, debug_utils_messenger) =
            create_instance(&entry, &window, name)
                .expect("Unable to create vulkan instance");

        // log::trace!("Creating vulkan surface");
        let surface = surface::Instance::new(&entry, &instance);

        // log::trace!("Creating vulkan khr surface");
        let surface_khr = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.display_handle().unwrap().into(),
                window.window_handle().unwrap().into(),
                None,
            )
            .unwrap()
        };

        let (physical_device, graphics_queue_index, present_queue_index) =
            pick_physical_device(&instance, &surface, surface_khr)
                .expect("Unable to pick physical device");

        let (device, graphics_queue, present_queue) =
            create_logical_device_with_queue(
                &instance,
                physical_device,
                graphics_queue_index,
                present_queue_index,
            )
            .expect("Unable to create logical device or graphics queue");

        let command_pool = {
            let command_pool_info = vk::CommandPoolCreateInfo::default()
                .queue_family_index(graphics_queue_index)
                .flags(vk::CommandPoolCreateFlags::empty());
            unsafe { device.create_command_pool(&command_pool_info, None)? }
        };

        Ok(Self {
            instance,
            surface,
            surface_khr,
            physical_device,
            graphics_queue_index,
            present_queue_index,
            device,
            graphics_queue,
            present_queue,
            command_pool,
            #[cfg(all(debug_assertions))]
            debug_utils,
            #[cfg(all(debug_assertions))]
            debug_utils_messenger,
        })
    }

    pub fn get_queue_indices(&self) -> QueueFamiliesIndices {
        QueueFamiliesIndices {
            graphics: self.graphics_queue_index,
            present: self.present_queue_index,
        }
    }
}

impl Drop for VulkanContext {
    fn drop(&mut self) {
        profile_scope!("Destroying VulkanContext");
        unsafe {
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
            self.surface.destroy_surface(self.surface_khr, None);
            self.debug_utils.destroy_debug_utils_messenger(
                self.debug_utils_messenger,
                None,
            );
            self.instance.destroy_instance(None);
        }
    }
}

fn create_instance(
    entry: &Entry,
    window: &Window,
    title: &str,
) -> Result<
    (
        ash::Instance,
        debug_utils::Instance,
        vk::DebugUtilsMessengerEXT,
    ),
    Box<dyn Error>,
> {
    profile_scope!("Creating vulkan instance");

    let app_name = CString::new(title)?;
    let engine_name = CString::new("No Engine")?;
    let app_info = vk::ApplicationInfo::default()
        .application_name(app_name.as_c_str())
        .application_version(vk::make_api_version(0, 0, 1, 0))
        .engine_name(engine_name.as_c_str())
        .engine_version(vk::make_api_version(0, 0, 1, 0))
        .api_version(vk::make_api_version(0, 1, 0, 0));

    let extension_names = get_required_extensions(window);
    let instance_create_flags = vk::InstanceCreateFlags::default();
    let mut instance_create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names)
        .flags(instance_create_flags);

    let (_layer_names, layer_names_ptrs) = get_layer_names_and_pointers();

    if ENABLE_VALIDATION_LAYERS {
        check_validation_layer_support(entry);
        instance_create_info =
            instance_create_info.enabled_layer_names(&layer_names_ptrs);
    }

    let instance =
        unsafe { entry.create_instance(&instance_create_info, None).unwrap() };

    // Vulkan debug report
    #[cfg(all(debug_assertions))]
    let create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
        .flags(vk::DebugUtilsMessengerCreateFlagsEXT::empty())
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        )
        .pfn_user_callback(Some(vulkan_debug_callback));
    let debug_utils = debug_utils::Instance::new(entry, &instance);
    let debug_utils_messenger = unsafe {
        debug_utils.create_debug_utils_messenger(&create_info, None)?
    };

    Ok((instance, debug_utils, debug_utils_messenger))
}

/// Check if the required validation set in `REQUIRED_LAYERS`
/// are supported by the Vulkan instance.
///
/// # Panics
///
/// Panic if at least one on the layer is not supported.
fn check_validation_layer_support(entry: &Entry) {
    let supported_layers =
        unsafe { entry.enumerate_instance_layer_properties().unwrap() };
    for required in REQUIRED_LAYERS.iter() {
        let found = supported_layers.iter().any(|layer| {
            let name = unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) };
            let name = name.to_str().expect("Failed to get layer name pointer");
            required == &name
        });

        if !found {
            panic!("Validation layer not supported: {}", required);
        }
    }
}

fn get_layer_names_and_pointers() -> (Vec<CString>, Vec<*const c_char>) {
    let layer_names = REQUIRED_LAYERS
        .iter()
        .map(|name| CString::new(*name).unwrap())
        .collect::<Vec<_>>();
    let layer_names_ptrs = layer_names
        .iter()
        .map(|name| name.as_ptr())
        .collect::<Vec<_>>();
    (layer_names, layer_names_ptrs)
}

fn get_required_extensions(window: &Window) -> Vec<*const i8> {
    let mut extensions = ash_window::enumerate_required_extensions(
        window.display_handle().unwrap().as_raw(),
    )
    .unwrap()
    .to_vec();

    if ENABLE_VALIDATION_LAYERS {
        extensions.push(debug_utils::NAME.as_ptr());
    }

    extensions
}

fn pick_physical_device(
    instance: &ash::Instance,
    surface: &surface::Instance,
    surface_khr: vk::SurfaceKHR,
) -> Result<(vk::PhysicalDevice, u32, u32), Box<dyn Error>> {
    profile_scope!("Picking vulkan physical device");

    let devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("No physical device found")
    };
    let mut graphics: Option<u32> = None;
    let mut present: Option<u32> = None;

    let device = devices
        .into_iter()
        .find(|device| {
            let device = *device;

            // Does device supports graphics and present queues
            let props = unsafe {
                instance.get_physical_device_queue_family_properties(device)
            };

            for (index, family) in
                props.iter().filter(|f| f.queue_count > 0).enumerate()
            {
                let index = index as u32;
                graphics = None;
                present = None;

                if family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                    && family.queue_flags.contains(vk::QueueFlags::COMPUTE)
                    && graphics.is_none()
                {
                    graphics = Some(index);
                }

                let present_support = unsafe {
                    surface
                        .get_physical_device_surface_support(
                            device,
                            index,
                            surface_khr,
                        )
                        .expect("Failed to get surface support")
                };
                if present_support && present.is_none() {
                    present = Some(index);
                }

                if graphics.is_some() && present.is_some() {
                    break;
                }
            }

            // Does device support desired extensions
            let extension_props = unsafe {
                instance
                    .enumerate_device_extension_properties(device)
                    .expect("Failed to get device ext properties")
            };

            let extension_support = extension_props.iter().any(|ext| {
                let name =
                    unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) };
                ash::khr::swapchain::NAME == name
            });

            // Does the device have available formats for the given surface
            let formats = unsafe {
                surface
                    .get_physical_device_surface_formats(device, surface_khr)
                    .expect("Failed to get physical device surface formats")
            };

            // Does the device have available present modes for the given surface
            let present_modes =
                unsafe {
                    surface
                .get_physical_device_surface_present_modes(device, surface_khr)
                .expect("Failed to get physical device surface present modes")
                };

            graphics.is_some()
                && present.is_some()
                && extension_support
                && !formats.is_empty()
                && !present_modes.is_empty()
        })
        .expect("Could not find a suitable device");

    unsafe {
        let props = instance.get_physical_device_properties(device);
        let device_name = CStr::from_ptr(props.device_name.as_ptr());
        log::debug!("Selected physical device: {device_name:?}");
    }

    Ok((device, graphics.unwrap(), present.unwrap()))
}

fn create_logical_device_with_queue(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    graphics_queue_index: u32,
    present_queue_index: u32,
) -> Result<(ash::Device, vk::Queue, vk::Queue), Box<dyn Error>> {
    profile_scope!("Creating vulkan logical device and graphics queue");

    let queue_priorities = [1.0f32];
    let queue_create_infos = {
        let mut indices = vec![graphics_queue_index, present_queue_index];
        indices.dedup();

        indices
            .iter()
            .map(|index| {
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(*index)
                    .queue_priorities(&queue_priorities)
            })
            .collect::<Vec<_>>()
    };

    let device_extensions_ptrs = [ash::khr::swapchain::NAME.as_ptr()];

    let device_create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_create_infos)
        .enabled_extension_names(&device_extensions_ptrs);

    let device = unsafe {
        instance.create_device(physical_device, &device_create_info, None)?
    };

    let graphics_queue =
        unsafe { device.get_device_queue(graphics_queue_index, 0) };
    let present_queue =
        unsafe { device.get_device_queue(present_queue_index, 0) };

    Ok((device, graphics_queue, present_queue))
}

unsafe extern "system" fn vulkan_debug_callback(
    flag: vk::DebugUtilsMessageSeverityFlagsEXT,
    typ: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    use vk::DebugUtilsMessageSeverityFlagsEXT as Flag;

    let message = CStr::from_ptr((*p_callback_data).p_message);
    match flag {
        Flag::VERBOSE => log::debug!("{typ:?} - {message:?}"),
        Flag::INFO => log::info!("{typ:?} - {message:?}"),
        Flag::WARNING => log::warn!("{typ:?} - {message:?}"),
        _ => log::error!("{typ:?} - {message:?}"),
    }
    vk::FALSE
}
