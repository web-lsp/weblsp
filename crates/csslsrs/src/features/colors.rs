use crate::{text_document, types::RangeWASM};
use lsp_types::ColorInformation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = ColorInformation)]
pub struct ColorInformationWASM(ColorInformation);

#[wasm_bindgen(js_name = Color)]
pub struct ColorWASM(lsp_types::Color);

#[wasm_bindgen(js_class = Color)]
impl ColorWASM {
    #[wasm_bindgen(getter)]
    pub fn red(&self) -> f32 {
        self.0.red
    }

    #[wasm_bindgen(getter)]
    pub fn green(&self) -> f32 {
        self.0.green
    }

    #[wasm_bindgen(getter)]
    pub fn blue(&self) -> f32 {
        self.0.blue
    }

    #[wasm_bindgen(getter)]
    pub fn alpha(&self) -> f32 {
        self.0.alpha
    }
}

impl From<lsp_types::Color> for ColorWASM {
    fn from(color: lsp_types::Color) -> Self {
        ColorWASM(color)
    }
}

#[wasm_bindgen(js_class = ColorInformation)]
impl ColorInformationWASM {
    #[wasm_bindgen(getter)]
    pub fn color(&self) -> ColorWASM {
        self.0.color.into()
    }

    #[wasm_bindgen(getter)]
    pub fn range(&self) -> RangeWASM {
        self.0.range.into()
    }
}

#[wasm_bindgen]
pub fn find_document_colors(
    _text_document: &text_document::TextDocument,
) -> Vec<ColorInformationWASM> {
    todo!("Implement find_document_colors")
}
