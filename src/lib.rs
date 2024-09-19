mod button;
mod text;
mod text_input;
mod ts;
mod ui;

use button::Button;
use std::panic;
use text::{Text, TextSize};
use text_input::TextInput;
use ts::{init_performance, now};
use ui::UIElement;
use wasm_bindgen::prelude::*;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, Performance};

#[wasm_bindgen]
pub struct Game {
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    mouse_x: f64,
    mouse_y: f64,
    is_start_screen: bool,
    text_input: TextInput,
    last_update: f64,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Game, JsValue> {
        console_error_panic_hook::set_once();
        init_performance()?;
        console::log_1(&"Initializing game...".into());

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("game-canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let width = canvas.width();
        let height = canvas.height();

        let text_input = TextInput::new(
            (width as f64 / 2.0) - 100.0,
            (height as f64 / 2.0) + 90.0,
            200.0,
            30.0,
        );

        Ok(Game {
            context,
            width,
            height,
            mouse_x: 0.0,
            mouse_y: 0.0,
            is_start_screen: true,
            text_input,
            last_update: now(),
        })
    }

    pub fn update(&mut self) {
        self.text_input.update();
        //let now = Instant::now();
        //let dt = now.duration_since(self.last_update);
        //self.last_update = now;

        // Update game logic here
        // self.text_input.update();
    }

    pub fn render(&self) {
        self.context
            .clear_rect(0.0, 0.0, self.width as f64, self.height as f64);

        //if self.is_start_screen {
        self.render_start_screen();
        //} else {
        // Game rendering will go here
        //}
    }

    fn render_start_screen(&self) {
        self.context.set_fill_style(&JsValue::from_str("white"));
        self.context
            .fill_rect(0.0, 0.0, self.width as f64, self.height as f64);

        let title = Text::new("Start a new game".to_string(), TextSize::ExtraLarge);
        title.render(
            &self.context,
            self.width as f64 / 2.0,
            self.height as f64 / 2.0 - 24.0,
        );

        let button_x = (self.width as f64 / 2.0) - 60.0;
        let button_y = (self.height as f64 / 2.0) + 10.0;
        let start_button = Button::new(button_x, button_y, 120.0, 50.0, "Let's go".to_string());
        start_button.render(&self.context, self.mouse_x, self.mouse_y);

        self.text_input
            .render(&self.context, self.mouse_x, self.mouse_y);

        let input_label = Text::new("Enter your name:".to_string(), TextSize::Normal);
        input_label.render(
            &self.context,
            self.width as f64 / 2.0,
            self.height as f64 / 2.0 + 80.0,
        );
    }

    pub fn handle_mouse_move(&mut self, x: f64, y: f64) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn handle_keydown(&mut self, key: String) {
        if key == "Enter" {
            self.is_start_screen = false;
        }
        console::log_1(&format!("Key pressed: {}", key).into());
        self.text_input.handle_key_press(&key);
    }

    pub fn handle_click(&mut self, x: f64, y: f64) {
        if self.is_start_screen {
            let button_x = (self.width as f64 / 2.0) - 60.0;
            let button_y = (self.height as f64 / 2.0) + 10.0;
            let button_width = 120.0;
            let button_height = 50.0;

            if x >= button_x
                && x <= button_x + button_width
                && y >= button_y
                && y <= button_y + button_height
            {
                self.is_start_screen = false;
            }

            self.text_input.handle_click(x, y);
        }
    }

    pub fn handle_key_press(&mut self, key: &str) {
        self.text_input.handle_key_press(key);
    }
}
