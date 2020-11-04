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
        })
    }

    pub fn mainloop(&mut self) -> Result<(), JsValue> {
        self.target
            .set_inner_html(format!("No. {}", self.counter).as_ref());

        self.counter += 1;
        Ok(())
    }
}
