use crate::Result;

pub fn create_context(
    window: &winit::window::Window,
) -> Result<(imgui::Context, imgui_winit_support::WinitPlatform)> {
    let mut imgui_context = imgui::Context::create();
    // imgui_context.set_ini_filename(None);

    let mut winit_platform =
        imgui_winit_support::WinitPlatform::new(&mut imgui_context);
    winit_platform.attach_window(
        imgui_context.io_mut(),
        window,
        imgui_winit_support::HiDpiMode::Rounded,
    );

    // Enable docking
    imgui_context
        .io_mut()
        .config_flags
        .insert(imgui::ConfigFlags::DOCKING_ENABLE);
    // imgui_context.io_mut().config_docking_with_shift = true;

    imgui_context
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    imgui_context.io_mut().font_global_scale =
        (1.0 / winit_platform.hidpi_factor()) as f32;

    Ok((imgui_context, winit_platform))
}
