// SPDX-FileCopyrightText: 2020-2021 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Conversion methods for [`syntect`][]’s text style types.
//!
//! *Requires the `syntect` feature.*
//!
//! This module implements these conversions:
//! - [`syntect::highlighting::Color`][] to [`Color`][]
//! - [`syntect::highlighting::FontStyle`][] to [`Effects`][]
//! - [`syntect::highlighting::Style`][] to [`Style`][]
//! - `(&str, syntect::highlighting::Style)` to [`StyledStr`][]
//!
//! # Example
//!
//! Converting highlighted ranges to styled strings and rendering them:
//!
//! ```
//! use syntect::{easy, parsing, highlighting, util};
//!
//! let ps = parsing::SyntaxSet::load_defaults_newlines();
//! let ts = highlighting::ThemeSet::load_defaults();
//!
//! let syntax = ps.find_syntax_by_extension("rs").unwrap();
//! let mut h = easy::HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
//! let s = "pub struct Wow { hi: u64 }\nfn blah() -> u64 {}";
//! for line in util::LinesWithEndings::from(s) {
//!     let ranges: Vec<(highlighting::Style, &str)> = h.highlight(line, &ps);
//!     text_style::ansi_term::render_iter(std::io::stdout(), ranges.iter())
//!         .expect("Could not render line");
//! }
//! ```
//!
//! [`syntect`]: https://docs.rs/syntect
//! [`syntect::highlighting::Color`]: https://docs.rs/syntect/latest/syntect/highlighting/struct.Color.html
//! [`syntect::highlighting::FontStyle`]: https://docs.rs/syntect/latest/syntect/highlighting/struct.Style.html
//! [`syntect::highlighting::Style`]: https://docs.rs/syntect/latest/syntect/highlighting/struct.Style.html
//! [`Color`]: ../enum.Color.html
//! [`Effects`]: ../struct.Effects.html
//! [`Style`]: ../struct.Style.html
//! [`StyledStr`]: ../struct.StyledStr.html

use syntect::highlighting;

use crate::{Color, Effects, Style, StyledStr};

impl From<highlighting::Color> for Color {
    fn from(color: highlighting::Color) -> Color {
        Color::Rgb {
            r: color.r,
            g: color.g,
            b: color.b,
        }
    }
}

impl From<highlighting::FontStyle> for Effects {
    fn from(font_style: highlighting::FontStyle) -> Effects {
        Effects {
            is_bold: font_style.contains(highlighting::FontStyle::BOLD),
            is_italic: font_style.contains(highlighting::FontStyle::ITALIC),
            is_underline: font_style.contains(highlighting::FontStyle::UNDERLINE),
            is_strikethrough: false,
        }
    }
}

impl From<highlighting::Style> for Style {
    fn from(style: highlighting::Style) -> Style {
        Style {
            fg: Some(style.foreground.into()),
            bg: Some(style.background.into()),
            effects: style.font_style.into(),
        }
    }
}

impl<'a, 'b> From<&'b (highlighting::Style, &'a str)> for StyledStr<'a> {
    fn from((style, s): &'b (highlighting::Style, &'a str)) -> StyledStr<'a> {
        StyledStr {
            s,
            style: Some(Style::from(*style)),
        }
    }
}

impl<'a> From<(highlighting::Style, &'a str)> for StyledStr<'a> {
    fn from((style, s): (highlighting::Style, &'a str)) -> StyledStr<'a> {
        StyledStr {
            s,
            style: Some(style.into()),
        }
    }
}
