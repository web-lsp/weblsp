use palette::named;
use palette::*;
use std::f32::consts::PI;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

/// The alpha value for opaque colors
pub const OPAQUE: f32 = 1.0;

/// A trait that allows converting between CSS colors and LSP colors and serializing the color back to CSS
pub trait CSSColor {
    fn to_lsp_color(&self) -> lsp_types::Color;
    fn from_lsp_color(lsp_color: lsp_types::Color) -> Self
    where
        Self: Sized;
    // TODO: CSSColor should hold what each component of a color was parsed as, so that it's possible when doing color presentations
    // to show the user the original color they wrote, instead of the color that was parsed. Ideally, this would be configurable.
    fn to_css(&self) -> String;
}

impl CSSColor for Srgba {
    fn to_lsp_color(&self) -> lsp_types::Color {
        lsp_types::Color {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: self.alpha,
        }
    }

    fn from_lsp_color(lsp_color: lsp_types::Color) -> Self {
        Srgba::new(
            lsp_color.red,
            lsp_color.green,
            lsp_color.blue,
            lsp_color.alpha,
        )
    }

    fn to_css(&self) -> String {
        let u8_color: Srgba<u8> = self.into_format();

        if self.alpha == OPAQUE {
            format!(
                "rgb({:.0} {:.0} {:.0})",
                u8_color.red, u8_color.green, u8_color.blue
            )
        } else {
            format!(
                "rgb({:.0} {:.0} {:.0} / {:.0}%)",
                u8_color.red,
                u8_color.green,
                u8_color.blue,
                self.alpha * 100.0
            )
        }
    }
}

impl CSSColor for Hsla {
    fn to_lsp_color(&self) -> lsp_types::Color {
        let color: Srgba = (*self).into_color();
        color.to_lsp_color()
    }

    fn from_lsp_color(lsp_color: lsp_types::Color) -> Self {
        let color: Srgba = Srgba::from_lsp_color(lsp_color);
        color.into_color()
    }

    fn to_css(&self) -> String {
        if self.alpha == OPAQUE {
            format!(
                "hsl({:.0} {:.0}% {:.0}%)",
                self.hue.into_positive_degrees(),
                self.saturation * 100.0,
                self.lightness * 100.0
            )
        } else {
            format!(
                "hsl({:.0} {:.0}% {:.0}% / {:.0}%)",
                self.hue.into_positive_degrees(),
                self.saturation * 100.0,
                self.lightness * 100.0,
                self.alpha * 100.0
            )
        }
    }
}

impl CSSColor for Hwba {
    fn to_lsp_color(&self) -> lsp_types::Color {
        let color: Srgba = self.into_format().into_color();
        color.to_lsp_color()
    }

    fn from_lsp_color(lsp_color: lsp_types::Color) -> Self {
        let color: Srgba = Srgba::from_lsp_color(lsp_color);
        color.into_color()
    }

    fn to_css(&self) -> String {
        if self.alpha == OPAQUE {
            format!(
                "hwb({:.0} {:.0}% {:.0}%)",
                self.hue.into_positive_degrees(),
                self.whiteness * 100.0,
                self.blackness * 100.0
            )
        } else {
            format!(
                "hwb({:.0} {:.0}% {:.0}% / {:.0}%)",
                self.hue.into_positive_degrees(),
                self.whiteness * 100.0,
                self.blackness * 100.0,
                self.alpha * 100.0
            )
        }
    }
}

impl CSSColor for Laba {
    fn to_lsp_color(&self) -> lsp_types::Color {
        let color: Srgba = (*self).into_color();
        color.to_lsp_color()
    }

    fn from_lsp_color(lsp_color: lsp_types::Color) -> Self {
        let color: Srgba = Srgba::from_lsp_color(lsp_color);
        color.into_color()
    }

    fn to_css(&self) -> String {
        if self.alpha == OPAQUE {
            format!("lab({:.2}% {:.2} {:.2})", self.l, self.a, self.b)
        } else {
            format!(
                "lab({:.2}% {:.2} {:.2} / {:.0}%)",
                self.l,
                self.a,
                self.b,
                self.alpha * 100.0
            )
        }
    }
}

impl CSSColor for Lcha {
    fn to_lsp_color(&self) -> lsp_types::Color {
        let color: Srgba = (*self).into_color();
        color.to_lsp_color()
    }

    fn from_lsp_color(lsp_color: lsp_types::Color) -> Self {
        let color: Srgba = Srgba::from_lsp_color(lsp_color);
        color.into_color()
    }

    fn to_css(&self) -> String {
        if self.alpha == OPAQUE {
            format!(
                "lch({:.2}% {:.2} {:.2})",
                self.l,
                self.chroma,
                self.hue.into_positive_degrees()
            )
        } else {
            format!(
                "lch({:.2}% {:.2} {:.2} / {:.0}%)",
                self.l,
                self.chroma,
                self.hue.into_positive_degrees(),
                self.alpha * 100.0
            )
        }
    }
}

// The parser below is adapted from https://github.com/mazznoer/csscolorparser-rs/blob/660b1eebaee0da32c926db69c023d29694867b86/src/parser/mod.rs to instead return palette's color types

/// Parses a string that might be a color in any supported CSS format (hex, rgb, hsl, hwb, lab, lch, named colors etc) into a palette color
pub fn parse_css_color(text: &str) -> Option<Box<dyn CSSColor>> {
    let text = text.trim().to_lowercase();

    if text == "transparent" {
        return Some(Box::new(Srgba::new(0.0, 0.0, 0.0, 0.0)));
    }

    if let Some(text) = text.strip_prefix('#') {
        match text.len() {
            3 | 6 => {
                let color: Srgb = Srgb::from_str(text).unwrap();
                return Some(Box::new(Srgba::from(color)));
            }
            4 | 8 => {
                let color: Srgba = Srgba::from_str(text).unwrap();
                return Some(Box::new(color));
            }
            _ => return None,
        }
    }

    if let (Some(i), Some(text)) = (text.find('('), text.strip_suffix(')')) {
        let function_name = &text[..i].trim_end();

        // CSS functions have a legacy and a modern shape, the legacy shape uses commas as separators, while the modern shape uses spaces
        let cleaned_content = &text[i + 1..].replace([',', '/'], " ");

        // TODO: Handle relative colors. They add an aditional parameter to the color, which contains spaces so this
        // implementation will not work for them. Luckily, relative colors are quite rare.
        let params = cleaned_content.split_whitespace().collect::<Vec<&str>>();
        let parameter_count = params.len();

        match *function_name {
            // RGB
            // In modern shapes, looks like this: rgb(255 0 0), or rgb(255 0 0 / 0.5)
            "rgb" | "rgba" => {
                let r = parse_percent_or_255(params[0])?;
                let g = parse_percent_or_255(params[1])?;
                let b = parse_percent_or_255(params[2])?;
                let a = if parameter_count == 4 {
                    parse_percent_or_float(params[3])?
                } else {
                    OPAQUE
                };

                return Some(Box::new(Srgba::new(r, g, b, a)));
            }
            // HSL
            // In modern shapes, looks ike this: hsl(120 100% 50%), or hsl(120 100% 50% / 0.5), or with degrees: hsl(120deg 100% 50%)
            "hsl" | "hsla" => {
                let h = parse_angle(params[0])?;
                let s = parse_percent_or_float(params[1])?;
                let l = parse_percent_or_float(params[2])?;
                let a = if parameter_count == 4 {
                    parse_percent_or_float(params[3])?
                } else {
                    OPAQUE
                };

                return Some(Box::new(Hsla::new(h, s, l, a)));
            }
            // HWB
            // Looks like this: hwb(120, 100%, 50%), or hwb(120 100% 50% / 0.5), or with degrees: hwb(0.5turn 10% 0% / .5);
            "hwb" | "hwba" => {
                let h = parse_angle(params[0])?;
                let w = parse_percent_or_float(params[1])?;
                let b = parse_percent_or_float(params[2])?;
                let a = if parameter_count == 4 {
                    parse_percent_or_float(params[3])?
                } else {
                    OPAQUE
                };

                return Some(Box::new(Hwba::new(h, w, b, a)));
            }
            // LAB
            // Looks like this: lab(52.2345% 40.1645 59.9971), or lab(52.2345% 40.1645 59.9971 / .5)
            "lab" => {
                let (l, is_l_percent) = parse_remapped_percent_or_float(params[0])?;
                let (a, is_a_percent) = parse_remapped_percent_or_float(params[1])?;
                let (b, is_b_percent) = parse_remapped_percent_or_float(params[2])?;
                let alpha = if parameter_count == 4 {
                    parse_percent_or_float(params[3])?
                } else {
                    OPAQUE
                };

                // Remap percentages to the expected ranges, for l that's 0-100, and for a and b -128-127
                let l = is_l_percent
                    .then(|| map_range((0.0, 1.0), (0.0, 100.0), l))
                    .unwrap_or(l);
                let a = is_a_percent
                    .then(|| map_range((-1.0, 1.0), (-128.0, 127.0), a))
                    .unwrap_or(a);
                let b = is_b_percent
                    .then(|| map_range((-1.0, 1.0), (-128.0, 127.0), b))
                    .unwrap_or(b);

                return Some(Box::new(Laba::new(l, a, b, alpha)));
            }
            // LCH
            // Looks like this: lch(52.2345% 72.2 56.2), or lch(52.2345% 72.2 56.2 / .5)
            "lch" => {
                let (l, is_l_percent) = parse_remapped_percent_or_float(params[0])?;
                let (c, is_c_percent) = parse_remapped_percent_or_float(params[1])?;
                let h = parse_angle(params[2])?;

                let alpha = if parameter_count == 4 {
                    parse_percent_or_float(params[3])?
                } else {
                    OPAQUE
                };

                // Remap percentages to the expected ranges, for l that's 0-100, and for c 0-150
                let l = is_l_percent
                    .then(|| map_range((0.0, 1.0), (0.0, 100.0), l))
                    .unwrap_or(l);
                let c = is_c_percent
                    .then(|| map_range((0.0, 1.0), (0.0, 150.0), c))
                    .unwrap_or(c);

                return Some(Box::new(Lcha::new(l, c, h, alpha)));
            }
            _ => return None,
        }
    }

    // If we still haven't found something, it might be a named color
    if let Some(color) = named::from_str(&text) {
        let color: Srgba = color.into_format().into();
        return Some(Box::new(color));
    }

    None
}

// Strangely, Rust doesn't have a built-in function to map a value from one range to another.
/// Maps a value from one range to another
fn map_range<T>(from_range: (T, T), to_range: (T, T), s: T) -> T
where
    T: Copy + Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

/// Parses a string that might be a percentage (ex: 32%) or a 0-1 float (ex: 0.32) or a "none" value
fn parse_percent_or_float(text: &str) -> Option<f32> {
    if let Some(stripped) = text.strip_suffix('%') {
        stripped.parse().ok().map(|t: f32| t / 100.0)
    } else {
        if text == "none" {
            return Some(0.0);
        }

        text.parse().ok()
    }
}

/// Same as parse_percent_or_float, but returns a tuple with a boolean indicating if the value was a percentage, which is needed
/// for percentages that will need to be remapped to a different range
fn parse_remapped_percent_or_float(text: &str) -> Option<(f32, bool)> {
    if let Some(stripped) = text.strip_suffix('%') {
        stripped.parse().ok().map(|t: f32| (t / 100.0, true))
    } else {
        if text == "none" {
            return Some((0.0, false));
        }

        text.parse().ok().map(|t| (t, false))
    }
}

/// Parses a string that might be a percentage (ex: 32%) or a 0-255 float (ex: 128) or a "none" value
fn parse_percent_or_255(text: &str) -> Option<f32> {
    if let Some(stripped) = text.strip_suffix('%') {
        stripped.parse().ok().map(|t: f32| t / 100.0)
    } else {
        if text == "none" {
            return Some(0.0);
        }

        text.parse().ok().map(|t: f32| t / 255.0)
    }
}

/// Parses a string that might be an angle in degrees (ex: 32deg), gradians (ex: 32grad), radians (ex: 32rad) or turns (ex: 0.5turn) or a float
fn parse_angle(text: &str) -> Option<f32> {
    // Parse as degrees if the string ends with "deg"
    if let Some(stripped) = text.strip_suffix("deg") {
        stripped.parse().ok()
    // Convert gradians to degrees if the string ends with "grad"
    } else if let Some(stripped) = text.strip_suffix("grad") {
        stripped.parse().ok().map(|t: f32| t * 360.0 / 400.0)
    // Convert radians to degrees if the string ends with "rad"
    } else if let Some(stripped) = text.strip_suffix("rad") {
        stripped.parse().ok().map(|t: f32| t * 360. / (2. * PI))
    // Convert turns to degrees if the string ends with "turn"
    } else if let Some(stripped) = text.strip_suffix("turn") {
        stripped.parse().ok().map(|t: f32| t * 360.0)
    } else {
        if text == "none" {
            return Some(0.0);
        }

        text.parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rgb_colors() {
        let color = parse_css_color("rgb(255 0 0)").unwrap();
        assert_eq!(color.to_css(), "rgb(255 0 0)");

        let color = parse_css_color("rgba(255 0 0 / 0.5)").unwrap();
        assert_eq!(color.to_css(), "rgb(255 0 0 / 50%)");

        let color = parse_css_color("rgb(255 none 0 / .4)").unwrap();
        assert_eq!(color.to_css(), "rgb(255 0 0 / 40%)");

        // Edge case: mixing percentages and 255 values, supported in browsers since 2023
        let color = parse_css_color("rgb(255 50% none / 40%)").unwrap();
        assert_eq!(color.to_css(), "rgb(255 128 0 / 40%)");
    }

    #[test]
    fn test_parse_hex_colors() {
        let color = parse_css_color("#f00").unwrap();
        assert_eq!(color.to_css(), "rgb(255 0 0)");

        let color = parse_css_color("#ff0000").unwrap();
        assert_eq!(color.to_css(), "rgb(255 0 0)");

        let color = parse_css_color("#f00f").unwrap();
        assert_eq!(color.to_css(), "rgb(255 0 0)");

        let color = parse_css_color("#ff0000ff").unwrap();
        assert_eq!(color.to_css(), "rgb(255 0 0)");

        let color = parse_css_color("#ff000080").unwrap();
        assert_eq!(color.to_css(), "rgb(255 0 0 / 50%)");

        // Edge case: #ff0000fe cannot be represented correctly in RGB, so it gets rounded to 100%
        let color = parse_css_color("#ff0000fe").unwrap();
        assert_eq!(color.to_css(), "rgb(255 0 0 / 100%)");
    }

    #[test]
    fn test_parse_hsl_colors() {
        let color = parse_css_color("hsl(120 100% 50%)").unwrap();
        assert_eq!(color.to_css(), "hsl(120 100% 50%)");

        let color = parse_css_color("hsla(120 100% 50% / 0.5)").unwrap();
        assert_eq!(color.to_css(), "hsl(120 100% 50% / 50%)");

        let color = parse_css_color("hsl(120deg 100% 50%)").unwrap();
        assert_eq!(color.to_css(), "hsl(120 100% 50%)");

        let color = parse_css_color("hsl(0.5turn 100% 50%)").unwrap();
        assert_eq!(color.to_css(), "hsl(180 100% 50%)");

        let color = parse_css_color("hsl(1.5708rad 100% 50%)").unwrap();
        assert_eq!(color.to_css(), "hsl(90 100% 50%)");

        let color = parse_css_color("hsl(120grad 100% 50%)").unwrap();
        assert_eq!(color.to_css(), "hsl(108 100% 50%)");
    }

    #[test]
    fn test_parse_hwb_colors() {
        let color = parse_css_color("hwb(120 100% 50%)").unwrap();
        assert_eq!(color.to_css(), "hwb(120 100% 50%)");

        let color = parse_css_color("hwba(120 100% 50% / 0.5)").unwrap();
        assert_eq!(color.to_css(), "hwb(120 100% 50% / 50%)");

        let color = parse_css_color("hwb(1.5708rad 100% 50%)").unwrap();
        assert_eq!(color.to_css(), "hwb(90 100% 50%)");

        let color = parse_css_color("hwb(100grad 100% 50%)").unwrap();
        assert_eq!(color.to_css(), "hwb(90 100% 50%)");
    }

    #[test]
    fn test_parse_lab_colors() {
        let color = parse_css_color("lab(52.2345% 40.1645 59.9971)").unwrap();
        assert_eq!(color.to_css(), "lab(52.23% 40.16 60.00)");

        let color = parse_css_color("lab(52.2345% 40.1645 59.9971 / .5)").unwrap();
        assert_eq!(color.to_css(), "lab(52.23% 40.16 60.00 / 50%)");
    }

    #[test]
    fn test_parse_lch_colors() {
        let color = parse_css_color("lch(52.2345% 72.2 56.2)").unwrap();
        assert_eq!(color.to_css(), "lch(52.23% 72.20 56.20)");

        let color = parse_css_color("lch(52.2345% 72.2 56.2 / .5)").unwrap();
        assert_eq!(color.to_css(), "lch(52.23% 72.20 56.20 / 50%)");
    }

    #[test]
    fn test_parse_named_colors() {
        let color = parse_css_color("red").unwrap();
        assert_eq!(color.to_css(), "rgb(255 0 0)");
    }

    #[test]
    fn test_parse_misc_colors() {
        // Just `none` isn't a valid color, so it should not return a value
        let color = parse_css_color("none");
        assert!(color.is_none());

        let color = parse_css_color("transparent").unwrap();
        assert_eq!(color.to_css(), "rgb(0 0 0 / 0%)");
    }
}
