use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement};

#[wasm_bindgen]
pub fn handle_ui_action(tag: &str, value: &str) -> String {
    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");

    match tag {
        "slider" => {
            let val_f64 = value.parse::<f64>().unwrap_or(0.0);
            let s = val_f64 / 100.0;
            
            // Rust memproses scaling visual
            if let Some(el) = document.get_element_by_id("turt") {
                if let Ok(html_el) = el.dyn_into::<HtmlElement>() {
                    let _ = html_el.style().set_property("transform", &format!("scale({})", 1.2 - s));
                }
            }
            if let Some(el) = document.get_element_by_id("rabb") {
                if let Ok(html_el) = el.dyn_into::<HtmlElement>() {
                    let _ = html_el.style().set_property("transform", &format!("scale({})", 0.8 + s));
                }
            }
            format!("WASM: Speed set to {}%", value)
        },
        "accordion" => {
            match value {
                "acc1" => "Konten Primary: Berhasil dimuat dari sistem Rust WASM.".to_string(),
                "acc2" => "Konten Secondary: Enkripsi data aktif di level modul.".to_string(),
                _ => "Detail terdeteksi.".to_string(),
            }
        },
        "button" => {
            "Rust WASM: Aksi Large Button Diterima!".to_string()
        },
        _ => format!("WASM: Tag {} diproses", tag),
    }
}
