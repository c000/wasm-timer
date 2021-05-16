use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod timer;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct RState {
    document: web_sys::Document,

    target: web_sys::HtmlDivElement,

    timers: Vec<timer::Instance>,
}

#[wasm_bindgen]
impl RState {
    #[wasm_bindgen(constructor)]
    pub fn new(target: web_sys::HtmlDivElement) -> Result<RState, JsValue> {
        let document = web_sys::window()
            .expect("no global `window` exists")
            .document()
            .expect("should have a document on window");

        Ok(RState {
            document: document,
            target: target,
            timers: Vec::new(),
        })
    }

    pub fn mainloop(&mut self) -> Result<(), JsValue> {
        for t in self.timers.iter_mut() {
            t.update()?;
        }
        Ok(())
    }

    pub fn exec(&mut self, s: &str) -> Result<bool, JsValue> {
        let res = timer::parse_time(s);

        match res {
            Err(e) => {
                log(format!("{}", e).as_ref());
                Ok(false)
            }

            Ok(duration) => {
                let div = self
                    .document
                    .create_element("div")?
                    .dyn_into::<web_sys::HtmlDivElement>()?;
                self.target.append_child(div.as_ref())?;
                let i = timer::Instance::new(&self.document, div, duration)?;
                self.timers.push(i);

                Ok(true)
            }
        }
    }
}
