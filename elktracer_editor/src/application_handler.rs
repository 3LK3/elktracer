use std::time::Instant;

use elktracer_core::profile_scope;
// use log::{trace, warn};
use winit::{application::ApplicationHandler, event_loop::ActiveEventLoop};

use crate::application::{Application, EditorApplication};

#[derive(Default)]
pub struct EditorApplicationHandler {
    should_close: bool,
    last_frame_time: Option<Instant>,
    last_frame_duration: std::time::Duration,
    application: Option<EditorApplication>,
}

impl EditorApplicationHandler {
    pub fn new() -> Self {
        Self {
            should_close: false,
            last_frame_time: Some(Instant::now()),
            last_frame_duration: std::time::Duration::ZERO,
            application: None,
        }
    }
}

impl ApplicationHandler for EditorApplicationHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        profile_scope!("Resuming application");

        if self.application.is_some() {
            log::warn!(
                "Application already initialized. TODO handle differently?"
            );
            return;
        }

        self.application = Some(
            EditorApplication::new(event_loop)
                .expect("Failed to initialize EditorApplication"),
        );
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _cause: winit::event::StartCause,
    ) {
        self.last_frame_duration = self.last_frame_time.unwrap().elapsed();
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
            // winit::event::WindowEvent::RedrawRequested => {
            // application.update(self.last_frame_duration);
            // }
            window_event => {
                application.on_unhandled_event(window_id, window_event);
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.application.is_none() {
            return;
        }

        if self.should_close {
            log::trace!("Exiting event loop");
            event_loop.exit();
            return;
        }

        self.application
            .as_mut()
            .unwrap()
            .update(self.last_frame_duration);
    }

    // fn about_to_wait(
    //     &mut self,
    //     event_loop: &winit::event_loop::ActiveEventLoop,
    // ) {
    //     if self.application.is_none() {
    //         return;
    //     }
    //     // let application = self.application.as_mut().unwrap();
    //     // if self.should_close {
    //     //     log::trace!("Exiting event loop");
    //     //     application.on_exit();
    //     //     event_loop.exit();
    //     //     return;
    //     // }
    //     // let renderer = self.vulkan_renderer.as_mut().unwrap();
    //     // let window = self.window.as_ref().unwrap();
    //     // renderer.begin_frame(&window);
    //     // let imgui = self.imgui_context.as_mut().unwrap();
    //     // let platform = self.platform.as_mut().unwrap();
    //     // {
    //     //     // Generate UI
    //     //     platform
    //     //         .prepare_frame(imgui.io_mut(), &window)
    //     //         .expect("Failed to prepare frame");
    //     //     let ui: &mut imgui::Ui = imgui.frame();
    //     //     ui.dockspace_over_main_viewport();
    //     //     unsafe {
    //     //         let dock_id = imgui::sys::igGetID_Str(imgui::ImStr::as_ptr(
    //     //             &imgui::ImString::new("Testyyyy"),
    //     //         ));
    //     //         let dock_flags: imgui::sys::ImGuiDockNodeFlags = 0;
    //     //         imgui::sys::igDockSpace(
    //     //             dock_id,
    //     //             ImVec2::zero(),
    //     //             dock_flags,
    //     //             imgui::sys::ImGuiWindowClass_ImGuiWindowClass(),
    //     //         );
    //     //     };
    //     //     ui.window("Test")
    //     //         .size([300.0, 110.0], imgui::Condition::FirstUseEver)
    //     //         .build(|| {
    //     //             ui.text_wrapped("Hello world!");
    //     //         });
    //     //     // unsafe {
    //     //     //     // imgui::sys::igDockSpaceOverViewport(
    //     //     //     //     imgui::sys::igGetMainViewport(),
    //     //     //     //     0,
    //     //     //     //     null_mut(),
    //     //     //     // );
    //     //     //     let dock_id = imgui::sys::igGetID_Str(imgui::ImStr::as_ptr(
    //     //     //         &imgui::ImString::new("Test"),
    //     //     //     ));
    //     //     //     let dock_flags: imgui::sys::ImGuiDockNodeFlags = 0;
    //     //     //     imgui::sys::igDockSpace(
    //     //     //         dock_id,
    //     //     //         ImVec2::zero(),
    //     //     //         dock_flags,
    //     //     //         imgui::sys::ImGuiWindowClass_ImGuiWindowClass(),
    //     //     //     );
    //     //     //     if !self.is_gui_setup {
    //     //     //         // let win = imgui::sys::igGetCurrentWindow();
    //     //     //         let mut size = ImVec2::zero();
    //     //     //         imgui::sys::igGetWindowSize(&mut size);
    //     //     //         imgui::sys::igDockBuilderAddNode(
    //     //     //             dock_id,
    //     //     //             dock_flags | imgui::sys::ImGuiDockNodeFlags_DockSpace,
    //     //     //         );
    //     //     //         imgui::sys::igDockBuilderSetNodeSize(dock_id, size);
    //     //     //         imgui::sys::igDockBuilderDockWindow(
    //     //     //             imgui::ImStr::as_ptr(&imgui::ImString::new(
    //     //     //                 "Test WHAAAT",
    //     //     //             )),
    //     //     //             dock_id,
    //     //     //         );
    //     //     //         imgui::sys::igDockBuilderFinish(dock_id);
    //     //     //         self.is_gui_setup = true;
    //     //     //     }
    //     //     //     // ui.window("Test")
    //     //     //     //     .size([300.0, 110.0], imgui::Condition::FirstUseEver)
    //     //     //     //     .build(|| {
    //     //     //     //         ui.text_wrapped("Hello world!");
    //     //     //     //     });
    //     //     //     // ui.window("Hello world")
    //     //     //     //     .size([300.0, 110.0], imgui::Condition::FirstUseEver)
    //     //     //     //     .build(|| {
    //     //     //     //         ui.text_wrapped("Hello world!");
    //     //     //     //         ui.button("This...is...imgui-rs!");
    //     //     //     //         ui.separator();
    //     //     //     //         let mouse_pos = ui.io().mouse_pos;
    //     //     //     //         ui.text(format!(
    //     //     //     //             "Mouse Position: ({:.1},{:.1})",
    //     //     //     //             mouse_pos[0], mouse_pos[1]
    //     //     //     //         ));
    //     //     //     //     });
    //     //     // }
    //     //     platform.prepare_render(ui, &window);
    //     // }
    //     // let imgui_draw_data = &imgui.render();
    //     // renderer.end_frame(imgui_draw_data);
    //     // // imgui.update_platform_windows();
    // }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        profile_scope!("Exiting application");

        self.application.as_mut().unwrap().on_exit();
    }
}
