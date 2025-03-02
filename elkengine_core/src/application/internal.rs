use glutin::{
    display::GetGlDisplay,
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::{GlSurface, SurfaceAttributesBuilder, WindowSurface},
};
use imgui_glow_renderer::glow::{self, HasContext};
use raw_window_handle::HasWindowHandle;

pub struct InternalApplication {
    window: winit::window::Window,
    gl_surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
    gl_context: glutin::context::PossiblyCurrentContext,
    gl_imgui_renderer: imgui_glow_renderer::Renderer,
    glow_context: glow::Context,
    imgui_context: imgui::Context,
    winit_platform: imgui_winit_support::WinitPlatform,
    textures: imgui::Textures<glow::Texture>,
}

impl InternalApplication {
    const INITIAL_WINDOW_SIZE: winit::dpi::LogicalSize<u32> =
        winit::dpi::LogicalSize::new(1280, 720);

    pub fn new(event_loop: &winit::event_loop::ActiveEventLoop) -> Self {
        let (window, config) = super::opengl::create_window(
            event_loop,
            Some(Self::window_attributes()),
        )
        .expect("Failed to create Window");

        let new_context = super::opengl::create_context(&window, &config);

        let raw_window_handle =
            window.window_handle().ok().map(|wh| wh.as_raw());

        let surface_attributes =
            SurfaceAttributesBuilder::<WindowSurface>::new()
                //.with_srgb(Some(true))
                .build(
                    raw_window_handle.unwrap(),
                    std::num::NonZeroU32::new(Self::INITIAL_WINDOW_SIZE.width)
                        .unwrap(),
                    std::num::NonZeroU32::new(Self::INITIAL_WINDOW_SIZE.height)
                        .unwrap(),
                );

        let surface = unsafe {
            config
                .display()
                .create_window_surface(&config, &surface_attributes)
                .expect("Failed to create OpenGL surface")
        };

        let context = new_context
            .make_current(&surface)
            .expect("Failed to make OpenGL the current context");

        let glow_context: imgui_glow_renderer::glow::Context = unsafe {
            imgui_glow_renderer::glow::Context::from_loader_function_cstr(|s| {
                context.display().get_proc_address(s).cast()
            })
        };

        let (mut imgui_context, winit_platform) =
            super::imgui::create_context(&window)
                .expect("Failed to initialize imgui");

        let mut textures = imgui::Textures::<glow::Texture>::default();

        let renderer = imgui_glow_renderer::Renderer::new(
            &glow_context,
            &mut imgui_context,
            &mut textures,
            false,
        )
        .expect("Failed to create imgui_glow_renderer");

        unsafe { glow_context.enable(glow::FRAMEBUFFER_SRGB) };

        Self {
            window,
            gl_surface: surface,
            gl_context: context,
            gl_imgui_renderer: renderer,
            glow_context,
            imgui_context,
            winit_platform,
            textures,
        }
    }

    pub fn prepare_frame(&mut self) {
        self.winit_platform
            .prepare_frame(self.imgui_context.io_mut(), &self.window)
            .expect("Failed to prepare frame");

        self.window.request_redraw();
    }

    pub fn redraw(
        &mut self,
        delta_time: std::time::Duration,
        layer_stack: &mut super::layer::LayerStack,
    ) {
        self.imgui_context.io_mut().update_delta_time(delta_time);

        unsafe {
            // self.glow_context.clear_color(0.5, 0.5, 0.1, 1.0);
            self.glow_context.clear(glow::COLOR_BUFFER_BIT);
        }

        for layer in layer_stack.iter_mut() {
            layer.update(delta_time);
        }

        let ui = self.imgui_context.frame();
        ui.dockspace_over_main_viewport();

        for layer in layer_stack.iter_mut() {
            layer.update_imgui(ui, &self.glow_context, &mut self.textures);
        }

        self.winit_platform.prepare_render(ui, &self.window);
        let draw_data = self.imgui_context.render();
        self.gl_imgui_renderer
            .render(&self.glow_context, &self.textures, draw_data)
            .expect("Error rendering imgui");

        self.gl_surface
            .swap_buffers(&self.gl_context)
            .expect("Failed to swap OpenGL buffers");

        // self.imgui_context.update_platform_windows();
    }

    pub fn on_window_resize(
        &mut self,
        window_id: winit::window::WindowId,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) {
        if new_size.width > 0 && new_size.height > 0 {
            self.gl_surface.resize(
                &self.gl_context,
                std::num::NonZeroU32::new(new_size.width).unwrap(),
                std::num::NonZeroU32::new(new_size.height).unwrap(),
            );
        }

        self.winit_platform.handle_event(
            self.imgui_context.io_mut(),
            &self.window,
            &winit::event::Event::<()>::WindowEvent {
                window_id,
                event: winit::event::WindowEvent::Resized(new_size),
            },
        );
    }

    pub fn on_unhandled_event(
        &mut self,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.winit_platform.handle_event(
            self.imgui_context.io_mut(),
            &self.window,
            &winit::event::Event::<()>::WindowEvent { window_id, event },
        );
    }

    pub fn on_exit(&mut self) {
        self.gl_imgui_renderer.destroy(&self.glow_context);
    }

    fn window_attributes() -> winit::window::WindowAttributes {
        winit::window::WindowAttributes::default()
            .with_title("Elktracer Editor")
            .with_resizable(true)
            .with_inner_size(Self::INITIAL_WINDOW_SIZE)
    }
}
