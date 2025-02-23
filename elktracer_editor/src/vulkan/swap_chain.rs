use std::error::Error;

use ash::{khr, vk};
use winit::dpi::PhysicalSize;

use super::context::VulkanContext;
use elktracer_core::profile_scope;

pub struct Swapchain {
    pub loader: khr::swapchain::Device,
    pub khr: vk::SwapchainKHR,
    pub extent: vk::Extent2D,
    images: Vec<vk::Image>,
    image_views: Vec<vk::ImageView>,
    pub render_pass: vk::RenderPass,
    pub frame_buffers: Vec<vk::Framebuffer>,
    pub window_size: PhysicalSize<u32>,
}

impl Swapchain {
    pub fn new(
        vulkan_context: &VulkanContext,
        window_size: PhysicalSize<u32>,
    ) -> Result<Self, Box<dyn Error>> {
        let (loader, khr, extent, format, images, image_views) =
            create_vulkan_swapchain(vulkan_context, &window_size)?;

        let render_pass =
            create_vulkan_render_pass(&vulkan_context.device, format)?;

        let frame_buffers = create_vulkan_frame_buffers(
            &vulkan_context.device,
            render_pass,
            extent,
            &image_views,
        )?;

        Ok(Self {
            loader,
            khr,
            extent,
            images,
            image_views,
            render_pass,
            frame_buffers,
            window_size,
        })
    }

    pub fn recreate(
        &mut self,
        vulkan_context: &VulkanContext,
    ) -> Result<(), Box<dyn Error>> {
        profile_scope!("Recreating the swapchain");

        unsafe { vulkan_context.device.device_wait_idle()? };

        self.destroy(vulkan_context);

        // Swapchain
        let (loader, khr, extent, format, images, image_views) =
            create_vulkan_swapchain(vulkan_context, &self.window_size)?;

        // Renderpass
        let render_pass =
            create_vulkan_render_pass(&vulkan_context.device, format)?;

        // Framebuffers
        let framebuffers = create_vulkan_frame_buffers(
            &vulkan_context.device,
            render_pass,
            extent,
            &image_views,
        )?;

        self.loader = loader;
        self.extent = extent;
        self.khr = khr;
        self.images = images;
        self.image_views = image_views;
        self.render_pass = render_pass;
        self.frame_buffers = framebuffers;

        Ok(())
    }

    pub fn destroy(&mut self, vulkan_context: &VulkanContext) {
        profile_scope!("Destroying swapchain");

        unsafe {
            self.frame_buffers.iter().for_each(|fb| {
                vulkan_context.device.destroy_framebuffer(*fb, None)
            });

            self.frame_buffers.clear();

            vulkan_context
                .device
                .destroy_render_pass(self.render_pass, None);

            self.image_views.iter().for_each(|v| {
                vulkan_context.device.destroy_image_view(*v, None)
            });

            self.image_views.clear();

            self.loader.destroy_swapchain(self.khr, None);
        }
    }
}

type CreateSwapchainResult = (
    khr::swapchain::Device,
    vk::SwapchainKHR,
    vk::Extent2D,
    vk::Format,
    Vec<vk::Image>,
    Vec<vk::ImageView>,
);

fn create_vulkan_swapchain(
    vulkan_context: &VulkanContext,
    window_size: &PhysicalSize<u32>,
) -> Result<CreateSwapchainResult, Box<dyn Error>> {
    profile_scope!("Creating vulkan swapchain");
    // Swapchain format
    let format = {
        let formats = unsafe {
            vulkan_context.surface.get_physical_device_surface_formats(
                vulkan_context.physical_device,
                vulkan_context.surface_khr,
            )?
        };
        if formats.len() == 1 && formats[0].format == vk::Format::UNDEFINED {
            vk::SurfaceFormatKHR {
                format: vk::Format::B8G8R8A8_UNORM,
                color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
            }
        } else {
            *formats
                .iter()
                .find(|format| {
                    format.format == vk::Format::B8G8R8A8_UNORM
                        && format.color_space
                            == vk::ColorSpaceKHR::SRGB_NONLINEAR
                })
                .unwrap_or(&formats[0])
        }
    };

    // Swapchain present mode
    let present_mode = {
        let present_modes = unsafe {
            vulkan_context
                .surface
                .get_physical_device_surface_present_modes(
                    vulkan_context.physical_device,
                    vulkan_context.surface_khr,
                )?
        };
        if present_modes.contains(&vk::PresentModeKHR::IMMEDIATE) {
            vk::PresentModeKHR::IMMEDIATE
        } else {
            vk::PresentModeKHR::FIFO
        }
    };

    let capabilities = unsafe {
        vulkan_context
            .surface
            .get_physical_device_surface_capabilities(
                vulkan_context.physical_device,
                vulkan_context.surface_khr,
            )?
    };

    // Swapchain extent
    let extent = {
        if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            let min = capabilities.min_image_extent;
            let max = capabilities.max_image_extent;
            vk::Extent2D {
                width: window_size.width.min(max.width).max(min.width),
                height: window_size.height.min(max.height).max(min.height),
            }
        }
    };

    // Swapchain image count
    let image_count = capabilities.min_image_count;

    // Swapchain
    let families_indices = vulkan_context.get_queue_indices();
    let queue_families_indices =
        [families_indices.graphics, families_indices.present];

    let create_info = {
        let mut builder = vk::SwapchainCreateInfoKHR::default()
            .surface(vulkan_context.surface_khr)
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT);

        builder = if families_indices.graphics != families_indices.present {
            builder
                .image_sharing_mode(vk::SharingMode::CONCURRENT)
                .queue_family_indices(&queue_families_indices)
        } else {
            builder.image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        };

        builder
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
    };

    let swapchain = khr::swapchain::Device::new(
        &vulkan_context.instance,
        &vulkan_context.device,
    );
    let swapchain_khr =
        unsafe { swapchain.create_swapchain(&create_info, None)? };

    // Swapchain images and image views
    let images = unsafe { swapchain.get_swapchain_images(swapchain_khr)? };
    let views = images
        .iter()
        .map(|image| {
            let create_info = vk::ImageViewCreateInfo::default()
                .image(*image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format.format)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });

            unsafe {
                vulkan_context.device.create_image_view(&create_info, None)
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    log::trace!(
        "\n\
        Swapchain\n\
        - format: {format:?}\n\
        - present mode: {present_mode:?}\n\
        - extent: {extent:?}\n\
        - image count: {image_count:?}"
    );

    Ok((
        swapchain,
        swapchain_khr,
        extent,
        format.format,
        images,
        views,
    ))
}

fn create_vulkan_render_pass(
    device: &ash::Device,
    format: vk::Format,
) -> Result<vk::RenderPass, Box<dyn Error>> {
    profile_scope!("Creating vulkan render pass");

    let attachment_descs = [vk::AttachmentDescription::default()
        .format(format)
        .samples(vk::SampleCountFlags::TYPE_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)];

    let color_attachment_refs = [vk::AttachmentReference::default()
        .attachment(0)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];

    let subpass_descs = [vk::SubpassDescription::default()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(&color_attachment_refs)];

    let subpass_deps = [vk::SubpassDependency::default()
        .src_subpass(vk::SUBPASS_EXTERNAL)
        .dst_subpass(0)
        .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .src_access_mask(vk::AccessFlags::empty())
        .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(
            vk::AccessFlags::COLOR_ATTACHMENT_READ
                | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        )];

    let render_pass_info = vk::RenderPassCreateInfo::default()
        .attachments(&attachment_descs)
        .subpasses(&subpass_descs)
        .dependencies(&subpass_deps);

    Ok(unsafe { device.create_render_pass(&render_pass_info, None)? })
}

fn create_vulkan_frame_buffers(
    device: &ash::Device,
    render_pass: vk::RenderPass,
    extent: vk::Extent2D,
    image_views: &[vk::ImageView],
) -> Result<Vec<vk::Framebuffer>, Box<dyn Error>> {
    profile_scope!("Creating vulkan framebuffers");

    Ok(image_views
        .iter()
        .map(|view| [*view])
        .map(|attachments| {
            let framebuffer_info = vk::FramebufferCreateInfo::default()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(extent.width)
                .height(extent.height)
                .layers(1);
            unsafe { device.create_framebuffer(&framebuffer_info, None) }
        })
        .collect::<Result<Vec<_>, _>>()?)
}
