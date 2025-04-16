//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]
mod common;

extern crate wasm_bindgen_test;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::common::test_utils::check_uint32array;
// use crate::common::wasm_utils::{translate_time, console_log};
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

    let result = radix_sort_gaussians_indexes(&positions, &model_view, count).unwrap();

    check_uint32array(&result, sorted_idx.as_ref()).unwrap();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => {log(&format_args!($($t)*).to_string())};
}

pub fn translate_time(perf_time: f64) -> SystemTime {
    let seconds = (perf_time as u64) / 1000;
    let nanoseconds = (((perf_time as u64) % 1000) as u32) * 1000000;
    UNIX_EPOCH + Duration::new(seconds, nanoseconds)
}

#[wasm_bindgen_test]
fn test_radix_sort_performance() {
    let window = web_sys::window().expect("Window expected in this context.");
    let perf = window.performance().expect("Performance object unavailable.");

    fn test_case() {
        let test_data = test_data::SortGaussianIndexesTestData::new().unwrap();
        let positions = test_data.get_positions();
        let model_view = test_data.get_model_view();
        let count = test_data.get_count();

        let _result = radix_sort_gaussians_indexes(&positions, &model_view, count).unwrap();
    }

    let start = translate_time(perf.now());
    for _ in 0..100 {
        test_case();
    }
    let end = translate_time(perf.now());
    let elapsed = end.duration_since(start).expect("It's a time machine, Marty!");
    console_log!("Elapsed Perf Testing Time: {:?}", elapsed);
}
