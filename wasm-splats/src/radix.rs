//! Radix sort implementation for sorting Gaussian Splats.

use wasm_bindgen::prelude::*;

/// Sorts the Gaussian Splats by depth using a radix sort. Uses SIMD through autovectorization
/// on WASM targets.
#[cfg_attr(target_family = "wasm", target_feature(enable = "simd128"))]
pub fn radix_sort_gaussians_indexes(
    positions: &[f32],
    model_view: &[f32],
    count: usize,
) -> Result<Vec<u32>, JsValue> {
    if positions.len() != count * 3 {
        return Err(JsValue::from_str("Invalid positions length"));
    }
    if model_view.len() != 16 {
        return Err(JsValue::from_str("Invalid model_view length"));
    }

    let mut depth_values = vec![0i32; count];
    let mut max_depth = f32::NEG_INFINITY;
    let mut min_depth = f32::INFINITY;

    let mv2 = model_view[2];
    let mv6 = model_view[6];
    let mv10 = model_view[10];

    for (i, depth_value) in depth_values.iter_mut().enumerate() {
        let depth =
            positions[i * 3] * mv2 + positions[i * 3 + 1] * mv6 + positions[i * 3 + 2] * mv10;

        let depth_int = (depth * 4096.0) as i32;
        *depth_value = depth_int;
        max_depth = max_depth.max(depth_int as f32);
        min_depth = min_depth.min(depth_int as f32);
    }

    let depth_offset = (-min_depth) as i32;
    for depth in depth_values.iter_mut() {
        *depth += depth_offset;
    }

    let mut indices: Vec<u32> = (0..count as u32).collect();
    let mut temp_depths = vec![0i32; count];
    let mut temp_indices = vec![0u32; count];

    for shift in (0..32).step_by(8) {
        let mut counts = [0u32; 256];

        for &depth in &depth_values {
            let byte = ((depth >> shift) & 0xFF) as usize;
            counts[byte] += 1;
        }

        let mut total = 0;
        for i in 0..counts.len() {
            let current = counts[i];
            counts[i] = total;
            total += current;
        }

        for i in 0..count {
            let byte = ((depth_values[i] >> shift) & 0xFF) as usize;
            let pos = counts[byte] as usize;
            counts[byte] += 1;

            temp_depths[pos] = depth_values[i];
            temp_indices[pos] = indices[i];
        }

        depth_values.copy_from_slice(&temp_depths);
        indices.copy_from_slice(&temp_indices);
    }

    Ok(indices)
}
