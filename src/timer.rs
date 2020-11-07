use chrono::{DateTime, Duration, Utc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlDivElement, HtmlProgressElement};

pub fn parse_time(ss: &str) -> Result<chrono::Duration, Box<dyn std::error::Error>> {
    if ss.is_empty() {
        return Err("empty string".into());
    }

    let ds = ss
        .split(':')
        .map(|s| {
            if s.is_empty() {
                Ok(0)
            } else {
                s.parse::<i64>()
            }
        })
        .collect::<std::result::Result<std::vec::Vec<_>, _>>()?;

    let mut d = chrono::Duration::zero();

    if let Some(h) = ds.get(0) {
        d = d + chrono::Duration::hours(*h);
    }
    if let Some(m) = ds.get(1) {
        d = d + chrono::Duration::minutes(*m);
    }
    if let Some(s) = ds.get(2) {
        d = d + chrono::Duration::seconds(*s);
    }

    Ok(d)
}

pub struct Instance {
    remains_text: HtmlDivElement,
    progress: HtmlProgressElement,

    start_at: DateTime<Utc>,
    duration: Duration,

    done: bool,
}

impl Instance {
    pub fn new(
        document: &Document,
        target: HtmlDivElement,
        duration: Duration,
    ) -> Result<Instance, JsValue> {
        let s = Utc::now();

        let remains = document.create_element("div")?;
        target.append_child(remains.as_ref())?;

        let progress = document.create_element("progress")?;
        progress.set_class_name("progress");
        target.append_child(progress.as_ref())?;

        Ok(Instance {
            remains_text: remains.dyn_into()?,
            progress: progress.dyn_into()?,

            start_at: s,
            duration: duration,

            done: false,
        })
    }

    pub fn update(&mut self) -> Result<(), JsValue> {
        let now = Utc::now();
        let passed = now - self.start_at;

        let p = if !self.duration.is_zero() {
            (passed.num_milliseconds() as f64) / (self.duration.num_milliseconds() as f64)
        } else {
            1.0 // Done
        };

        self.remains_text.set_inner_text(
            format!("{} ({:.3})", format_duration(&(passed - self.duration)), p).as_ref(),
        );

        if !self.done {
            self.progress.set_value(p);
            if 1.0 <= p {
                self.progress.class_list().add_1("is-danger")?;
                self.done = true;
            }
        }

        Ok(())
    }
}

fn format_duration(d: &Duration) -> String {
    let dms = d.num_milliseconds().abs();
    let ds = dms / 1000;
    let sign = if d.num_milliseconds() < 0 { "-" } else { "+" };

    let ms = (dms / 100) % 10;
    let s = ds % 60;
    let m = (ds / 60) % 60;
    let h = ds / 60 / 60;

    format!("T{}{:02}:{:02}:{:02}.{:01}", sign, h, m, s, ms)
}
