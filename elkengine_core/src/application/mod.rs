pub mod imgui;
pub mod layer;
pub mod opengl;

mod internal;

use std::time::Instant;

use winit::{application::ApplicationHandler, event_loop::EventLoop};

use crate::Result;
use layer::Layer;

pub struct Application {
    application: Option<internal::InternalApplication>,
    should_close: bool,
    last_frame_time: Instant,
    last_frame_duration: std::time::Duration,
    layer_stack: layer::LayerStack,
}

impl Application {
    pub fn new() -> Self {
        Self {
            application: None,
            should_close: false,
            last_frame_time: Instant::now(),
            last_frame_duration: std::time::Duration::ZERO,
            layer_stack: layer::LayerStack::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        match event_loop.run_app(self) {
            Ok(_) => Ok(()),
            Err(error) => Err(Box::new(error)),
        }
    }

    pub fn add_layer(&mut self, layer: Box<dyn Layer>) {
        self.layer_stack.push_layer(layer);
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        log::trace!("ElkEngineApplicationHandler :: resume");

        if self.application.is_some() {
            log::warn!(
                "Application already initialized. TODO handle differently?"
            );
            return;
        }

        self.application = Some(internal::InternalApplication::new(event_loop));
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if self.application.is_none() {
            return;
        }

        let application = self.application.as_mut().unwrap();

        match event {
            winit::event::WindowEvent::CloseRequested => {
                log::trace!("Requested window close");
                self.should_close = true
            }
            winit::event::WindowEvent::Resized(new_size) => {
                log::trace!("Windows resized to {new_size:?}");
                application.on_window_resize(window_id, new_size);
            }
            winit::event::WindowEvent::RedrawRequested => {
                let now = Instant::now();
                self.last_frame_duration =
                    now.duration_since(self.last_frame_time);
                self.last_frame_time = now;

                application
                    .redraw(self.last_frame_duration, &mut self.layer_stack);
            }
            window_event => {
                application.on_unhandled_event(window_id, window_event);
            }
        }
    }

    fn about_to_wait(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
    ) {
        if self.application.is_none() {
            return;
        }

        if self.should_close {
            log::trace!("Exiting event loop");
            event_loop.exit();
            return;
        }

        self.application.as_mut().unwrap().prepare_frame();
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        log::trace!("ElkEngineApplicationHandler :: exiting");

        self.application.as_mut().unwrap().on_exit();
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
