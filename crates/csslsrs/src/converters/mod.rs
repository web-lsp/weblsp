use biome_rowan::TextSize;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

pub(crate) mod from_proto;
pub(crate) mod line_index;
pub(crate) mod to_proto;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Default)]
#[wasm_bindgen]
pub enum PositionEncoding {
    Utf8,
    #[default]
    Utf16,
    Utf32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct LineCol {
    /// Zero-based
    pub(crate) line: u32,
    /// Zero-based utf8 offset
    pub(crate) col: u32,
}

/// Deliberately not a generic type and different from `LineCol`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WideLineCol {
    /// Zero-based
    pub line: u32,
    /// Zero-based
    pub col: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct WideChar {
    /// Start offset of a character inside a line, zero-based
    pub(crate) start: TextSize,
    /// End offset of a character inside a line, zero-based
    pub(crate) end: TextSize,
}

impl WideChar {
    /// Returns the length in 8-bit UTF-8 code units.
    fn len(&self) -> TextSize {
        self.end - self.start
    }

    /// Returns the length in UTF-16 or UTF-32 code units.
    fn wide_len(&self, enc: PositionEncoding) -> usize {
        match enc {
            PositionEncoding::Utf8 => panic!("Encoding isn't wide"),
            PositionEncoding::Utf16 => {
                if self.len() == TextSize::from(4) {
                    2
                } else {
                    1
                }
            }

            PositionEncoding::Utf32 => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::converters::from_proto::offset;
    use crate::converters::line_index::LineIndex;
    use crate::converters::to_proto::position;
    use crate::converters::PositionEncoding::{Utf16, Utf32};
    use crate::converters::{LineCol, PositionEncoding};
    use biome_rowan::TextSize;
    use lsp_types::Position;

    macro_rules! check_conversion {
        ($line_index:ident : $position:expr => $text_size:expr ) => {
            let position_encoding = PositionEncoding::Utf16;

            let offset = offset(&$line_index, $position, position_encoding).ok();
            assert_eq!(offset, Some($text_size));

            let position = position(&$line_index, offset.unwrap(), position_encoding).ok();

            assert_eq!(position, Some($position));
        };
    }

    #[test]
    fn empty_string() {
        let line_index = LineIndex::new("");
        check_conversion!(line_index: Position { line: 0, character: 0 } => TextSize::from(0));
    }

    #[test]
    fn empty_line() {
        let line_index = LineIndex::new("\n\n");
        check_conversion!(line_index: Position { line: 1, character: 0 } => TextSize::from(1));
    }

    #[test]
    fn line_end() {
        let line_index = LineIndex::new("abc\ndef\nghi");
        check_conversion!(line_index: Position { line: 1, character: 3 } => TextSize::from(7));
    }

    #[test]
    fn out_of_bounds_line() {
        let line_index = LineIndex::new("abcde\nfghij\n");

        let offset = line_index.offset(LineCol { line: 5, col: 0 });
        assert!(offset.is_none());
    }

    #[test]
    fn unicode() {
        let line_index = LineIndex::new("'Jan 1, 2018 – Jan 1, 2019'");

        check_conversion!(line_index: Position { line: 0, character: 0 } => TextSize::from(0));
        check_conversion!(line_index: Position { line: 0, character: 1 } => TextSize::from(1));
        check_conversion!(line_index: Position { line: 0, character: 12 } => TextSize::from(12));
        check_conversion!(line_index: Position { line: 0, character: 13 } => TextSize::from(15));
        check_conversion!(line_index: Position { line: 0, character: 14 } => TextSize::from(18));
        check_conversion!(line_index: Position { line: 0, character: 15 } => TextSize::from(21));
        check_conversion!(line_index: Position { line: 0, character: 26 } => TextSize::from(32));
        check_conversion!(line_index: Position { line: 0, character: 27 } => TextSize::from(33));
    }

    #[ignore]
    #[test]
    fn test_every_chars() {
        let text: String = {
            let mut chars: Vec<char> = ((0 as char)..char::MAX).collect();
            chars.extend("\n".repeat(chars.len() / 16).chars());
            chars.into_iter().collect()
        };

        let line_index = LineIndex::new(&text);

        let mut lin_col = LineCol { line: 0, col: 0 };
        let mut col_utf16 = 0;
        let mut col_utf32 = 0;
        for (offset, char) in text.char_indices() {
            let got_offset = line_index.offset(lin_col).unwrap();
            assert_eq!(usize::from(got_offset), offset);

            let got_lin_col = line_index.line_col(got_offset).unwrap();
            assert_eq!(got_lin_col, lin_col);

            for enc in [Utf16, Utf32] {
                let wide_lin_col = line_index.to_wide(enc, lin_col).unwrap();
                let got_lin_col = line_index.to_utf8(enc, wide_lin_col);
                assert_eq!(got_lin_col, lin_col);

                let want_col = match enc {
                    Utf16 => col_utf16,
                    Utf32 => col_utf32,
                    _ => unreachable!(),
                };
                assert_eq!(wide_lin_col.col, want_col)
            }

            if char == '\n' {
                lin_col.line += 1;
                lin_col.col = 0;
                col_utf16 = 0;
                col_utf32 = 0;
            } else {
                lin_col.col += char.len_utf8() as u32;
                col_utf16 += char.len_utf16() as u32;
                col_utf32 += 1;
            }
        }
    }
}
