use glutin::{
    config::ConfigTemplateBuilder, context::ContextAttributesBuilder,
    display::GetGlDisplay, prelude::GlDisplay,
};
use raw_window_handle::HasWindowHandle;
use winit::window::WindowAttributes;

use crate::Result;

pub fn create_context(
    window: &winit::window::Window,
    config: &glutin::config::Config,
) -> glutin::context::NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());
    let context_attributes =
        ContextAttributesBuilder::new().build(raw_window_handle);
    unsafe {
        config
            .display()
            .create_context(config, &context_attributes)
            .expect("Failed to create OpenGL context")
    }
}

pub fn create_window(
    event_loop: &winit::event_loop::ActiveEventLoop,
    window_attibutes: Option<WindowAttributes>,
) -> Result<(winit::window::Window, glutin::config::Config)> {
    let (window, config) = glutin_winit::DisplayBuilder::new()
        .with_window_attributes(window_attibutes)
        .build(event_loop, ConfigTemplateBuilder::new(), |mut configs| {
            configs.next().unwrap()
        })
        .expect("Failed to create OpenGL window");

    Ok((window.unwrap(), config))
}
