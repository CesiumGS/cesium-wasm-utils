use wasm_bindgen::prelude::*;
use js_sys::{Float32Array, Uint8Array, Uint32Array, WebAssembly};
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn radix_sort_gaussians_attrs(
    positions: &Float32Array,
    scales: &Float32Array,
    rotations: &Float32Array,
    colors: &Uint8Array,
    model_view: &Float32Array,
    count: usize,
) -> Result<js_sys::Array, JsValue> {
    if positions.length() as usize != count * 3 
        || scales.length() as usize != count * 3 
        || rotations.length() as usize != count * 4 
        || colors.length() as usize != count * 4 
        || model_view.length() != 16 {
        return Err(JsValue::from_str("Invalid array lengths"));
    }

    //set capacity first
    let positions_vec = positions.to_vec();
    let model_view_vec = model_view.to_vec();

    let mut depth_values = vec![0i32; count];
    let mut max_depth = f32::NEG_INFINITY;
    let mut min_depth = f32::INFINITY;

    for i in 0..count {
        let depth = positions_vec[i * 3] * model_view_vec[2] +
                   positions_vec[i * 3 + 1] * model_view_vec[6] +
                   positions_vec[i * 3 + 2] * model_view_vec[10];
        
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

    let mut new_positions: Vec<f32> = vec![0.0; count * 3];
    let mut new_scales: Vec<f32> = vec![0.0; count * 3];
    let mut new_rotations: Vec<f32> = vec![0.0; count * 4];
    let mut new_colors: Vec<u8> = vec![0; count * 4];

    let scales_vec = scales.to_vec();
    let rotations_vec = rotations.to_vec();
    let colors_vec = colors.to_vec();

    for i in 0..count {
        let j = indices[i] as usize;

        new_positions[i * 3] = positions_vec[j * 3];
        new_positions[i * 3 + 1] = positions_vec[j * 3 + 1];
        new_positions[i * 3 + 2] = positions_vec[j * 3 + 2];

        new_scales[i * 3] = scales_vec[j * 3];
        new_scales[i * 3 + 1] = scales_vec[j * 3 + 1];
        new_scales[i * 3 + 2] = scales_vec[j * 3 + 2];

        new_rotations[i * 4] = rotations_vec[j * 4];
        new_rotations[i * 4 + 1] = rotations_vec[j * 4 + 1];
        new_rotations[i * 4 + 2] = rotations_vec[j * 4 + 2];
        new_rotations[i * 4 + 3] = rotations_vec[j * 4 + 3];

        new_colors[i * 4] = colors_vec[j * 4];
        new_colors[i * 4 + 1] = colors_vec[j * 4 + 1];
        new_colors[i * 4 + 2] = colors_vec[j * 4 + 2];
        new_colors[i * 4 + 3] = colors_vec[j * 4 + 3];
    }

    let new_positions_array = Float32Array::new_with_length(count as u32 * 3);
    new_positions_array.copy_from(&new_positions[..]);

    let new_scales_array = Float32Array::new_with_length(count as u32 * 3);
    new_scales_array.copy_from(&new_scales[..]);

    let new_rotations_array = Float32Array::new_with_length(count as u32 * 4);
    new_rotations_array.copy_from(&new_rotations[..]);

    let new_colors_array = Uint8Array::new_with_length(count as u32 * 4);
    new_colors_array.copy_from(&new_colors[..]);

    let result = js_sys::Array::new();
    result.push(&new_positions_array);
    result.push(&new_scales_array);
    result.push(&new_rotations_array);
    result.push(&new_colors_array);

    Ok(result)
}


#[wasm_bindgen]
pub fn radix_sort_gaussians_indexes(
    positions: &Float32Array,
    model_view: &Float32Array,
    texture_width: u32,
    count: usize,
) -> Result<js_sys::Uint32Array, JsValue> {
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
        let depth = positions_vec[i * 3] * model_view_vec[2] +
                   positions_vec[i * 3 + 1] * model_view_vec[6] +
                   positions_vec[i * 3 + 2] * model_view_vec[10];

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

