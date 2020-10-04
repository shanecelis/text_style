// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Conversion methods for [`cursive`][]’s text style types.
//!
//! *Requires the `cursive` feature.*
//!
//! This module implements these conversions:
//! - [`AnsiColor`][] to [`cursive::theme::BaseColor`][]
//! - [`Color`][] to [`cursive::theme::Color`][]
//! - [`Effect`][] to [`cursive::theme::Effect`][]
//! - [`Style`][] to [`cursive::theme::Style`][]
//! - [`StyledStr`][] and [`StyledString`][] to [`cursive::utils::markup::StyledString`][]
//!
//! # Example
//!
//! Rendering a string:
//!
//! ```
//! let text = text_style::StyledStr::plain("test").bold();
//! let mut s = cursive::dummy();
//! s.add_layer(cursive::views::TextView::new(text));
//! s.add_global_callback('q', |s| s.quit());
//! s.run();
//! ```
//!
//! [`cursive`]: https://docs.rs/cursive
//! [`cursive::theme::BaseColor`]: https://docs.rs/cursive/latest/cursive/theme/enum.BaseColor.html
//! [`cursive::theme::Color`]: https://docs.rs/cursive/latest/cursive/theme/enum.Color.html
//! [`cursive::theme::Effect`]: https://docs.rs/cursive/latest/cursive/theme/enum.Effect.html
//! [`cursive::theme::Style`]: https://docs.rs/cursive/latest/cursive/theme/struct.Style.html
//! [`cursive::utils::markup::StyledString`]: https://docs.rs/cursive/latest/cursive/utils/markup/type.StyledString.html
//! [`AnsiColor`]: ../enum.AnsiColor.html
//! [`Color`]: ../enum.Color.html
//! [`Effect`]: ../enum.Effect.html
//! [`Style`]: ../struct.Style.html
//! [`StyledStr`]: ../struct.StyledStr.html
//! [`StyledString`]: ../struct.StyledString.html

use cursive::{theme, utils::markup};

use crate::{AnsiColor, AnsiMode, Color, Effect, Style, StyledStr, StyledString};

impl From<Color> for theme::Color {
    fn from(color: Color) -> theme::Color {
        match color {
            Color::Ansi { color, mode } => match mode {
                AnsiMode::Dark => theme::Color::Dark(color.into()),
                AnsiMode::Light => theme::Color::Light(color.into()),
            },
            Color::Rgb { r, g, b } => theme::Color::Rgb(r, g, b),
        }
    }
}

impl From<AnsiColor> for theme::BaseColor {
    fn from(color: AnsiColor) -> theme::BaseColor {
        match color {
            AnsiColor::Black => theme::BaseColor::Black,
            AnsiColor::Red => theme::BaseColor::Red,
            AnsiColor::Green => theme::BaseColor::Green,
            AnsiColor::Yellow => theme::BaseColor::Yellow,
            AnsiColor::Blue => theme::BaseColor::Blue,
            AnsiColor::Magenta => theme::BaseColor::Magenta,
            AnsiColor::Cyan => theme::BaseColor::Cyan,
            AnsiColor::White => theme::BaseColor::White,
        }
    }
}

impl From<Effect> for theme::Effect {
    fn from(effect: Effect) -> theme::Effect {
        match effect {
            Effect::Bold => theme::Effect::Bold,
            Effect::Italic => theme::Effect::Italic,
            Effect::Underline => theme::Effect::Underline,
            Effect::Strikethrough => theme::Effect::Strikethrough,
        }
    }
}

impl From<Style> for theme::Style {
    fn from(style: Style) -> theme::Style {
        theme::Style {
            effects: style.effects.into_iter().map(theme::Effect::from).collect(),
            color: get_color_style(style.fg, style.bg),
        }
    }
}

fn get_color_style(fg: Option<Color>, bg: Option<Color>) -> Option<theme::ColorStyle> {
    let mut front = theme::Color::TerminalDefault;
    let mut back = theme::Color::TerminalDefault;

    if let Some(fg) = fg {
        front = fg.into();
    }
    if let Some(bg) = bg {
        back = bg.into();
    }

    if front == theme::Color::TerminalDefault && back == theme::Color::TerminalDefault {
        None
    } else {
        Some(theme::ColorStyle::new(front, back))
    }
}

impl<'a, 'b> From<&'b StyledStr<'a>> for markup::StyledString {
    fn from(s: &'b StyledStr<'a>) -> markup::StyledString {
        if let Some(style) = s.style {
            markup::StyledString::styled(s.s.to_owned(), style)
        } else {
            markup::StyledString::plain(s.s.to_owned())
        }
    }
}

impl<'a> From<StyledStr<'a>> for markup::StyledString {
    fn from(s: StyledStr<'a>) -> markup::StyledString {
        if let Some(style) = s.style {
            markup::StyledString::styled(s.s.to_owned(), style)
        } else {
            markup::StyledString::plain(s.s.to_owned())
        }
    }
}

impl From<StyledString> for markup::StyledString {
    fn from(s: StyledString) -> markup::StyledString {
        if let Some(style) = s.style {
            markup::StyledString::styled(s.s, style)
        } else {
            markup::StyledString::plain(s.s)
        }
    }
}
