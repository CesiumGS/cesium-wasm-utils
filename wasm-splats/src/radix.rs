use js_sys::{Float32Array, Uint32Array};
use wasm_bindgen::prelude::*;

/// Sorts the Gaussian Splats by depth using a radix sort.
#[wasm_bindgen]
pub fn radix_sort_gaussians_indexes(
    positions: &Float32Array,
    model_view: &Float32Array,
    _texture_width: u32, // TODO: FIGURE OUT IF THIS IS NEEDED.
    count: usize,
) -> Result<Uint32Array, JsValue> {
    if positions.length() as usize != count * 3 {
        return Err(JsValue::from_str("Invalid positions length"));
    }
    if model_view.length() != 16 {
        return Err(JsValue::from_str("Invalid model_view length"));
    }

    let positions_vec = positions.to_vec();
    let model_view_vec = model_view.to_vec();
    let mut depth_values = vec![0i32; count];
    let mut max_depth = f32::NEG_INFINITY;
    let mut min_depth = f32::INFINITY;

    for i in 0..count {
        let depth = positions_vec[i * 3] * model_view_vec[2]
            + positions_vec[i * 3 + 1] * model_view_vec[6]
            + positions_vec[i * 3 + 2] * model_view_vec[10];

        let depth_int = (depth * 4096.0) as i32;
        depth_values[i] = depth_int;
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

        for &depth in depth_values.iter() {
            let byte = ((depth >> shift) & 0xFF) as usize;
            counts[byte] += 1;
        }

        let mut total = 0;
        for count in counts.iter_mut() {
            let current = *count;
            *count = total;
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

    let indices_array = Uint32Array::new_with_length(count as u32);
    indices_array.copy_from(&indices);

    Ok(indices_array)
}
