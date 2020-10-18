// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Conversion methods for [`genpdf`][]’s text style types.
//!
//! *Requires the `genpdf` feature.*
//!
//! This module implements these conversions:
//! - [`Color`][] to [`genpdf::style::Color`][]
//! - [`Style`][] to [`genpdf::style::Style`][]
//! - [`StyledStr`][] and [`StyledString`][] to [`genpdf::style::StyledStr`][] and
//!   [`genpdf::style::StyledString`][]
//!
//! # Example
//!
//! Adding a string to a paragraph:
//!
//! ```
//! let text = text_style::StyledStr::plain("test").bold();
//! let p = genpdf::elements::Paragraph::new(text);
//! ```
//!
//! [`genpdf`]: https://docs.rs/genpdf
//! [`genpdf::style::Color`]: https://docs.rs/genpdf/latest/genpdf/style/enum.Color.html
//! [`genpdf::style::Style`]: https://docs.rs/genpdf/latest/genpdf/style/struct.Style.html
//! [`genpdf::style::StyledStr`]: https://docs.rs/genpdf/latest/genpdf/style/struct.StyledStr.html
//! [`genpdf::style::StyledString`]: https://docs.rs/genpdf/latest/genpdf/style/struct.StyledString.html
//! [`Color`]: ../enum.Color.html
//! [`Style`]: ../struct.Style.html
//! [`StyledStr`]: ../struct.StyledStr.html
//! [`StyledString`]: ../struct.StyledString.html

use genpdf::style;

use crate::{AnsiColor, AnsiMode, Color, Style, StyledStr, StyledString};

impl From<Color> for style::Color {
    fn from(c: Color) -> style::Color {
        match c {
            Color::Ansi { color, mode } => get_rgb_color(color, mode),
            Color::Rgb { r, g, b } => style::Color::Rgb(r, g, b),
        }
    }
}

fn get_rgb_color(color: AnsiColor, mode: AnsiMode) -> style::Color {
    use AnsiColor::*;
    use AnsiMode::*;

    let (r, g, b) = match (mode, color) {
        (Dark, Black) => (0, 0, 0),
        (Dark, Red) => (170, 0, 0),
        (Dark, Green) => (0, 170, 0),
        (Dark, Yellow) => (170, 85, 0),
        (Dark, Blue) => (0, 0, 170),
        (Dark, Magenta) => (170, 0, 170),
        (Dark, Cyan) => (0, 170, 170),
        (Dark, White) => (170, 170, 170),
        (Light, Black) => (85, 85, 85),
        (Light, Red) => (255, 85, 85),
        (Light, Green) => (85, 255, 85),
        (Light, Yellow) => (255, 255, 85),
        (Light, Blue) => (85, 85, 255),
        (Light, Magenta) => (255, 85, 255),
        (Light, Cyan) => (85, 255, 255),
        (Light, White) => (255, 255, 255),
    };
    style::Color::Rgb(r, g, b)
}

impl From<Style> for style::Style {
    fn from(s: Style) -> style::Style {
        let mut style = style::Style::new();
        if let Some(color) = s.fg {
            style.set_color(color.into());
        }
        if s.effects.is_bold {
            style.set_bold();
        }
        if s.effects.is_italic {
            style.set_italic();
        }
        style
    }
}

impl<'a, 's> From<&'a StyledStr<'s>> for style::StyledStr<'s> {
    fn from(s: &'a StyledStr<'s>) -> style::StyledStr<'s> {
        style::StyledStr::new(s.s, s.style.map(style::Style::from).unwrap_or_default())
    }
}

impl<'s> From<StyledStr<'s>> for style::StyledStr<'s> {
    fn from(s: StyledStr<'s>) -> style::StyledStr<'s> {
        style::StyledStr::new(s.s, s.style.map(style::Style::from).unwrap_or_default())
    }
}

impl<'s> From<&'s StyledString> for style::StyledStr<'s> {
    fn from(s: &'s StyledString) -> style::StyledStr<'s> {
        style::StyledStr::new(&s.s, s.style.map(style::Style::from).unwrap_or_default())
    }
}

impl<'a, 's> From<&'a StyledStr<'s>> for style::StyledString {
    fn from(s: &'a StyledStr<'s>) -> style::StyledString {
        style::StyledString::new(s.s, s.style.map(style::Style::from).unwrap_or_default())
    }
}

impl<'s> From<StyledStr<'s>> for style::StyledString {
    fn from(s: StyledStr<'s>) -> style::StyledString {
        style::StyledString::new(s.s, s.style.map(style::Style::from).unwrap_or_default())
    }
}

impl<'s> From<&'s StyledString> for style::StyledString {
    fn from(s: &'s StyledString) -> style::StyledString {
        style::StyledString::new(&s.s, s.style.map(style::Style::from).unwrap_or_default())
    }
}

impl From<StyledString> for style::StyledString {
    fn from(s: StyledString) -> style::StyledString {
        style::StyledString::new(s.s, s.style.map(style::Style::from).unwrap_or_default())
    }
}
