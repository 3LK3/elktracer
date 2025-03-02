use imgui_glow_renderer::glow;

pub trait Layer {
    fn update(&mut self, delta_time: std::time::Duration);
    fn update_imgui(
        &mut self,
        ui: &mut imgui::Ui,
        glow_context: &glow::Context,
        textures: &mut imgui::Textures<glow::Texture>,
    );

    fn on_attached(&self);
    fn on_detached(&self);
}

#[derive(Default)]
pub struct LayerStack {
    layers: Vec<Box<dyn Layer>>,
}

impl LayerStack {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        layer.on_attached();
        self.layers.push(layer);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn Layer>> {
        self.layers.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Layer>> {
        self.layers.iter_mut()
    }
}
