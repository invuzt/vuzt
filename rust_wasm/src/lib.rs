use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[wasm_bindgen]
pub fn handle_slider(value: f64) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let s = value / 100.0;
    
    // Update Kura-kura
    if let Some(el) = document.get_element_by_id("turt") {
        if let Ok(html_el) = el.dyn_into::<HtmlElement>() {
            let _ = html_el.style().set_property("transform", &format!("scale({})", 1.2 - s));
        }
    }
    
    // Update Kelinci
    if let Some(el) = document.get_element_by_id("rabb") {
        if let Ok(html_el) = el.dyn_into::<HtmlElement>() {
            let _ = html_el.style().set_property("transform", &format!("scale({})", 0.8 + s));
        }
    }
}

#[wasm_bindgen]
pub fn get_accordion_content(id: &str) -> String {
    match id {
        "acc1" => "Data Enkripsi v1: Logika diproses oleh WASM secara instan.".to_string(),
        "acc2" => "Sistem Keamanan v2: Modul Rust WASM aktif di WebView.".to_string(),
        _ => "Konten dari Rust WASM.".to_string(),
    }
}
