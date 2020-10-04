// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Conversion methods for [`crossterm`][]’s text style types.
//!
//! *Requires the `crossterm` feature.*
//!
//! This module implements these conversions:
//! - [`Color`][] to [`crossterm::style::Color`][]
//! - [`Effect`][] to [`crossterm::style::Attribute`][]
//! - [`Effects`][] to [`crossterm::style::Attributes`][]
//! - [`Style`][] to [`crossterm::style::ContentStyle`][]
//! - [`StyledStr`][] and [`StyledString`][] to [`crossterm::style::StyledContent`][]
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
//! text_style::crossterm::render(std::io::stdout(), s)
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
//! text_style::crossterm::render_iter(std::io::stdout(), v.iter())
//!     .expect("Failed to render string");
//! ```
//!
//! [`crossterm`]: https://docs.rs/crossterm
//! [`crossterm::style::Attribute`]: https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html
//! [`crossterm::style::Attributes`]: https://docs.rs/crossterm/latest/crossterm/style/struct.Attributes.html
//! [`crossterm::style::Color`]: https://docs.rs/crossterm/latest/crossterm/style/enum.Color.html
//! [`crossterm::style::ContentStyle`]: https://docs.rs/crossterm/latest/crossterm/style/struct.ContentStyle.html
//! [`crossterm::style::StyledContent`]: https://docs.rs/crossterm/latest/crossterm/style/struct.StyledContent.html
//! [`Color`]: ../enum.Color.html
//! [`Effect`]: ../enum.Effect.html
//! [`Effects`]: ../struct.Effects.html
//! [`Style`]: ../struct.Style.html
//! [`StyledStr`]: ../struct.StyledStr.html
//! [`StyledString`]: ../struct.StyledString.html
//! [`render`]: fn.render.html
//! [`render_iter`]: fn.render_iter.html

use std::io;

use crossterm::style;

use crate::{AnsiColor, AnsiMode, Color, Effect, Effects, Style, StyledStr, StyledString};

impl From<Color> for style::Color {
    fn from(color: Color) -> style::Color {
        use AnsiColor::*;
        use AnsiMode::*;

        match color {
            Color::Ansi { color, mode } => match (mode, color) {
                (Dark, Black) => style::Color::Black,
                (Dark, Red) => style::Color::DarkRed,
                (Dark, Green) => style::Color::DarkGreen,
                (Dark, Yellow) => style::Color::DarkYellow,
                (Dark, Blue) => style::Color::DarkBlue,
                (Dark, Magenta) => style::Color::DarkMagenta,
                (Dark, Cyan) => style::Color::DarkCyan,
                // TODO: check greys
                (Dark, White) => style::Color::Grey,
                (Light, Black) => style::Color::DarkGrey,
                (Light, Red) => style::Color::Red,
                (Light, Green) => style::Color::Green,
                (Light, Yellow) => style::Color::Yellow,
                (Light, Blue) => style::Color::Blue,
                (Light, Magenta) => style::Color::Magenta,
                (Light, Cyan) => style::Color::Cyan,
                (Light, White) => style::Color::White,
            },
            Color::Rgb { r, g, b } => style::Color::Rgb { r, g, b },
        }
    }
}

impl From<Effect> for style::Attribute {
    fn from(effect: Effect) -> style::Attribute {
        match effect {
            Effect::Bold => style::Attribute::Bold,
            Effect::Italic => style::Attribute::Italic,
            Effect::Underline => style::Attribute::Underlined,
            Effect::Strikethrough => style::Attribute::CrossedOut,
        }
    }
}

impl From<Effects> for style::Attributes {
    fn from(effects: Effects) -> style::Attributes {
        let mut attributes = style::Attributes::default();
        for effect in effects {
            attributes.set(effect.into());
        }
        attributes
    }
}

impl From<Style> for style::ContentStyle {
    fn from(style: Style) -> style::ContentStyle {
        style::ContentStyle {
            foreground_color: style.fg.map(Into::into),
            background_color: style.bg.map(Into::into),
            attributes: style.effects.into(),
        }
    }
}

impl<'a, 'b> From<&'b StyledStr<'a>> for style::StyledContent<&'a str> {
    fn from(s: &'b StyledStr<'a>) -> style::StyledContent<&'a str> {
        style::StyledContent::new(s.style.map(Into::into).unwrap_or_default(), s.s)
    }
}

impl<'a> From<StyledStr<'a>> for style::StyledContent<&'a str> {
    fn from(s: StyledStr<'a>) -> style::StyledContent<&'a str> {
        style::StyledContent::new(s.style.map(Into::into).unwrap_or_default(), s.s)
    }
}

impl From<StyledString> for style::StyledContent<String> {
    fn from(s: StyledString) -> style::StyledContent<String> {
        style::StyledContent::new(s.style.map(Into::into).unwrap_or_default(), s.s)
    }
}

/// Renders a styled string to the given output using `crossterm`.
///
/// # Example
///
/// ```
/// let s = text_style::StyledStr::plain("test").bold();
/// text_style::crossterm::render(std::io::stdout(), s)
///     .expect("Failed to render string");
/// ```
pub fn render<'a>(mut w: impl io::Write, s: impl Into<StyledStr<'a>>) -> crossterm::Result<()> {
    use crossterm::ExecutableCommand;

    w.execute(crossterm::style::PrintStyledContent(s.into().into()))
        .map(|_| {})
}

/// Renders multiple styled string to the given output using `crossterm`.
///
/// This function queues the draw commands, so the output has to be flushed by the caller.
///
/// # Example
///
/// ```
/// let v = vec![
///     text_style::StyledStr::plain("test").bold(),
///     text_style::StyledStr::plain(" "),
///     text_style::StyledStr::plain("test2").italic(),
/// ];
/// text_style::crossterm::render_iter(std::io::stdout(), v.iter())
///     .expect("Failed to render string");
/// ```
pub fn render_iter<'a, I, Iter, S, W>(mut w: W, iter: I) -> crossterm::Result<()>
where
    I: IntoIterator<Item = S, IntoIter = Iter>,
    Iter: Iterator<Item = S>,
    S: Into<StyledStr<'a>>,
    W: io::Write,
{
    use crossterm::QueueableCommand;

    for s in iter {
        w.queue(crossterm::style::PrintStyledContent(s.into().into()))?;
    }
    Ok(())
}
