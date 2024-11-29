use palette::named;
use palette::*;
use std::f32::consts::PI;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

/// The alpha value for opaque colors
pub const OPAQUE: f32 = 1.0;

// `palette`'s named colors map isn't public, so we need to define it ourselves
// See: https://github.com/Ogeon/palette/issues/421
pub static NAMED_COLORS: phf::Map<&'static str, Srgb<u8>> = phf::phf_map! {
    "aliceblue" => Srgb::new(240, 248, 255),
    "antiquewhite" => Srgb::new(250, 235, 215),
    "aqua" => Srgb::new(0, 255, 255),
    "aquamarine" => Srgb::new(127, 255, 212),
    "azure" => Srgb::new(240, 255, 255),
    "beige" => Srgb::new(245, 245, 220),
    "bisque" => Srgb::new(255, 228, 196),
    "black" => Srgb::new(0, 0, 0),
    "blanchedalmond" => Srgb::new(255, 235, 205),
    "blue" => Srgb::new(0, 0, 255),
    "blueviolet" => Srgb::new(138, 43, 226),
    "brown" => Srgb::new(165, 42, 42),
    "burlywood" => Srgb::new(222, 184, 135),
    "cadetblue" => Srgb::new(95, 158, 160),
    "chartreuse" => Srgb::new(127, 255, 0),
    "chocolate" => Srgb::new(210, 105, 30),
    "coral" => Srgb::new(255, 127, 80),
    "cornflowerblue" => Srgb::new(100, 149, 237),
    "cornsilk" => Srgb::new(255, 248, 220),
    "crimson" => Srgb::new(220, 20, 60),
    "cyan" => Srgb::new(0, 255, 255),
    "darkblue" => Srgb::new(0, 0, 139),
    "darkcyan" => Srgb::new(0, 139, 139),
    "darkgoldenrod" => Srgb::new(184, 134, 11),
    "darkgray" => Srgb::new(169, 169, 169),
    "darkgreen" => Srgb::new(0, 100, 0),
    "darkgrey" => Srgb::new(169, 169, 169),
    "darkkhaki" => Srgb::new(189, 183, 107),
    "darkmagenta" => Srgb::new(139, 0, 139),
    "darkolivegreen" => Srgb::new(85, 107, 47),
    "darkorange" => Srgb::new(255, 140, 0),
    "darkorchid" => Srgb::new(153, 50, 204),
    "darkred" => Srgb::new(139, 0, 0),
    "darksalmon" => Srgb::new(233, 150, 122),
    "darkseagreen" => Srgb::new(143, 188, 143),
    "darkslateblue" => Srgb::new(72, 61, 139),
    "darkslategray" => Srgb::new(47, 79, 79),
    "darkslategrey" => Srgb::new(47, 79, 79),
    "darkturquoise" => Srgb::new(0, 206, 209),
    "darkviolet" => Srgb::new(148, 0, 211),
    "deeppink" => Srgb::new(255, 20, 147),
    "deepskyblue" => Srgb::new(0, 191, 255),
    "dimgray" => Srgb::new(105, 105, 105),
    "dimgrey" => Srgb::new(105, 105, 105),
    "dodgerblue" => Srgb::new(30, 144, 255),
    "firebrick" => Srgb::new(178, 34, 34),
    "floralwhite" => Srgb::new(255, 250, 240),
    "forestgreen" => Srgb::new(34, 139, 34),
    "fuchsia" => Srgb::new(255, 0, 255),
    "gainsboro" => Srgb::new(220, 220, 220),
    "ghostwhite" => Srgb::new(248, 248, 255),
    "gold" => Srgb::new(255, 215, 0),
    "goldenrod" => Srgb::new(218, 165, 32),
    "gray" => Srgb::new(128, 128, 128),
    "green" => Srgb::new(0, 128, 0),
    "greenyellow" => Srgb::new(173, 255, 47),
    "grey" => Srgb::new(128, 128, 128),
    "honeydew" => Srgb::new(240, 255, 240),
    "hotpink" => Srgb::new(255, 105, 180),
    "indianred" => Srgb::new(205, 92, 92),
    "indigo" => Srgb::new(75, 0, 130),
    "ivory" => Srgb::new(255, 255, 240),
    "khaki" => Srgb::new(240, 230, 140),
    "lavender" => Srgb::new(230, 230, 250),
    "lavenderblush" => Srgb::new(255, 240, 245),
    "lawngreen" => Srgb::new(124, 252, 0),
    "lemonchiffon" => Srgb::new(255, 250, 205),
    "lightblue" => Srgb::new(173, 216, 230),
    "lightcoral" => Srgb::new(240, 128, 128),
    "lightcyan" => Srgb::new(224, 255, 255),
    "lightgoldenrodyellow" => Srgb::new(250, 250, 210),
    "lightgray" => Srgb::new(211, 211, 211),
    "lightgreen" => Srgb::new(144, 238, 144),
    "lightgrey" => Srgb::new(211, 211, 211),
    "lightpink" => Srgb::new(255, 182, 193),
    "lightsalmon" => Srgb::new(255, 160, 122),
    "lightseagreen" => Srgb::new(32, 178, 170),
    "lightskyblue" => Srgb::new(135, 206, 250),
    "lightslategray" => Srgb::new(119, 136, 153),
    "lightslategrey" => Srgb::new(119, 136, 153),
    "lightsteelblue" => Srgb::new(176, 196, 222),
    "lightyellow" => Srgb::new(255, 255, 224),
    "lime" => Srgb::new(0, 255, 0),
    "limegreen" => Srgb::new(50, 205, 50),
    "linen" => Srgb::new(250, 240, 230),
    "magenta" => Srgb::new(255, 0, 255),
    "maroon" => Srgb::new(128, 0, 0),
    "mediumaquamarine" => Srgb::new(102, 205, 170),
    "mediumblue" => Srgb::new(0, 0, 205),
    "mediumorchid" => Srgb::new(186, 85, 211),
    "mediumpurple" => Srgb::new(147, 112, 219),
    "mediumseagreen" => Srgb::new(60, 179, 113),
    "mediumslateblue" => Srgb::new(123, 104, 238),
    "mediumspringgreen" => Srgb::new(0, 250, 154),
    "mediumturquoise" => Srgb::new(72, 209, 204),
    "mediumvioletred" => Srgb::new(199, 21, 133),
    "midnightblue" => Srgb::new(25, 25, 112),
    "mintcream" => Srgb::new(245, 255, 250),
    "mistyrose" => Srgb::new(255, 228, 225),
    "moccasin" => Srgb::new(255, 228, 181),
    "navajowhite" => Srgb::new(255, 222, 173),
    "navy" => Srgb::new(0, 0, 128),
    "oldlace" => Srgb::new(253, 245, 230),
    "olive" => Srgb::new(128, 128, 0),
    "olivedrab" => Srgb::new(107, 142, 35),
    "orange" => Srgb::new(255, 165, 0),
    "orangered" => Srgb::new(255, 69, 0),
    "orchid" => Srgb::new(218, 112, 214),
    "palegoldenrod" => Srgb::new(238, 232, 170),
    "palegreen" => Srgb::new(152, 251, 152),
    "paleturquoise" => Srgb::new(175, 238, 238),
    "palevioletred" => Srgb::new(219, 112, 147),
    "papayawhip" => Srgb::new(255, 239, 213),
    "peachpuff" => Srgb::new(255, 218, 185),
    "peru" => Srgb::new(205, 133, 63),
    "pink" => Srgb::new(255, 192, 203),
    "plum" => Srgb::new(221, 160, 221),
    "powderblue" => Srgb::new(176, 224, 230),
    "purple" => Srgb::new(128, 0, 128),
    "rebeccapurple" => Srgb::new(102, 51, 153),
    "red" => Srgb::new(255, 0, 0),
    "rosybrown" => Srgb::new(188, 143, 143),
    "royalblue" => Srgb::new(65, 105, 225),
    "saddlebrown" => Srgb::new(139, 69, 19),
    "salmon" => Srgb::new(250, 128, 114),
    "sandybrown" => Srgb::new(244, 164, 96),
    "seagreen" => Srgb::new(46, 139, 87),
    "seashell" => Srgb::new(255, 245, 238),
    "sienna" => Srgb::new(160, 82, 45),
    "silver" => Srgb::new(192, 192, 192),
    "skyblue" => Srgb::new(135, 206, 235),
    "slateblue" => Srgb::new(106, 90, 205),
    "slategray" => Srgb::new(112, 128, 144),
    "slategrey" => Srgb::new(112, 128, 144),
    "snow" => Srgb::new(255, 250, 250),
    "springgreen" => Srgb::new(0, 255, 127),
    "steelblue" => Srgb::new(70, 130, 180),
    "tan" => Srgb::new(210, 180, 140),
    "teal" => Srgb::new(0, 128, 128),
    "thistle" => Srgb::new(216, 191, 216),
    "tomato" => Srgb::new(255, 99, 71),
    "turquoise" => Srgb::new(64, 224, 208),
    "violet" => Srgb::new(238, 130, 238),
    "wheat" => Srgb::new(245, 222, 179),
    "white" => Srgb::new(255, 255, 255),
    "whitesmoke" => Srgb::new(245, 245, 245),
    "yellow" => Srgb::new(255, 255, 0),
    "yellowgreen" => Srgb::new(154, 205, 50),
};

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
        let laba_color = Laba::new(self.l, self.a, self.b, self.alpha);
        let color: Srgba = laba_color.into_color();
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
        let lcha_color = Lcha::new(self.l, self.chroma, self.hue, self.alpha);
        let color: Srgba = lcha_color.into_color();
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
                let color: Srgb = Srgb::from_str(text).unwrap().into();
                return Some(Box::new(Srgba::from(color)));
            }
            4 | 8 => {
                let color: Srgba = Srgba::from_str(text).unwrap().into_format();
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
