pub trait Element {
    fn render(delta_time: std::time::Duration);
}

pub struct RenderOptionsWindow {}

impl RenderOptionsWindow {
    pub fn new() -> Self {
        Self {}
    }
}

impl Element for RenderOptionsWindow {
    fn render(_delta_time: std::time::Duration) {
        todo!()
    }
}
