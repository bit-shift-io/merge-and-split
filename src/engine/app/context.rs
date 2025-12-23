use crate::engine::app::graphics_helper::GraphicsHelper;
use crate::engine::app::window_helper::WindowHelper;
use crate::engine::app::input_helper::InputHelper;
use crate::engine::app::ui_helper::UIHelper;

pub struct Context {
    pub graphics: GraphicsHelper,
    pub window: WindowHelper,
    pub input: InputHelper,
    pub ui: UIHelper,
    pub dt: f32,
    pub frame_count: u64,
}

impl Context {
    pub fn new(graphics: GraphicsHelper, window: WindowHelper, input: InputHelper, ui: UIHelper) -> Self {
        Self {
            graphics,
            window,
            input,
            ui,
            dt: 0.0,
            frame_count: 0,
        }
    }
}
