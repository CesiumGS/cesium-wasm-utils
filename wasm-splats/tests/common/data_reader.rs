use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateSplatTxtAttrsTestInput {
    pub positions: Vec<f32>,
    pub scales: Vec<f32>,
    pub rotations: Vec<f32>,
    pub colors: Vec<u8>,
    pub count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSplatTxtAttrsTestOutput {
    pub texture_data: Vec<u32>,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RadixSortTestInput {
    pub position: Vec<f32>,
    pub model_view: Vec<f32>,
    pub idx_count: usize,
    pub sort_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RadixSortTestOutput {
    pub sorted_idx: Vec<u32>,
}

pub fn read_string_data<T>(data: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let u: T = serde_json::from_str(data)?;

    Ok(u)
}
