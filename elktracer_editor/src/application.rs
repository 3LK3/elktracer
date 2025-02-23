use std::error::Error;

use elktracer_core::raytracer::Raytracer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::vulkan::renderer::VulkanRenderer;

pub struct EditorApplication {
    window: winit::window::Window,
    vulkan_renderer: VulkanRenderer,
    platform: imgui_winit_support::WinitPlatform,
    imgui_context: imgui::Context,
}

impl EditorApplication {}

pub trait Application {
    fn new(event_loop: &ActiveEventLoop) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn update(&mut self, delta_time: std::time::Duration);
    fn on_exit(&self);
    fn on_unhandled_event(&mut self, window_id: WindowId, event: WindowEvent);
    fn on_window_resize(
        &mut self,
        window_id: winit::window::WindowId,
        new_size: PhysicalSize<u32>,
    );
}

impl Application for EditorApplication {
    fn new(event_loop: &ActiveEventLoop) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        let window =
            create_window(event_loop, "Elktracer", LogicalSize::new(1280, 720))
                .expect("Unable to create window");

        let mut imgui = imgui::Context::create();
        // imgui.set_ini_filename(None);
        imgui
            .io_mut()
            .config_flags
            .insert(imgui::ConfigFlags::DOCKING_ENABLE);
        // imgui.io_mut().config_flags |= imgui::ConfigFlags::VIEWPORTS_ENABLE;
        imgui.io_mut().config_docking_with_shift = true;

        let mut platform = WinitPlatform::new(&mut imgui);

        let hidpi_factor = platform.hidpi_factor();
        // let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);

        let vulkan_renderer = VulkanRenderer::new(&window, &mut imgui)
            .expect("Failed to create VulkanRenderer");

        Ok(Self {
            window,
            vulkan_renderer,
            platform,
            imgui_context: imgui,
        })
    }

    fn on_exit(&self) {
        log::trace!("on_exit just fyi atm");
    }

    fn update(&mut self, delta_time: std::time::Duration) {
        self.imgui_context.io_mut().update_delta_time(delta_time);

        self.vulkan_renderer.begin_frame(&self.window);

        // Generate UI
        self.platform
            .prepare_frame(self.imgui_context.io_mut(), &self.window)
            .expect("Failed to prepare frame");

        let ui: &mut imgui::Ui = self.imgui_context.frame();
        ui.dockspace_over_main_viewport();

        ui.window("Test")
            .size([300.0, 110.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text_wrapped("Hello world!");
                if ui.button("Generate image") {
                    log::info!("Generating image ...");
                    let aspect_ratio: f64 = 16.0 / 9.0;
                    let image_width = 600;
                    let raytracer = Raytracer::new(image_width, aspect_ratio);
                    raytracer.render_image();
                }
            });

        self.platform.prepare_render(ui, &self.window);

        let imgui_draw_data = self.imgui_context.render();

        self.vulkan_renderer.end_frame(imgui_draw_data);

        // imgui.update_platform_windows();
    }

    fn on_unhandled_event(&mut self, window_id: WindowId, event: WindowEvent) {
        self.platform.handle_event(
            self.imgui_context.io_mut(),
            &self.window,
            &Event::<()>::WindowEvent { window_id, event },
        );
    }

    fn on_window_resize(
        &mut self,
        window_id: winit::window::WindowId,
        new_size: PhysicalSize<u32>,
    ) {
        self.vulkan_renderer.on_window_resized(&new_size);

        self.platform.handle_event(
            self.imgui_context.io_mut(),
            &self.window,
            &Event::<()>::WindowEvent {
                window_id,
                event: winit::event::WindowEvent::Resized(new_size),
            },
        );
    }
}

fn create_window(
    event_loop: &winit::event_loop::ActiveEventLoop,
    title: &str,
    size: LogicalSize<i32>,
) -> Result<winit::window::Window, Box<dyn Error>> {
    log::trace!("Create window");

    let window_attributes = winit::window::WindowAttributes::default()
        .with_title(title)
        .with_resizable(true)
        .with_inner_size(size);

    Ok(event_loop.create_window(window_attributes)?)
}
