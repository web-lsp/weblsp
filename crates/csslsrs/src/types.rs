// Implement wasm-bindgen friendly versions of Range and Position
use lsp_types::{Position, Range};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Position)]
pub struct PositionWASM(Position);

#[wasm_bindgen(js_name = Range)]
pub struct RangeWASM(Range);

#[wasm_bindgen(js_class = Position)]
impl PositionWASM {
    #[wasm_bindgen(getter)]
    pub fn line(&self) -> u32 {
        self.0.line
    }

    #[wasm_bindgen(getter)]
    pub fn character(&self) -> u32 {
        self.0.character
    }
}

impl From<Position> for PositionWASM {
    fn from(position: Position) -> Self {
        PositionWASM(position)
    }
}

impl From<Range> for RangeWASM {
    fn from(range: Range) -> Self {
        RangeWASM(range)
    }
}

#[wasm_bindgen(js_class = Range)]
impl RangeWASM {
    #[wasm_bindgen(getter)]
    pub fn start(&self) -> PositionWASM {
        self.0.start.into()
    }

    #[wasm_bindgen(getter)]
    pub fn end(&self) -> PositionWASM {
        self.0.end.into()
    }
}
