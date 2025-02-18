//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]
mod common;

extern crate wasm_bindgen_test;
use crate::common::test_utils::check_uint32array;
use common::test_data;
use wasm_bindgen_test::*;
use wasm_splats::radix::radix_sort_gaussians_indexes;
use wasm_splats::texture_gen::generate_texture_from_attrs;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_generate_splat_texture_from_attrs() {
    let test_data = test_data::GenerateSplatTextureTestData::new().unwrap();
    let positions = test_data.get_positions();
    let scales = test_data.get_scales();
    let rotations = test_data.get_rotations();
    let colors = test_data.get_colors();
    let count = test_data.get_count();
    let texture_data = test_data.get_texture_data();
    let width = test_data.get_width();
    let height = test_data.get_height();

    let result =
        generate_texture_from_attrs(&positions, &scales, &rotations, &colors, count).unwrap();

    assert_eq!(result.data(), texture_data);
    assert_eq!(result.width(), width);
    assert_eq!(result.height(), height);
}

#[wasm_bindgen_test]
fn test_radix_sort_gaussians_indexes() {
    let test_data = test_data::SortGaussianIndexesTestData::new().unwrap();
    let positions = test_data.get_positions();
    let model_view = test_data.get_model_view();
    let count = test_data.get_count();
    let sorted_idx = test_data.get_sorted_idx();

    let result =
        radix_sort_gaussians_indexes(&positions, &model_view, count).unwrap();

    check_uint32array(&result, sorted_idx.as_ref()).unwrap();
}
