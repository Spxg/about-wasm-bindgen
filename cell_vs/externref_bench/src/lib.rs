use js_sys::Uint8Array;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn bench() -> u8 {
    let array = Uint8Array::new_with_length(1);
    array.get_index(0)
}
