mod radix;
mod radix_simd;
mod texture_gen;

use js_sys::{Float32Array, Object, Uint32Array, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

//Wrapper func. Most are called directly
#[wasm_bindgen]
pub fn generate_splat_texture_from_attrs(
    positions: &Float32Array,
    scales: &Float32Array,
    rotations: &Float32Array,
    colors: &Uint8Array,
    count: usize,
) -> Result<Object, JsValue> {
    let texture_data =
        texture_gen::generate_texture_from_attrs(positions, scales, rotations, colors, count)?;

    let js_data =
        Uint32Array::new_with_length(texture_data.width() * texture_data.height() * 4);
    js_data.copy_from(&texture_data.data());

    let result = Object::new();
    js_sys::Reflect::set(&result, &"data".into(), &js_data)?;
    js_sys::Reflect::set(
        &result,
        &"width".into(),
        &(texture_data.width() as f64).into(),
    )?;
    js_sys::Reflect::set(
        &result,
        &"height".into(),
        &(texture_data.height() as f64).into(),
    )?;

    Ok(result)
}
