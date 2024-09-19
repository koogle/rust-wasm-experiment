use crate::ui::UIElement;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub enum TextSize {
    Small,
    Normal,
    Large,
    ExtraLarge,
}

pub struct Text {
    content: String,
    size: TextSize,
}

impl Text {
    pub fn new(content: String, size: TextSize) -> Text {
        Text { content, size }
    }

    fn font_size(&self) -> &str {
        match self.size {
            TextSize::Small => "10px",
            TextSize::Normal => "12px",
            TextSize::Large => "16px",
            TextSize::ExtraLarge => "24px",
        }
    }
}

impl UIElement for Text {
    fn render(&self, context: &CanvasRenderingContext2d, x: f64, y: f64) {
        context.set_font(&format!("{} 'Press Start 2P'", self.font_size()));
        context.set_fill_style(&JsValue::from_str("black"));
        context.set_text_align("center");
        context.set_text_baseline("middle");
        context.fill_text(&self.content, x, y).unwrap();
    }
}
