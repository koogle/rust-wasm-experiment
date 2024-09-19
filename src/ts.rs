use wasm_bindgen::JsValue;
use web_sys::Performance;

// Global performance object
thread_local! {
    static PERFORMANCE: Performance = web_sys::window()
        .expect("no global `window` exists")
        .performance()
        .expect("should have `performance` on `window`");
}

/// Returns the current timestamp in milliseconds.
pub fn now() -> f64 {
    PERFORMANCE.with(|p| p.now())
}

/// Initializes the performance object. Call this once at the start of your application.
pub fn init_performance() -> Result<(), JsValue> {
    PERFORMANCE.with(|_| Ok(()))
}
