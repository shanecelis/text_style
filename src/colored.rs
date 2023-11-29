use colored;

use crate::{Color, Effects, Style, StyledStr};

impl From<colored::Color> for Color {
    fn from(color: colored::Color) -> Color {
        use colored::Color::*;
        use crate::AnsiColor;
        use crate::AnsiMode::*;
        match color {
            Black => Color::Ansi { color: AnsiColor::Black, mode: Dark },
            _ => todo!()
        }
    }
}

// impl From<highlighting::FontStyle> for Effects {
//     fn from(font_style: highlighting::FontStyle) -> Effects {
//         Effects {
//             is_bold: font_style.contains(highlighting::FontStyle::BOLD),
//             is_italic: font_style.contains(highlighting::FontStyle::ITALIC),
//             is_underline: font_style.contains(highlighting::FontStyle::UNDERLINE),
//             is_strikethrough: false,
//         }
//     }
// }

// impl From<highlighting::Style> for Style {
//     fn from(style: highlighting::Style) -> Style {
//         Style {
//             fg: Some(style.foreground.into()),
//             bg: Some(style.background.into()),
//             effects: style.font_style.into(),
//         }
//     }
// }

// impl<'a, 'b> From<&'b (highlighting::Style, &'a str)> for StyledStr<'a> {
//     fn from((style, s): &'b (highlighting::Style, &'a str)) -> StyledStr<'a> {
//         StyledStr {
//             s,
//             style: Some(Style::from(*style)),
//         }
//     }
// }

// impl<'a> From<(highlighting::Style, &'a str)> for StyledStr<'a> {
//     fn from((style, s): (highlighting::Style, &'a str)) -> StyledStr<'a> {
//         StyledStr {
//             s,
//             style: Some(style.into()),
//         }
//     }
// }
