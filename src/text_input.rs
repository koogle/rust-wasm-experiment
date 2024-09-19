use crate::ts::{init_performance, now};
use crate::ui::UIElement;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct TextInput {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    value: String,
    is_focused: bool,
    cursor_visible: bool,
    last_blink: f64,
    blink_interval: f64,
}

impl TextInput {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        TextInput {
            x,
            y,
            width,
            height,
            value: String::new(),
            is_focused: false,
            cursor_visible: true,
            last_blink: 0.0,
            blink_interval: 500.0, // Blink every 500ms
        }
    }

    pub fn handle_click(&mut self, mouse_x: f64, mouse_y: f64) {
        self.is_focused = mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height;
    }

    pub fn handle_key_press(&mut self, key: &str) {
        if self.is_focused {
            if key == "Backspace" {
                self.value.pop();
            } else if key.len() == 1 {
                self.value.push_str(key);
            }
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn update(&mut self) {
        if self.is_focused && now() - self.last_blink >= self.blink_interval {
            self.cursor_visible = !self.cursor_visible;
            self.last_blink = now();
        }
    }
}

impl UIElement for TextInput {
    fn render(&self, context: &CanvasRenderingContext2d, _mouse_x: f64, _mouse_y: f64) {
        // Draw the input box
        context.set_stroke_style(&JsValue::from_str("black"));
        context.set_line_width(2.0);
        context.stroke_rect(self.x, self.y, self.width, self.height);

        // Draw the text
        context.set_font("16px 'Press Start 2P'");
        context.set_fill_style(&JsValue::from_str("black"));
        context.set_text_baseline("middle");
        context
            .fill_text(&self.value, self.x + 5.0, self.y + self.height / 2.0)
            .unwrap();

        // Draw the blinking cursor if focused
        if self.is_focused && self.cursor_visible {
            let estimated_char_width = 10.0;
            let text_width = self.value.len() as f64 * estimated_char_width;
            let cursor_x = self.x + 5.0 + text_width;
            context.set_stroke_style(&JsValue::from_str("black"));
            context.begin_path();
            context.move_to(cursor_x, self.y + 5.0);
            context.line_to(cursor_x, self.y + self.height - 5.0);
            context.stroke();
        }
    }
}
