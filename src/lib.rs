// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! The `text_style` crate provides types and conversions for styled text.
//!
//! # Overview
//!
//! The central types of this crate are [`StyledStr`][] and [`StyledString`][]:  owned and borrowed
//! strings that are annotated with an optional style information, [`Style`][].  This style
//! information consists of foreground and background colors ([`Color`][]) and multiple effects
//! ([`Effect`][]: bold, italic, underline or strikeout).
//!
//! `text_style`’s types can be created directly or converted from or to several formats (all
//! optional and activated by features):
//!
//! - [`ansi_term`][]: convert to [`ansi_term::ANSIString`][]
//! - [`crossterm`][]: convert to [`crossterm::style::StyledContent`][]
//! - [`cursive`][]: convert to [`cursive::utils::markup::StyledString`][]
//! - [`genpdf`][]: convert to [`genpdf::style::StyledStr`][] and [`genpdf::style::StyledString`][]
//! - [`syntect`][]: convert from [`syntect::highlighting::Style`][]
//! - [`termion`][]: convert to a termion escape string
//! - [`colored`][]: convert to a [`colored::ColoredString`][]
//! - [`bevy`][]: convert to a bevy [`bevy::prelude::TextBundle`][]
//!
//! # Background
//!
//! There is a plethora of crates that produce or consume styled text.  Most of these crates use
//! very similar types: ANSI and RGB colors, text effects, styled strings.  But converting between
//! these types requires a lot of boilerplate code.  The goal of this crate is to provide a subset
//! of common style types to enable conversion between the different crates.
//!
//! # Usage
//!
//! ## Creating styled text
//!
//! The [`StyledString`][] and [`StyledStr`][] structs provide many utility methods for creating
//! styled strings easily:
//!
//! ```
//! use text_style::{AnsiColor, Effect, StyledStr};
//! let s = StyledStr::plain("text")
//!     .with(AnsiColor::Red.light())
//!     .on(AnsiColor::Green.dark())
//!     .bold();
//! # #[cfg(feature = "ansi_term")]
//! text_style::ansi_term::render(std::io::stdout(), s)
//!     .expect("Could not render line");
//! ```
//!
//! If the `syntect` feature is activated, conversion traits from `syntect`’s style types are
//! implemented:
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
//!     # #[cfg(feature = "ansi_term")]
//!     text_style::ansi_term::render_iter(std::io::stdout(), ranges.iter())
//!         .expect("Could not render line");
//! }
//! ```
//!
//! ## Rendering styled text
//!
//! The backends define conversion traits from or to the `text_style` types where applicable.
//!
//! Most backends also define `render` and `render_iter` methods to display a styled string or an
//! iterator over styled strings:
//!
//! ```
//! let s = text_style::StyledStr::plain("test").bold();
//!
//! let mut w = std::io::stdout();
//! # #[cfg(feature = "ansi_term")]
//! text_style::ansi_term::render(&mut w, &s).expect("Rendering failed");
//! # #[cfg(feature = "crossterm")]
//! text_style::crossterm::render(&mut w, &s).expect("Rendering failed");
//! # #[cfg(feature = "termion")]
//! text_style::termion::render(&mut w, &s).expect("Rendering failed");
//! ```
//!
//! For more information, see the module documentations.
//!
//! [`Color`]: enum.Color.html
//! [`Effect`]: enum.Effect.html
//! [`Style`]: struct.Style.html
//! [`StyledStr`]: struct.StyledStr.html
//! [`StyledString`]: struct.StyledString.html
//! [`ansi_term`]: ./ansi_term/index.html
//! [`crossterm`]: ./crossterm/index.html
//! [`cursive`]: ./cursive/index.html
//! [`genpdf`]: ./genpdf/index.html
//! [`syntect`]: ./syntect/index.html
//! [`termion`]: ./termion/index.html
//! [`bevy`]: ./bevy/index.html
//! [`colored`]: ./colored/index.html
//! [`ansi_term::ANSIString`]: https://docs.rs/ansi_term/latest/ansi_term/type.ANSIString.html
//! [`crossterm::style::StyledContent`]: https://docs.rs/crossterm/latest/crossterm/style/struct.StyledContent.html
//! [`cursive::utils::markup::StyledString`]: https://docs.rs/cursive/latest/cursive/utils/markup/type.StyledString.html
//! [`genpdf::style::StyledStr`]: https://docs.rs/genpdf/latest/genpdf/style/struct.StyledStr.html
//! [`genpdf::style::StyledString`]: https://docs.rs/genpdf/latest/genpdf/style/struct.StyledString.html
//! [`syntect::highlighting::Style`]: https://docs.rs/syntect/latest/syntect/highlighting/struct.Style.html
//! [`bevy::prelude::TextBundle`]: https://docs.rs/bevy/latest/bevy/prelude/struct.TextBundle.html
//! [`colored::ColoredString`]: https://docs.rs/colored/latest/colored/struct.ColoredString.html

#![warn(missing_docs, rust_2018_idioms)]

#[cfg(feature = "ansi_term")]
pub mod ansi_term;
#[cfg(feature = "bevy")]
pub mod bevy;
#[cfg(feature = "colored")]
pub mod colored;
#[cfg(feature = "crossterm")]
pub mod crossterm;
#[cfg(feature = "cursive")]
pub mod cursive;
#[cfg(feature = "genpdf")]
pub mod genpdf;
#[cfg(feature = "syntect")]
pub mod syntect;
#[cfg(feature = "termion")]
pub mod termion;
/// A borrowed string with an optional style annotation.
///
/// # Example
///
/// ```
/// let s = text_style::StyledStr::plain("test").bold();
///
/// let s1 = text_style::StyledStr::styled("test", text_style::Style::fg(text_style::AnsiColor::Red.dark()));
/// let s2 = text_style::StyledStr::plain("test").with(text_style::AnsiColor::Red.dark());
/// assert_eq!(s1, s2);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct StyledStr<'a> {
    /// The content of this string.
    pub s: &'a str,
    /// The style of this string.
    pub style: Option<Style>,
}

/// An owned string with an optional style annotation.
///
/// # Example
///
/// ```
/// let s = format!("Some number: {}", 42);
///
/// let s0 = text_style::StyledString::plain(s.clone()).bold();
///
/// let s1 = text_style::StyledString::styled(s.clone(), text_style::Style::fg(text_style::AnsiColor::Red.dark()));
/// let s2 = text_style::StyledString::plain(s.clone()).with(text_style::AnsiColor::Red.dark());
/// assert_eq!(s1, s2);
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyledString {
    /// The content of this string.
    pub s: String,
    /// The style of this string.
    pub style: Option<Style>,
}

/// A text style, a combination of a foreground color, a background color and text effects (all
/// optional).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Style {
    /// The foreground color (if set).
    pub fg: Option<Color>,
    /// The background color (if set).
    pub bg: Option<Color>,
    /// The text effects.
    pub effects: Effects,
}

/// A text effect.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Effect {
    /// Bold text.
    Bold,
    /// Italic text.
    Italic,
    /// Underlined text.
    Underline,
    /// Struckthrough text.
    Strikethrough,
}

/// All available text effects.
pub const EFFECTS: &[Effect] = &[
    Effect::Bold,
    Effect::Italic,
    Effect::Underline,
    Effect::Strikethrough,
];

/// A set of text effects.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Effects {
    /// Whether the bold text effect is set.
    pub is_bold: bool,
    /// Whether the italic text effect is set.
    pub is_italic: bool,
    /// Whether the underline text effect is set.
    pub is_underline: bool,
    /// Whether the strikethrough text effect is set.
    pub is_strikethrough: bool,
}

/// An iterator over text effects.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EffectsIter {
    effects: Effects,
    i: usize,
}

/// A color.
///
/// This enum stores colors, either as an ANSI color (see [`AnsiColor`][] and [`AnsiMode`][]) or as
/// an RGB color.
///
/// [`AnsiColor`]: enum.AnsiColor.html
/// [`AnsiMode`]: enum.AnsiMode.html
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    /// An ANSI color.
    Ansi {
        /// The ANSI base color.
        color: AnsiColor,
        /// The variant of the ANSI base color (light or dark).
        mode: AnsiMode,
    },
    /// An RGB color.
    Rgb {
        /// The red component.
        r: u8,
        /// The green component.
        g: u8,
        /// THe blue component.
        b: u8,
    },
}

/// An ANSI base color.
///
/// This enum contains the basic eight ANSI colors.  These colors are available in two modes:
/// [`Dark`][] and [`Light`][].  Combinations of an ANSI color and a mode are stored in the
/// [`Color`][] enum.
///
/// [`Color`]: enum.Color.html
/// [`Dark`]: enum.AnsiMode.html#variant.Dark
/// [`Light`]: enum.AnsiMode.html#variant.Light
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnsiColor {
    /// Black (ANSI color #0 (dark) or #8 (light)).
    Black,
    /// Red (ANSI color #1 (dark) or #9 (light)).
    Red,
    /// Green (ANSI color #2 (dark) or #10 (light)).
    Green,
    /// Yellow (ANSI color #3 (dark) or #11 (light)).
    Yellow,
    /// Blue (ANSI color #4 (dark) or #12 (light)).
    Blue,
    /// Magenta (ANSI color #5 (dark) or #13 (light)).
    Magenta,
    /// Cyan (ANSI color #6 (dark) or #14 (light)).
    Cyan,
    /// White (ANSI color #7 (dark) or #15 (light)).
    White,
}

/// An ANSI color mode.
///
/// The ANSI base colors, stored in the [`AnsiColor`][] enum, are available in two modes:
/// [`Dark`][] and [`Light`][].  Combinations of an ANIS color and a mode are stored in the
/// [`Color`][] enum.
///
/// [`AnsiColor`]: enum.AnsiColor.html
/// [`Color`]: enum.Color.html
/// [`Dark`]: #variant.Dark
/// [`Light`]: #variant.Light
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnsiMode {
    /// The dark variant of an ANSI color.
    Dark,
    /// The light variant of an ANSI color.
    Light,
}

impl<'a> StyledStr<'a> {
    /// Creates a new styled string from the given string and an optional style.
    pub fn new(s: &'a str, style: Option<Style>) -> StyledStr<'a> {
        StyledStr { s, style }
    }

    /// Creates a new styled string from the given string and style.
    pub fn styled(s: &'a str, style: Style) -> StyledStr<'a> {
        StyledStr::new(s, Some(style))
    }

    /// Creates a new styled string from the given string without a style.
    pub fn plain(s: &'a str) -> StyledStr<'a> {
        StyledStr::new(s, None)
    }

    /// Sets the foreground color for this styled string.
    pub fn with(mut self, fg: Color) -> Self {
        self.style_mut().fg = Some(fg);
        self
    }

    /// Sets the background color for this styled string.
    pub fn on(mut self, bg: Color) -> Self {
        self.style_mut().bg = Some(bg);
        self
    }

    /// Sets the bold effect for this styled string.
    pub fn bold(self) -> Self {
        self.effect(Effect::Bold)
    }

    /// Sets the italic effect for this styled string.
    pub fn italic(self) -> Self {
        self.effect(Effect::Italic)
    }

    /// Sets the underline effect for this styled string.
    pub fn underline(self) -> Self {
        self.effect(Effect::Underline)
    }

    /// Sets the strikethrough effect for this styled string.
    pub fn strikethrough(self) -> Self {
        self.effect(Effect::Strikethrough)
    }

    /// Sets the given effect for this styled string.
    pub fn effect(mut self, effect: Effect) -> Self {
        self.style_mut().effects.set(effect, true);
        self
    }

    /// Returns a mutable reference to the style of this string, creating a new style with the
    /// default settings if the style is currently `None`.
    pub fn style_mut(&mut self) -> &mut Style {
        self.style.get_or_insert_with(Default::default)
    }
}

impl StyledString {
    /// Creates a new styled string from the given string and an optional style.
    pub fn new(s: String, style: Option<Style>) -> StyledString {
        StyledString { s, style }
    }

    /// Creates a new styled string from the given string and style.
    pub fn styled(s: String, style: Style) -> StyledString {
        StyledString::new(s, Some(style))
    }

    /// Creates a new styled string from the given string and style.
    pub fn plain(s: String) -> StyledString {
        StyledString::new(s, None)
    }

    /// Sets the foreground color for this styled string.
    pub fn with(mut self, fg: Color) -> Self {
        self.style_mut().fg = Some(fg);
        self
    }

    /// Sets the background color for this styled string.
    pub fn on(mut self, bg: Color) -> Self {
        self.style_mut().bg = Some(bg);
        self
    }

    /// Sets the bold effect for this styled string.
    pub fn bold(self) -> Self {
        self.effect(Effect::Bold)
    }

    /// Sets the italic effect for this styled string.
    pub fn italic(self) -> Self {
        self.effect(Effect::Italic)
    }

    /// Sets the underline effect for this styled string.
    pub fn underline(self) -> Self {
        self.effect(Effect::Underline)
    }

    /// Sets the strikethrough effect for this styled string.
    pub fn strikethrough(self) -> Self {
        self.effect(Effect::Strikethrough)
    }

    /// Sets the given effect for this styled string.
    pub fn effect(mut self, effect: Effect) -> Self {
        self.style_mut().effects.set(effect, true);
        self
    }

    /// Returns a mutable reference to the style of this string, creating a new style with the
    /// default settings if the style is currently `None`.
    pub fn style_mut(&mut self) -> &mut Style {
        self.style.get_or_insert_with(Default::default)
    }
}

impl<'a, 'b> From<&'b StyledStr<'a>> for StyledStr<'a> {
    fn from(s: &'b StyledStr<'a>) -> StyledStr<'a> {
        StyledStr {
            s: s.s,
            style: s.style,
        }
    }
}

impl<'a> From<&'a StyledString> for StyledStr<'a> {
    fn from(s: &'a StyledString) -> StyledStr<'a> {
        StyledStr {
            s: &s.s,
            style: s.style,
        }
    }
}

impl<'a> From<StyledStr<'a>> for StyledString {
    fn from(s: StyledStr<'a>) -> StyledString {
        StyledString {
            s: s.s.to_owned(),
            style: s.style,
        }
    }
}

impl<'a> From<&'a str> for StyledStr<'a> {
    fn from(s: &'a str) -> StyledStr<'a> {
        StyledStr::plain(s)
    }
}

impl From<String> for StyledString {
    fn from(s: String) -> StyledString {
        StyledString::plain(s)
    }
}

impl Style {
    /// Creates a new style with the given foreground and background colors and effects.
    pub fn new(fg: Option<Color>, bg: Option<Color>, effects: Effects) -> Style {
        Style { fg, bg, effects }
    }

    /// Creates a new style with the given foreground color.
    pub fn fg(color: Color) -> Style {
        Style::new(Some(color), None, Effects::new())
    }

    /// Creates a new style with the given background color.
    pub fn bg(color: Color) -> Style {
        Style::new(None, Some(color), Effects::new())
    }

    /// Creates a new style with the given text effect.
    pub fn effect(effect: Effect) -> Style {
        Style::new(None, None, Effects::only(effect))
    }

    /// Creates a new style with the given text effects.
    pub fn effects(effects: Effects) -> Style {
        Style::new(None, None, effects)
    }

    /// Combines this style with another style.
    ///
    /// If a color is set by both styles, the current color is overwritten.
    ///
    /// # Example
    ///
    /// ```
    /// use text_style::{AnsiColor, Effects, Style};
    ///
    /// assert_eq!(
    ///     Style::fg(AnsiColor::Red.dark()).and(Style::bg(AnsiColor::White.dark())),
    ///     Style::new(Some(AnsiColor::Red.dark()), Some(AnsiColor::White.dark()), Effects::empty()),
    /// );
    /// ```
    pub fn and(mut self, style: Style) -> Style {
        if let Some(fg) = style.fg {
            self.fg = Some(fg);
        }
        if let Some(bg) = style.bg {
            self.bg = Some(bg);
        }
        self.effects = self.effects.and(style.effects);
        self
    }

    /// Sets the foreground color of this style.
    pub fn set_fg(&mut self, color: Color) {
        self.fg = Some(color);
    }

    /// Sets the background color of this style.
    pub fn set_bg(&mut self, color: Color) {
        self.bg = Some(color);
    }

    /// Sets or unsets the bold effect for this style.
    pub fn set_bold(&mut self, bold: bool) {
        self.effects.is_bold = bold;
    }

    /// Sets or unsets the italic effect for this style.
    pub fn set_italic(&mut self, italic: bool) {
        self.effects.is_italic = italic;
    }

    /// Sets or unsets the underline effect for this style.
    pub fn set_underline(&mut self, underline: bool) {
        self.effects.is_underline = underline;
    }

    /// Sets or unsets the strikethrough effect for this style.
    pub fn strikethrough(&mut self, strikethrough: bool) {
        self.effects.is_strikethrough = strikethrough;
    }

    /// Sets or unsets the given effect for this style.
    pub fn set_effect(&mut self, effect: Effect, set: bool) {
        self.effects.set(effect, set);
    }
}

impl From<Effect> for Style {
    fn from(effect: Effect) -> Style {
        Style::effect(effect)
    }
}

impl From<Effects> for Style {
    fn from(effects: Effects) -> Style {
        Style::effects(effects)
    }
}

impl Effects {
    /// Creates an empty set of text effects.
    pub fn new() -> Effects {
        Default::default()
    }

    /// Creates an empty set of text effects.
    pub fn empty() -> Effects {
        Effects::new()
    }

    /// Creates a set of text effects with only the given effect.
    pub fn only(effect: Effect) -> Effects {
        Effects::from(effect)
    }

    /// Sets or unsets the given text effect.
    pub fn set(&mut self, effect: Effect, set: bool) {
        match effect {
            Effect::Bold => self.is_bold = set,
            Effect::Italic => self.is_italic = set,
            Effect::Underline => self.is_underline = set,
            Effect::Strikethrough => self.is_strikethrough = set,
        }
    }

    /// Checks whether the given effect is set.
    pub fn is_set(&self, effect: Effect) -> bool {
        match effect {
            Effect::Bold => self.is_bold,
            Effect::Italic => self.is_italic,
            Effect::Underline => self.is_underline,
            Effect::Strikethrough => self.is_strikethrough,
        }
    }

    /// Combines this set of text effects with another set of text effects.
    pub fn and(&self, other: Effects) -> Effects {
        Effects {
            is_bold: self.is_bold || other.is_bold,
            is_italic: self.is_italic || other.is_italic,
            is_underline: self.is_underline || other.is_underline,
            is_strikethrough: self.is_strikethrough || other.is_strikethrough,
        }
    }

    /// Checks whether this set of text effects is empty.
    pub fn is_empty(&self) -> bool {
        !self.is_bold && !self.is_italic && !self.is_underline && !self.is_strikethrough
    }
}

impl std::iter::FromIterator<Effect> for Effects {
    fn from_iter<I: IntoIterator<Item = Effect>>(iter: I) -> Effects {
        let mut effects = Effects::new();
        for effect in iter {
            effects.set(effect, true);
        }
        effects
    }
}

impl IntoIterator for Effects {
    type Item = Effect;
    type IntoIter = EffectsIter;

    fn into_iter(self) -> EffectsIter {
        EffectsIter::from(self)
    }
}

impl From<Effect> for Effects {
    fn from(effect: Effect) -> Effects {
        let mut effects = Effects::new();
        effects.set(effect, true);
        effects
    }
}

impl Iterator for EffectsIter {
    type Item = Effect;

    fn next(&mut self) -> Option<Effect> {
        let mut next = None;
        while let Some(effect) = EFFECTS.get(self.i) {
            self.i += 1;
            if self.effects.is_set(*effect) {
                next = Some(*effect);
                break;
            }
        }
        next
    }
}

impl From<Effects> for EffectsIter {
    fn from(effects: Effects) -> EffectsIter {
        EffectsIter { effects, i: 0 }
    }
}

impl AnsiColor {
    /// Returns the dark variant of this ANSI color.
    pub fn dark(self) -> Color {
        Color::Ansi {
            color: self,
            mode: AnsiMode::Dark,
        }
    }

    /// Returns the light variant of this ANSI color.
    pub fn light(self) -> Color {
        Color::Ansi {
            color: self,
            mode: AnsiMode::Light,
        }
    }
}
