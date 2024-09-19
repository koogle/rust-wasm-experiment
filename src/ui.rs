use web_sys::CanvasRenderingContext2d;

pub trait UIElement {
    fn render(&self, context: &CanvasRenderingContext2d, mouse_x: f64, mouse_y: f64);
}
