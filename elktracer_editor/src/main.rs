pub mod application;
pub mod application_handler;
pub mod ui;
pub mod vulkan;

use std::process::ExitCode;

use application_handler::EditorApplicationHandler;
use winit::event_loop::EventLoop;

fn main() -> ExitCode {
    elktracer_core::logging::initialize();
    log::info!("Starting Elktracer Editor");

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    match event_loop.run_app(&mut EditorApplicationHandler::new()) {
        Ok(_) => return ExitCode::SUCCESS,
        Err(err) => {
            log::error!("Error: {err}");
            return ExitCode::FAILURE;
        }
    }
}
