// SPDX-FileCopyrightText: 2023 Shane Celis <shane.celis@gmail.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Conversion methods for [`bevy`][]’s text style types.
//!
//! *Requires the `bevy` feature.*
//!
//! This module implements these conversions:
//! - [`Color`][] to [`bevy::prelude::Color`][]
//! - [`Style`][] to [`genpdf::style::Style`][]
//! - [`StyledStr`][] and [`StyledString`][] to [`bevy::prelude::TextBundle`][] and
//!
//! # Example
//!
//! Adding a string to a paragraph:
//!
//! ```
//! # use bevy::prelude::*;
//! fn setup(mut commands: Commands) {
//!    let text = text_style::StyledStr::plain("test").bold();
//!    commands
//!        .spawn(NodeBundle::default())
//!        .with_children(|parent| {
//!            text_style::bevy::render(
//!                parent,
//!                &(TextStyle {
//!                    font_size: 50.0,
//!                    ..default()
//!                }.into()),
//!                text
//!            );
//!        });
//! }
//! ```
use crate::{AnsiColor, AnsiMode, Color, Style, StyledStr, StyledString};
use bevy::{
    self,
    prelude::{Color as bevy_Color, *},
};

/// Provide text style parameters that include bold and italic font's if
/// available.
#[derive(Debug, Default, Clone)]
pub struct TextStyleParams {
    /// This is bevy's typical font style. Its font can be considered
    /// text_style's plain font.
    pub text_style: TextStyle,
    /// A bold font if available.
    pub bold: Option<Handle<Font>>,
    /// An italic font if available.
    pub italic: Option<Handle<Font>>,
    // underline
    // strikethrough
}

impl From<TextStyle> for TextStyleParams {
    fn from(text_style: TextStyle) -> TextStyleParams {
        TextStyleParams {
            text_style: text_style,
            ..default()
        }
    }
}

impl From<Color> for bevy_Color {
    fn from(c: Color) -> bevy_Color {
        match c {
            Color::Ansi { color, mode } => get_rgb_color(color, mode),
            Color::Rgb { r, g, b } => bevy_Color::rgb_u8(r, g, b),
        }
    }
}

fn use_params(params: &TextStyleParams, style: &Style) -> TextStyle {
    let font: Option<Handle<Font>> = if style.effects.is_bold {
        params.bold.clone()
    } else if style.effects.is_italic {
        params.italic.clone()
    } else {
        None
    };
    TextStyle {
        font: font.unwrap_or(params.text_style.font.clone()),
        ..params.text_style.clone()
    }
}

fn get_rgb_color(color: AnsiColor, mode: AnsiMode) -> bevy_Color {
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
    bevy_Color::rgb_u8(r, g, b)
}

fn with_style(s: StyledString, text_style_params: &TextStyleParams) -> TextBundle {
    let bundle = TextBundle::from_section(
        s.s,
        s.style
            .and_then(|style| {
                style.fg.map(|fg| TextStyle {
                    color: fg.into(),
                    ..use_params(text_style_params, &style)
                })
            })
            .unwrap_or(text_style_params.text_style.clone()),
    );
    let bg: Option<bevy_Color> = s.style.and_then(|style| style.bg.map(Into::into));
    match bg {
        None => bundle,
        Some(color) => bundle.with_background_color(color),
    }
}

impl From<StyledString> for TextBundle {
    fn from(s: StyledString) -> TextBundle {
        with_style(s, &TextStyle::default().into())
    }
}

// I originally had this function too:
//
// fn with_style_str<'a>(s: StyledStr<'a>, text_style_params: &TextStyleParams) -> TextBundle;
//
// but TextBundle requires a String, so Into<StyledString> seemed more explicit.
fn with_style_string(s: impl Into<StyledString>, text_style_params: &TextStyleParams) -> TextBundle {
    let s = s.into();
    let bundle = TextBundle::from_section(
        s.s,
        s.style
            .and_then(|style| {
                style.fg.map(|fg| TextStyle {
                    color: fg.into(),
                    ..use_params(text_style_params, &style)
                })
            })
            .unwrap_or(text_style_params.text_style.clone()),
    );
    let bg: Option<bevy_Color> = s.style.and_then(|style| style.bg.map(Into::into));
    match bg {
        None => bundle,
        Some(color) => bundle.with_background_color(color),
    }
}

impl<'a> From<StyledStr<'a>> for TextBundle {
    fn from(s: StyledStr<'a>) -> TextBundle {
        with_style_string(s, &TextStyle::default().into())
    }
}

/// Renders a styled string to the given output using `bevy`.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// fn setup(mut commands: Commands) {
///     let text = text_style::StyledStr::plain("test").bold();
///     commands
///         .spawn(NodeBundle::default())
///         .with_children(|parent| {
///             text_style::bevy::render(
///                 parent,
///                 &TextStyle {
///                     font_size: 50.0,
///                     ..default()
///                 }.into(),
///                 text
///             );
///         });
/// }
/// ```
pub fn render<'a>(
    parent: &mut ChildBuilder<'_, '_, '_>,
    o: &TextStyleParams,
    s: impl Into<StyledString>,
) {
    parent.spawn(with_style_string(s.into(), o));
}

/// Renders multiple styled string to the given output using `bevy`.
///
/// # Example
///
/// ```
/// # use ::bevy::prelude::*;
/// # use text_style::*;
/// fn setup(mut commands: Commands) {
///     commands
///         .spawn(NodeBundle::default())
///         .with_children(|parent| {
///             text_style::bevy::render_iter(
///                 parent,
///                 &(TextStyle {
///                     font_size: 50.0,
///                     ..default()
///                 }.into()),
///                 [
///                     StyledStr::plain("ansi red light").with(AnsiColor::Red.light()),
///                     " ".into(),
///                     StyledStr::plain("red").with(text_style::Color::Rgb { r: 255, g: 0, b: 0 }),
///                     " ".into(),
///                     StyledStr::plain("on green dark").on(AnsiColor::Green.dark()),
///                     " ".into(),
///                     StyledStr::plain("on green").on(text_style::Color::Rgb { r: 0, g: 255, b: 0 }),
///                 ],
///             );
///          });
/// }
/// ```
pub fn render_iter<'a, I, Iter, S>(
    parent: &mut ChildBuilder<'_, '_, '_>,
    o: &TextStyleParams,
    iter: I,
) where
    I: IntoIterator<Item = S, IntoIter = Iter>,
    Iter: Iterator<Item = S>,
    S: Into<StyledString>,
{
    iter.into_iter().for_each(|b| render(parent, o, b));
}
