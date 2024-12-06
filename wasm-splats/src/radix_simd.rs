use std::arch::wasm32::*;
use wasm_bindgen::prelude::*;
use js_sys::{Float32Array, Uint8Array, Uint32Array, WebAssembly};
use wasm_bindgen::JsCast;
use web_sys::console;

use crate::perf_timer;

#[wasm_bindgen]
pub struct GSplatData {
    positions: Vec<f32>,
    scales: Vec<f32>,
    rotations: Vec<f32>,
    colors: Vec<u8>,
    model_view: [f32; 16],
    count: usize,
}

#[wasm_bindgen]
impl GSplatData {
    #[wasm_bindgen(constructor)]
    pub fn new(
        positions: Vec<f32>,
        scales: Vec<f32>,
        rotations: Vec<f32>,
        colors: Vec<u8>,
        model_view: Vec<f32>,
        count: usize,
    ) -> Self {
        let mut model_view_array = [0.0; 16];
        model_view_array.copy_from_slice(&model_view);
        
        Self {
            positions,
            scales,
            rotations,
            colors,
            model_view: model_view_array,
            count,
        }
    }

    #[wasm_bindgen(js_name = fromFloat32Arrays)]
    pub fn from_float32_arrays(
        positions: Float32Array,
        scales: Float32Array,
        rotations: Float32Array,
        colors: Uint8Array,
        model_view: Float32Array,
        count: usize,
    ) -> Result<GSplatData, JsValue> {
        if positions.length() as usize != count * 3 {
            return Err(JsValue::from_str("Invalid positions length"));
        }
        if scales.length() as usize != count * 3 {
            return Err(JsValue::from_str("Invalid scales length"));
        }
        if rotations.length() as usize != count * 4 {
            return Err(JsValue::from_str("Invalid rotations length"));
        }
        if colors.length() as usize != count * 4 {
            return Err(JsValue::from_str("Invalid colors length"));
        }
        if model_view.length() != 16 {
            return Err(JsValue::from_str("Model view matrix must have 16 elements"));
        }

        let positions: Vec<f32> = positions.to_vec();
        let scales: Vec<f32> = scales.to_vec();
        let rotations: Vec<f32> = rotations.to_vec();
        let colors: Vec<u8> = colors.to_vec();
        let model_view: Vec<f32> = model_view.to_vec();

        Ok(GSplatData::new(
            positions,
            scales,
            rotations,
            colors,
            model_view,
            count,
        ))
    }

    #[wasm_bindgen(js_name = getPositions)]
    pub fn get_positions(&self) -> Float32Array {
        let result = Float32Array::new_with_length(self.positions.len() as u32);
        result.copy_from(&self.positions[..]);
        result
    }

    #[wasm_bindgen(js_name = getScales)]
    pub fn get_scales(&self) -> Float32Array {
        let result = Float32Array::new_with_length(self.scales.len() as u32);
        result.copy_from(&self.scales[..]);
        result
    }

    #[wasm_bindgen(js_name = getRotations)]
    pub fn get_rotations(&self) -> Float32Array {
        let result = Float32Array::new_with_length(self.rotations.len() as u32);
        result.copy_from(&self.rotations[..]);
        result
    }

    #[wasm_bindgen(js_name = getColors)]
    pub fn get_colors(&self) -> Uint8Array {
        let result = Uint8Array::new_with_length(self.colors.len() as u32);
        result.copy_from(&self.colors[..]);
        result
    }
}

#[target_feature(enable = "simd128")]
unsafe fn compute_depths_simd(positions: &[f32], model_view: &[f32], count: usize) -> Vec<i32> {
    let mut depths = Vec::with_capacity(count);
    let simd_count = count - (count % 4);
    
    let scale = f32x4(4096.0, 4096.0, 4096.0, 4096.0);
    let mv2 = f32x4_splat(model_view[2]);
    let mv6 = f32x4_splat(model_view[6]);
    let mv10 = f32x4_splat(model_view[10]);
    
    for chunk_idx in (0..simd_count).step_by(4) {
        let base_idx = chunk_idx * 3;
        if base_idx + 11 >= positions.len() {
            break; 
        }
        
        let pos = v128_load(positions[base_idx..].as_ptr() as *const v128);
        let mut depth = f32x4_mul(pos, mv2);
        
        let pos_y = v128_load(positions[base_idx + 4..].as_ptr() as *const v128);
        depth = f32x4_add(depth, f32x4_mul(pos_y, mv6));
        
        let pos_z = v128_load(positions[base_idx + 8..].as_ptr() as *const v128);
        depth = f32x4_add(depth, f32x4_mul(pos_z, mv10));
        
        let depth_scaled = f32x4_mul(depth, scale);
        let depth_int = i32x4_trunc_sat_f32x4(depth_scaled);
        
        let mut result = [0i32; 4];
        v128_store(result.as_mut_ptr() as *mut v128, depth_int);
        depths.extend_from_slice(&result);
    }
    
    let remainder_start = (count / 4) * 4;
    for i in remainder_start..count {
        let idx = i * 3;
        if idx + 2 < positions.len() {
            let depth = positions[idx] * model_view[2] +
                       positions[idx + 1] * model_view[6] +
                       positions[idx + 2] * model_view[10];
            depths.push((depth * 4096.0) as i32);
        }
    }
    
    depths.truncate(count);
    depths
}

#[target_feature(enable = "simd128")]
unsafe fn reorder_attributes_simd(data: &mut GSplatData, indices: &[u32]) {
    let mut new_positions = vec![0.0; data.positions.len()];
    let mut new_scales = vec![0.0; data.scales.len()];
    let mut new_rotations = vec![0.0; data.rotations.len()];
    let mut new_colors = vec![0; data.colors.len()];
    
    for (new_idx, &old_idx) in indices.iter().enumerate() {
        let old_idx = old_idx as usize;

        if old_idx * 3 + 2 >= data.positions.len() || 
           new_idx * 3 + 2 >= new_positions.len() {
            break;
        }

        let pos_idx = new_idx * 3;
        let old_pos_idx = old_idx * 3;
        new_positions[pos_idx..pos_idx + 3]
            .copy_from_slice(&data.positions[old_pos_idx..old_pos_idx + 3]);
        
        if old_idx * 3 + 2 >= data.scales.len() || 
           new_idx * 3 + 2 >= new_scales.len() {
            break;
        }
        
        let scale_idx = new_idx * 3;
        let old_scale_idx = old_idx * 3;
        new_scales[scale_idx..scale_idx + 3]
            .copy_from_slice(&data.scales[old_scale_idx..old_scale_idx + 3]);
        
        if old_idx * 4 + 3 >= data.rotations.len() || 
           new_idx * 4 + 3 >= new_rotations.len() {
            break;
        }
        
        let rot_idx = new_idx * 4;
        let old_rot_idx = old_idx * 4;
        new_rotations[rot_idx..rot_idx + 4]
            .copy_from_slice(&data.rotations[old_rot_idx..old_rot_idx + 4]);
        
        if old_idx * 4 + 3 >= data.colors.len() || 
           new_idx * 4 + 3 >= new_colors.len() {
            break;
        }

        let color_idx = new_idx * 4;
        let old_color_idx = old_idx * 4;
        new_colors[color_idx..color_idx + 4]
            .copy_from_slice(&data.colors[old_color_idx..old_color_idx + 4]);
    }
    
    data.positions = new_positions;
    data.scales = new_scales;
    data.rotations = new_rotations;
    data.colors = new_colors;
}

#[wasm_bindgen]
pub fn radix_sort_simd(data: &mut GSplatData) -> Result<(), JsValue> {
    let count = data.count;
    
    if count * 3 > data.positions.len() ||
       count * 3 > data.scales.len() ||
       count * 4 > data.rotations.len() ||
       count * 4 > data.colors.len() {
        return Err(JsValue::from_str("Invalid input sizes"));
    }
    
    let mut depths = unsafe { 
        compute_depths_simd(&data.positions, &data.model_view, count) 
    };
    let mut indices: Vec<u32> = (0..count as u32).collect();
    
    let mut temp_depths = vec![0i32; count];
    let mut temp_indices = vec![0u32; count];
    
    for shift in (0..32).step_by(8) {
        let mut counts = [0u32; 256];
        
        unsafe { count_frequencies_simd(&depths, shift, &mut counts) };

        let mut total = 0u32;
        for count in counts.iter_mut() {
            let current = *count;
            *count = total;
            total += current;
        }

        unsafe { 
            scatter_elements_simd(
                &depths, 
                &indices, 
                shift, 
                &counts, 
                &mut temp_depths, 
                &mut temp_indices
            ) 
        };
        std::mem::swap(&mut depths, &mut temp_depths);
        std::mem::swap(&mut indices, &mut temp_indices);
    }
    
    unsafe { reorder_attributes_simd(data, &indices) };
    Ok(())
}

#[target_feature(enable = "simd128")]
unsafe fn count_frequencies_simd(depths: &[i32], shift: u32, counts: &mut [u32; 256]) {
    unsafe {
        let mask = i32x4_splat(0xFF);
        
        for chunk in depths.chunks_exact(4) {
            let values = v128_load(chunk.as_ptr() as *const v128);
            let shifted = i32x4_shr(values, shift);
            let bytes = v128_and(shifted as v128, mask);
            
            let mut result = [0i32; 4];
            v128_store(result.as_mut_ptr() as *mut v128, bytes);
            
            for &value in &result {
                counts[value as usize] += 1;
            }
        }
    }
    
    for &depth in depths.chunks_exact(4).remainder() {
        let byte = ((depth >> shift) & 0xFF) as usize;
        counts[byte] += 1;
    }
}

#[target_feature(enable = "simd128")]
unsafe fn scatter_elements_simd(
    depths: &[i32],
    indices: &[u32],
    shift: u32,
    counts: &[u32; 256],
    temp_depths: &mut [i32],
    temp_indices: &mut [u32],
) {
    let mut offsets = counts.to_owned();
    
    for (&depth, &index) in depths.iter().zip(indices.iter()) {
        let byte = ((depth >> shift) & 0xFF) as usize;
        let pos = offsets[byte] as usize;
        
        temp_depths[pos] = depth;
        temp_indices[pos] = index;
        
        offsets[byte] += 1;
    }
}