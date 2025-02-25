use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn s2t(s: &str) -> String {
    hanconv::s2t(s)
}

#[wasm_bindgen]
pub fn t2s(s: &str) -> String {
    hanconv::t2s(s)
}

#[wasm_bindgen]
pub fn s2tw(s: &str) -> String {
    hanconv::s2tw(s)
}

#[wasm_bindgen]
pub fn tw2s(s: &str) -> String {
    hanconv::tw2s(s)
}

#[wasm_bindgen]
pub fn s2twp(s: &str) -> String {
    hanconv::s2twp(s)
}

#[wasm_bindgen]
pub fn tw2sp(s: &str) -> String {
    hanconv::tw2sp(s)
}

#[wasm_bindgen]
pub fn t2tw(s: &str) -> String {
    hanconv::t2tw(s)
}

#[wasm_bindgen]
pub fn tw2t(s: &str) -> String {
    hanconv::tw2t(s)
}

#[wasm_bindgen]
pub fn s2hk(s: &str) -> String {
    hanconv::s2hk(s)
}

#[wasm_bindgen]
pub fn hk2s(s: &str) -> String {
    hanconv::hk2s(s)
}

#[wasm_bindgen]
pub fn t2hk(s: &str) -> String {
    hanconv::t2hk(s)
}

#[wasm_bindgen]
pub fn hk2t(s: &str) -> String {
    hanconv::hk2t(s)
}

#[wasm_bindgen]
pub fn t2jp(s: &str) -> String {
    hanconv::t2jp(s)
}

#[wasm_bindgen]
pub fn jp2t(s: &str) -> String {
    hanconv::jp2t(s)
}
