//! Module encapsulating the data models exposed through WebAssembly.

use wasm_bindgen::prelude::wasm_bindgen;

/// A structure representing texture data. This is used to pass the texture data from generation in [`texture_gen`] to the JavaScript side.
#[wasm_bindgen]
pub struct TextureData {
    /// The texture data.
    data: Vec<u32>,
    /// Width of the texture in pixels.
    width: u32,
    /// Height of the texture in pixels.
    height: u32,
}

#[wasm_bindgen]
impl TextureData {
    /// Getter for the underlying texture data. Always returns a copy.
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<u32> {
        self.data.clone()
    }

    /// Getter for the width of the texture in pixels.
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Getter for the height of the texture in pixels.
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Creates a new texture data object with the underlying data, width, and height.
    pub fn new(data: Vec<u32>, width: u32, height: u32) -> Self {
        TextureData {
            data,
            width,
            height,
        }
    }
}
