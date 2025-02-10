use crate::common::data_reader::*;
use js_sys::{Float32Array, Uint8Array};
use wasm_bindgen::JsValue;

pub struct GenerateSplatTextureTestData {
    in_positions: Float32Array,
    in_scales: Float32Array,
    in_rotations: Float32Array,
    in_colors: Uint8Array,
    in_count: usize,
    out_texture_data: Vec<u32>,
    out_width: u32,
    out_height: u32,
}

impl GenerateSplatTextureTestData {
    pub fn new() -> Result<Self, JsValue> {
        let input_data: GenerateSplatTxtAttrsTestInput = read_string_data(include_str!(
            "./data/generate-splat-tex-attrs-input-data.json"
        ))
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let output_data: GenerateSplatTxtAttrsTestOutput = read_string_data(include_str!(
            "./data/generate-splat-tex-attrs-output-data.json"
        ))
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let positions = Float32Array::new_with_length(input_data.positions.len() as u32);
        positions.copy_from(&input_data.positions);

        let scales = Float32Array::new_with_length(input_data.scales.len() as u32);
        scales.copy_from(&input_data.scales);

        let rotations = Float32Array::new_with_length(input_data.rotations.len() as u32);
        rotations.copy_from(&input_data.rotations);

        let colors = Uint8Array::new_with_length(input_data.colors.len() as u32);
        colors.copy_from(&input_data.colors);

        Ok(Self {
            in_positions: positions,
            in_scales: scales,
            in_rotations: rotations,
            in_colors: colors,
            in_count: input_data.count,
            out_texture_data: output_data.texture_data,
            out_width: output_data.width,
            out_height: output_data.height,
        })
    }

    pub fn get_positions(&self) -> Float32Array {
        self.in_positions.clone()
    }

    pub fn get_scales(&self) -> Float32Array {
        self.in_scales.clone()
    }

    pub fn get_rotations(&self) -> Float32Array {
        self.in_rotations.clone()
    }

    pub fn get_colors(&self) -> Uint8Array {
        self.in_colors.clone()
    }

    pub fn get_count(&self) -> usize {
        self.in_count
    }

    pub fn get_texture_data(&self) -> Vec<u32> {
        self.out_texture_data.clone()
    }

    pub fn get_width(&self) -> u32 {
        self.out_width
    }

    pub fn get_height(&self) -> u32 {
        self.out_height
    }
}

pub struct SortGaussianIndexesTestData {
    in_positions: Float32Array,
    in_model_view: Float32Array,
    in_texture_width: u32,
    in_count: usize,
    out_sorted_idx: Vec<u32>,
}

impl SortGaussianIndexesTestData {
    pub fn new() -> Result<Self, JsValue> {
        let input_data: RadixSortTestInput =
            read_string_data(include_str!("./data/radix-sort-input-data.json"))
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let output_data: RadixSortTestOutput =
            read_string_data(include_str!("./data/radix-sort-output-data.json"))
                .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let positions = Float32Array::new_with_length(input_data.position.len() as u32);
        positions.copy_from(&input_data.position);

        let model_view = Float32Array::new_with_length(input_data.model_view.len() as u32);
        model_view.copy_from(&input_data.model_view);

        let sorted_idx = output_data.sorted_idx;

        Ok(Self {
            in_positions: positions,
            in_model_view: model_view,
            in_texture_width: 0,
            in_count: input_data.idx_count,
            out_sorted_idx: sorted_idx,
        })
    }

    pub fn get_positions(&self) -> Float32Array {
        self.in_positions.clone()
    }

    pub fn get_model_view(&self) -> Float32Array {
        self.in_model_view.clone()
    }

    pub fn get_texture_width(&self) -> u32 {
        self.in_texture_width
    }

    pub fn get_count(&self) -> usize {
        self.in_count
    }

    pub fn get_sorted_idx(&self) -> Vec<u32> {
        self.out_sorted_idx.clone()
    }
}
