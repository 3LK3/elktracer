use std::error::Error;

use ash::vk;
use imgui_rs_vulkan_renderer::{Options, Renderer};
use winit::{dpi::PhysicalSize, window};

use super::{
    context::VulkanContext, record_command_buffers, swap_chain::Swapchain,
};
use elktracer_core::profile_scope;

pub struct VulkanRenderer {
    pub renderer: Renderer,
    vulkan_context: VulkanContext,
    is_swapchain_dirty: bool,
    swapchain: Swapchain,
    image_available_semaphore: vk::Semaphore,
    render_finished_semaphore: vk::Semaphore,
    fence: vk::Fence,
    command_buffer: vk::CommandBuffer,
}

impl VulkanRenderer {
    pub fn new(
        window: &window::Window,
        imgui_context: &mut imgui::Context,
    ) -> Result<Self, Box<dyn Error>> {
        let vulkan_context = VulkanContext::initialize(&window, "Elktracer")
            .expect("Unable to initialize VulkanContext");

        let command_buffer = {
            let allocate_info = vk::CommandBufferAllocateInfo::default()
                .command_pool(vulkan_context.command_pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1);

            unsafe {
                vulkan_context
                    .device
                    .allocate_command_buffers(&allocate_info)
                    .unwrap()[0]
            }
        };

        let swapchain = Swapchain::new(&vulkan_context, window.inner_size())
            .expect("Unable to create Swapchain");

        let image_available_semaphore = {
            let semaphore_info = vk::SemaphoreCreateInfo::default();
            unsafe {
                vulkan_context
                    .device
                    .create_semaphore(&semaphore_info, None)
                    .expect("Unable to create image_available semaphore")
            }
        };

        let render_finished_semaphore = {
            let semaphore_info = vk::SemaphoreCreateInfo::default();
            unsafe {
                vulkan_context
                    .device
                    .create_semaphore(&semaphore_info, None)
                    .expect("Unable to create render_finished semaphore")
            }
        };

        let fence = {
            let fence_info = vk::FenceCreateInfo::default()
                .flags(vk::FenceCreateFlags::SIGNALED);
            unsafe {
                vulkan_context
                    .device
                    .create_fence(&fence_info, None)
                    .expect("Unable to create fence")
            }
        };

        // #[cfg(not(any(feature = "gpu-allocator", feature = "vk-mem")))]
        let renderer = Renderer::with_default_allocator(
            &vulkan_context.instance,
            vulkan_context.physical_device,
            vulkan_context.device.clone(),
            vulkan_context.graphics_queue,
            vulkan_context.command_pool,
            swapchain.render_pass,
            imgui_context,
            Some(Options {
                in_flight_frames: 1,
                ..Default::default()
            }),
        )
        .expect("Unable to create Renderer");

        Ok(Self {
            renderer,
            vulkan_context,
            is_swapchain_dirty: false,
            swapchain,
            image_available_semaphore,
            render_finished_semaphore,
            fence,
            command_buffer,
        })
    }

    pub fn begin_frame(&mut self, window: &window::Window) {
        // If swapchain must be recreated wait for windows to not be minimized anymore
        if self.is_swapchain_dirty {
            let PhysicalSize { width, height } = window.inner_size();
            if width > 0 && height > 0 {
                self.swapchain
                    .recreate(&self.vulkan_context)
                    .expect("Failed to recreate swapchain");

                self.renderer
                    .set_render_pass(self.swapchain.render_pass)
                    .expect("Failed to rebuild renderer pipeline");

                self.is_swapchain_dirty = false;
            } else {
                return;
            }
        }
    }

    pub fn end_frame(&mut self, imgui_draw_data: &imgui::DrawData) {
        unsafe {
            self.vulkan_context
                .device
                .wait_for_fences(&[self.fence], true, u64::MAX)
                .expect("Failed to wait ")
        };

        // Drawing the frame
        let next_image_result = unsafe {
            self.swapchain.loader.acquire_next_image(
                self.swapchain.khr,
                u64::MAX,
                self.image_available_semaphore,
                vk::Fence::null(),
            )
        };

        let image_index = match next_image_result {
            Ok((image_index, _)) => image_index,
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                self.is_swapchain_dirty = true;
                return;
            }
            Err(error) => {
                panic!("Error while acquiring next image. Cause: {}", error)
            }
        };

        unsafe {
            self.vulkan_context
                .device
                .reset_fences(&[self.fence])
                .expect("Failed to reset fences")
        };

        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let wait_semaphores = [self.image_available_semaphore];
        let signal_semaphores = [self.render_finished_semaphore];

        // Re-record commands to draw geometry
        record_command_buffers(
            &self.vulkan_context.device,
            self.vulkan_context.command_pool,
            self.command_buffer,
            self.swapchain.frame_buffers[image_index as usize],
            self.swapchain.render_pass,
            self.swapchain.extent,
            &mut self.renderer,
            imgui_draw_data,
        )
        .expect("Failed to record command buffer");

        let command_buffers = [self.command_buffer];
        let submit_info = [vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores)];

        unsafe {
            self.vulkan_context
                .device
                .queue_submit(
                    self.vulkan_context.graphics_queue,
                    &submit_info,
                    self.fence,
                )
                .expect("Failed to submit work to gpu.")
        };

        let swapchains = [self.swapchain.khr];
        let images_indices = [image_index];
        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&images_indices);

        let present_result = unsafe {
            self.swapchain
                .loader
                .queue_present(self.vulkan_context.present_queue, &present_info)
        };
        match present_result {
            Ok(is_suboptimal) if is_suboptimal => {
                self.is_swapchain_dirty = true;
            }
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                self.is_swapchain_dirty = true;
            }
            Err(error) => panic!("Failed to present queue. Cause: {}", error),
            _ => {}
        }
    }

    pub fn on_window_resized(&mut self, size: &PhysicalSize<u32>) {
        self.swapchain.window_size = *size;
        self.is_swapchain_dirty = true;
    }
}

impl Drop for VulkanRenderer {
    fn drop(&mut self) {
        profile_scope!("Destroying VulkanRenderer");

        unsafe {
            self.vulkan_context
                .device
                .device_wait_idle()
                .expect("Failed to wait for graphics device to idle");

            self.vulkan_context.device.destroy_fence(self.fence, None);

            self.vulkan_context
                .device
                .destroy_semaphore(self.image_available_semaphore, None);
            self.vulkan_context
                .device
                .destroy_semaphore(self.render_finished_semaphore, None);

            self.swapchain.destroy(&self.vulkan_context);

            self.vulkan_context.device.free_command_buffers(
                self.vulkan_context.command_pool,
                &[self.command_buffer],
            );
        };
    }
}
