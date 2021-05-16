use chrono::{DateTime, Duration, Utc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlDivElement, HtmlElement};

const PROGRESS_STANDARD_COLOR: &str = "bg-gray-700";
const PROGRESS_COMPLETED_COLOR: &str = "bg-orange-600";

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
    progress: HtmlDivElement,

    start_at: DateTime<Utc>,
    duration: Duration,

    done: bool,
}

impl Instance {
    pub fn new(
        document: &Document,
        target: &HtmlDivElement,
        duration: Duration,
    ) -> Result<Instance, JsValue> {
        let s = Utc::now();

        add_classes(&target, &["flex", "bg-gray-50", "shadow", "my-2"])?;

        let remains = document
            .create_element("div")?
            .dyn_into::<HtmlDivElement>()?;
        add_classes(&remains, &["flex", "p-1", "self-center"])?;
        target.append_child(remains.as_ref())?;

        let progress_container = document
            .create_element("div")?
            .dyn_into::<HtmlDivElement>()?;
        add_classes(
            &progress_container,
            &[
                "overflow-hidden",
                "m-2",
                "flex",
                "flex-grow",
                "self-center",
                "bg-gray-300",
                "text-xs",
                "text-white",
            ],
        )?;
        target.append_child(progress_container.as_ref())?;

        let progress = document
            .create_element("div")?
            .dyn_into::<HtmlDivElement>()?;
        add_classes(
            &progress,
            &[
                PROGRESS_STANDARD_COLOR,
                "flex",
                "flex-col",
                "px-1",
                "text-right",
                "font-sans",
            ],
        )?;
        progress_container.append_child(progress.as_ref())?;

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

        self.remains_text
            .set_inner_text(format!("{}", format_duration(&(passed - self.duration))).as_ref());

        self.progress.set_inner_text(format!("{:.3}", p).as_ref());

        if !self.done {
            let percent = p * 100.0;
            self.progress
                .style()
                .set_property("width", format!("{}%", percent).as_ref())?;

            if 1.0 <= p {
                let cl = self.progress.class_list();
                cl.remove_1(PROGRESS_STANDARD_COLOR)?;
                cl.add_1(PROGRESS_COMPLETED_COLOR)?;
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

fn add_classes<T>(element: T, classes: &[&str]) -> Result<(), JsValue>
where
    T: AsRef<HtmlElement>,
{
    let cl = element.as_ref().class_list();
    for c in classes {
        cl.add_1(c)?;
    }
    Ok(())
}
