use crate::converters::line_index::LineIndex;
use crate::converters::{LineCol, PositionEncoding, WideLineCol};
use biome_rowan::{TextRange, TextSize};

/// The function is used to convert a LSP position to TextSize.
pub(crate) fn offset(
    line_index: &LineIndex,
    position: lsp_types::Position,
    position_encoding: PositionEncoding,
) -> Result<TextSize, ()> {
    let line_col = match position_encoding {
        PositionEncoding::Utf8 => LineCol {
            line: position.line,
            col: position.character,
        },
        PositionEncoding::Wide(enc) => {
            let line_col = WideLineCol {
                line: position.line,
                col: position.character,
            };
            line_index.to_utf8(enc, line_col)
        }
    };

    line_index.offset(line_col).ok_or(())
}

/// The function is used to convert a LSP range to TextRange.
#[allow(dead_code)]
pub(crate) fn text_range(
    line_index: &LineIndex,
    range: lsp_types::Range,
    position_encoding: PositionEncoding,
) -> Result<TextRange, ()> {
    let start = offset(line_index, range.start, position_encoding)?;
    let end = offset(line_index, range.end, position_encoding)?;

    Ok(TextRange::new(start, end))
}
