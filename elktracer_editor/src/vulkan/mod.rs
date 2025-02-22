use std::error::Error;

use ash::vk;
use imgui::DrawData;
use imgui_rs_vulkan_renderer::Renderer;

pub mod context;
pub mod swap_chain;
pub mod renderer;

pub fn record_command_buffers(
    device: &ash::Device,
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    framebuffer: vk::Framebuffer,
    render_pass: vk::RenderPass,
    extent: vk::Extent2D,
    renderer: &mut Renderer,
    draw_data: &DrawData,
) -> Result<(), Box<dyn Error>> {
    unsafe {
        device.reset_command_pool(
            command_pool,
            vk::CommandPoolResetFlags::empty(),
        )?
    };

    let command_buffer_begin_info = vk::CommandBufferBeginInfo::default()
        .flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);
    unsafe {
        device
            .begin_command_buffer(command_buffer, &command_buffer_begin_info)?
    };

    let render_pass_begin_info = vk::RenderPassBeginInfo::default()
        .render_pass(render_pass)
        .framebuffer(framebuffer)
        .render_area(vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent,
        })
        .clear_values(&[vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [1.0, 1.0, 1.0, 1.0],
            },
        }]);

    unsafe {
        device.cmd_begin_render_pass(
            command_buffer,
            &render_pass_begin_info,
            vk::SubpassContents::INLINE,
        )
    };

    renderer.cmd_draw(command_buffer, draw_data)?;

    unsafe { device.cmd_end_render_pass(command_buffer) };

    unsafe { device.end_command_buffer(command_buffer)? };

    Ok(())
}
