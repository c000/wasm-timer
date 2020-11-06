use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct RState {
    document: web_sys::Document,

    target: web_sys::HtmlElement,

    counter: i32,

    buffer: String,
}

#[wasm_bindgen]
impl RState {
    #[wasm_bindgen(constructor)]
    pub fn new(target: web_sys::HtmlElement) -> Result<RState, JsValue> {
        let document = web_sys::window()
            .expect("no global `window` exists")
            .document()
            .expect("should have a document on window");

        Ok(RState {
            document: document,
            target: target,
            counter: 0,
            buffer: String::new(),
        })
    }

    pub fn mainloop(&mut self) -> Result<(), JsValue> {
        let dt = chrono::Local::now();

        self.target.set_inner_text(
            format!(
                "T: {}\nN: {}\nS: {}",
                dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, false),
                self.counter,
                self.buffer,
            )
            .as_ref(),
        );

        self.counter += 1;
        Ok(())
    }

    pub fn exec(&mut self, s: &str) -> Result<(), JsValue> {
        self.buffer.push_str(s);
        Ok(())
    }
}
