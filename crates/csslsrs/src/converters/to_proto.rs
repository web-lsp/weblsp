use crate::converters::line_index::LineIndex;
use crate::converters::PositionEncoding;
use biome_rowan::{TextRange, TextSize};

/// The function is used to convert TextSize to a LSP position.
pub(crate) fn position(
    line_index: &LineIndex,
    offset: TextSize,
    position_encoding: PositionEncoding,
) -> Result<lsp_types::Position, ()> {
    let line_col = line_index.line_col(offset).ok_or(())?;

    let position = match position_encoding {
        PositionEncoding::Utf8 => lsp_types::Position::new(line_col.line, line_col.col),
        PositionEncoding::Wide(enc) => {
            let line_col = line_index.to_wide(enc, line_col).ok_or(())?;
            lsp_types::Position::new(line_col.line, line_col.col)
        }
    };

    Ok(position)
}

/// The function is used to convert TextRange to a LSP range.
pub(crate) fn range(
    line_index: &LineIndex,
    range: TextRange,
    position_encoding: PositionEncoding,
) -> Result<lsp_types::Range, ()> {
    let start = position(line_index, range.start(), position_encoding)?;
    let end = position(line_index, range.end(), position_encoding)?;
    Ok(lsp_types::Range::new(start, end))
}
