//! Library containing high-performance functions for the Gaussian Splats rendering process within CesiumJS.

pub mod models;
pub mod radix;
pub mod texture_gen;

use crate::models::TextureData;
use js_sys::{Float32Array, Uint32Array, Uint8Array};
use wasm_bindgen::prelude::*;

/// Generate a splat texture from the given attributes.
///
/// Wraps the [`texture_gen::generate_texture_from_attrs`] function for access from JavaScript.
#[wasm_bindgen]
pub fn generate_splat_texture(
    positions: &Float32Array,
    scales: &Float32Array,
    rotations: &Float32Array,
    colors: &Uint8Array,
    count: usize,
) -> Result<TextureData, JsValue> {
    texture_gen::generate_texture_from_attrs(positions, scales, rotations, colors, count)
}

/// Sorts the Gaussian Splats by depth using a radix sort.
///
/// Wraps the [`radix::radix_sort_gaussians_indexes`] function for access from JavaScript.
#[wasm_bindgen]
pub fn radix_sort_gaussians_indexes(
    positions_arr: &Float32Array,
    model_view_arr: &Float32Array,
    count: usize,
) -> Result<Uint32Array, JsValue> {
    let positions = positions_arr.to_vec();
    let model_view = model_view_arr.to_vec();

    let indices = radix::radix_sort_gaussians_indexes(&positions, &model_view, count)?;

    let indices_array = Uint32Array::from(&indices[..]);
    Ok(indices_array)
}
