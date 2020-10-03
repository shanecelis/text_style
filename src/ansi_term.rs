// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Conversion methods for [`ansi_term`][]’s text style types.
//!
//! *Requires the `ansi_term` feature.*
//!
//! This module implements these conversions:
//! - [`Color`][] to [`ansi_term::Color`][]
//! - [`Style`][] to [`ansi_term::Style`][]
//! - [`StyledStr`][] and [`StyledString`][] to [`ansi_term::ANSIString`][]
//!
//! It also provides the [`render`][] and [`render_iter`][] methods to render strings and iterators
//! over strings.
//!
//! # Examples
//!
//! Rendering a single string:
//!
//! ```
//! let s = text_style::StyledStr::plain("test").bold();
//! text_style::ansi_term::render(std::io::stdout(), s)
//!     .expect("Failed to render string");
//! ```
//!
//! Rendering multiple strings:
//!
//! ```
//! let v = vec![
//!     text_style::StyledStr::plain("test").bold(),
//!     text_style::StyledStr::plain(" "),
//!     text_style::StyledStr::plain("test2").italic(),
//! ];
//! text_style::ansi_term::render_iter(std::io::stdout(), v.iter())
//!     .expect("Failed to render string");
//! ```
//!
//! [`ansi_term`]: https://docs.rs/ansi_term
//! [`ansi_term::ANSIString`]: https://docs.rs/ansi_term/latest/ansi_term/type.ANSIString.html
//! [`ansi_term::Color`]: https://docs.rs/ansi_term/latest/ansi_term/enum.Color.html
//! [`ansi_term::Style`]: https://docs.rs/ansi_term/latest/ansi_term/struct.Style.html
//! [`Color`]: ../enum.Color.html
//! [`Style`]: ../struct.Style.html
//! [`StyledStr`]: ../struct.StyledStr.html
//! [`StyledString`]: ../struct.StyledString.html
//! [`render`]: fn.render.html
//! [`render_iter`]: fn.render_iter.html

use std::io;

use crate::{AnsiColor, AnsiMode, Color, Effect, Style, StyledStr, StyledString};

impl From<Color> for ansi_term::Color {
    fn from(color: Color) -> ansi_term::Color {
        match color {
            Color::Ansi { color, mode } => match mode {
                AnsiMode::Dark => get_dark_color(color),
                AnsiMode::Light => get_light_color(color),
            },
            Color::Rgb { r, g, b } => ansi_term::Color::RGB(r, g, b),
        }
    }
}

fn get_dark_color(color: AnsiColor) -> ansi_term::Color {
    match color {
        AnsiColor::Black => ansi_term::Color::Black,
        AnsiColor::Red => ansi_term::Color::Red,
        AnsiColor::Green => ansi_term::Color::Green,
        AnsiColor::Yellow => ansi_term::Color::Yellow,
        AnsiColor::Blue => ansi_term::Color::Blue,
        AnsiColor::Magenta => ansi_term::Color::Purple,
        AnsiColor::Cyan => ansi_term::Color::Cyan,
        AnsiColor::White => ansi_term::Color::White,
    }
}

fn get_light_color(color: AnsiColor) -> ansi_term::Color {
    match color {
        AnsiColor::Black => ansi_term::Color::Fixed(8),
        AnsiColor::Red => ansi_term::Color::Fixed(9),
        AnsiColor::Green => ansi_term::Color::Fixed(10),
        AnsiColor::Yellow => ansi_term::Color::Fixed(11),
        AnsiColor::Blue => ansi_term::Color::Fixed(12),
        AnsiColor::Magenta => ansi_term::Color::Fixed(13),
        AnsiColor::Cyan => ansi_term::Color::Fixed(14),
        AnsiColor::White => ansi_term::Color::Fixed(15),
    }
}

impl From<Style> for ansi_term::Style {
    fn from(style: Style) -> ansi_term::Style {
        ansi_term::Style {
            foreground: style.fg.map(Into::into),
            background: style.bg.map(Into::into),
            is_bold: style.effects.contains(Effect::Bold),
            is_italic: style.effects.contains(Effect::Italic),
            is_underline: style.effects.contains(Effect::Underline),
            ..Default::default()
        }
    }
}

impl<'a, 'b> From<&'b StyledStr<'a>> for ansi_term::ANSIString<'a> {
    fn from(s: &'b StyledStr<'a>) -> ansi_term::ANSIString<'a> {
        s.style
            .map_or_else(ansi_term::Style::new, From::from)
            .paint(s.s)
    }
}

impl<'a> From<StyledStr<'a>> for ansi_term::ANSIString<'a> {
    fn from(s: StyledStr<'a>) -> ansi_term::ANSIString<'a> {
        s.style
            .map_or_else(ansi_term::Style::new, From::from)
            .paint(s.s)
    }
}

impl<'a> From<StyledString> for ansi_term::ANSIString<'a> {
    fn from(s: StyledString) -> ansi_term::ANSIString<'a> {
        s.style
            .map_or_else(ansi_term::Style::new, From::from)
            .paint(s.s)
    }
}

/// Renders a styled string to the given output using `ansi_term`.
///
/// # Example
///
/// ```
/// let s = text_style::StyledStr::plain("test").bold();
/// text_style::ansi_term::render(std::io::stdout(), s)
///     .expect("Failed to render string");
/// ```
pub fn render<'a>(mut w: impl io::Write, s: impl Into<StyledStr<'a>>) -> io::Result<()> {
    write!(w, "{}", ansi_term::ANSIString::from(s.into()))
}

/// Renders multiple styled string to the given output using `ansi_term`.
///
/// This function uses [`ansi_term::ANSIStrings`][] to minimize the written control sequences.
///
/// # Example
///
/// ```
/// let v = vec![
///     text_style::StyledStr::plain("test").bold(),
///     text_style::StyledStr::plain(" "),
///     text_style::StyledStr::plain("test2").italic(),
/// ];
/// text_style::ansi_term::render_iter(std::io::stdout(), v.iter())
///     .expect("Failed to render string");
/// ```
///
/// [`ansi_term::ANSIStrings`]: https://docs.rs/ansi_term/latest/ansi_term/fn.ANSIStrings.html
pub fn render_iter<'a, I, Iter, S, W>(mut w: W, iter: I) -> io::Result<()>
where
    I: IntoIterator<Item = S, IntoIter = Iter>,
    Iter: Iterator<Item = S>,
    S: Into<StyledStr<'a>>,
    W: io::Write,
{
    let strings: Vec<_> = iter
        .into_iter()
        .map(Into::into)
        .map(ansi_term::ANSIString::from)
        .collect();
    write!(w, "{}", ansi_term::ANSIStrings(&strings))
}
