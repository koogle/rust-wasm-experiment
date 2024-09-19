use crate::ui::UIElement;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Button {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    text: String,
}

impl Button {
    pub fn new(x: f64, y: f64, width: f64, height: f64, text: String) -> Button {
        Button {
            x,
            y,
            width,
            height,
            text,
        }
    }
}

impl UIElement for Button {
    fn render(&self, context: &CanvasRenderingContext2d, mouse_x: f64, mouse_y: f64) {
        let is_mouse_over = mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height;

        // Set button background color
        if is_mouse_over {
            context.set_fill_style(&JsValue::from_str("#e0e0e0")); // Light gray on hover
        } else {
            context.set_fill_style(&JsValue::from_str("white"));
        }

        // Draw button background
        context.fill_rect(self.x, self.y, self.width, self.height);

        // Draw button border
        context.set_stroke_style(&JsValue::from_str("black"));
        context.set_line_width(2.0);
        context.stroke_rect(self.x, self.y, self.width, self.height);

        // Draw button text
        context.set_font("12px 'Press Start 2P'");
        context.set_fill_style(&JsValue::from_str("black"));
        context.set_text_align("center");
        context.set_text_baseline("middle");
        context
            .fill_text(
                &self.text,
                self.x + self.width / 2.0,
                self.y + self.height / 2.0,
            )
            .unwrap();
    }
}
