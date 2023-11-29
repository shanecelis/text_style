use crate::{AnsiColor, AnsiMode, Color, Style, StyledStr, StyledString};
use bevy::{
    self,
    prelude::{Color as bevy_Color, *},
};

impl From<Color> for bevy_Color {
    fn from(c: Color) -> bevy_Color {
        match c {
            Color::Ansi { color, mode } => get_rgb_color(color, mode),
            Color::Rgb { r, g, b } => bevy_Color::rgb_u8(r, g, b),
        }
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

fn with_style(s: StyledString, text_style: TextStyle) -> TextBundle {
    let bundle = TextBundle::from_section(
        s.s,
        s.style
            .and_then(|style| {
                style.fg.map(|fg| TextStyle {
                    color: fg.into(),
                    ..text_style.clone()
                })
            })
            .unwrap_or(text_style),
    );
    let bg: Option<bevy_Color> = s.style.and_then(|style| style.bg.map(Into::into));
    match bg {
        None => bundle,
        Some(color) => bundle.with_background_color(color),
    }
}

impl From<StyledString> for TextBundle {
    fn from(s: StyledString) -> TextBundle {
        with_style(s, TextStyle::default())
    }
}

fn with_style_str<'a>(s: StyledStr<'a>, text_style: TextStyle) -> TextBundle {
    let bundle = TextBundle::from_section(
        s.s.to_owned(),
        s.style
            .and_then(|style| {
                style.fg.map(|fg| TextStyle {
                    color: fg.into(),
                    ..text_style.clone()
                })
            })
            .unwrap_or(text_style),
    );
    let bg: Option<bevy_Color> = s.style.and_then(|style| style.bg.map(Into::into));
    match bg {
        None => bundle,
        Some(color) => bundle.with_background_color(color),
    }
}

impl<'a> From<StyledStr<'a>> for TextBundle {
    fn from(s: StyledStr<'a>) -> TextBundle {
        with_style_str(s, TextStyle::default())
    }
}

pub fn render<'a>(
    parent: &mut ChildBuilder<'_, '_, '_>,
    o: Option<TextStyle>,
    s: impl Into<StyledStr<'a>>,
) {
    let mut bundle = with_style_str(s.into(), o.unwrap_or(TextStyle::default()));
    parent.spawn(bundle);
}

pub fn render_iter<'a, I, Iter, S>(
    parent: &mut ChildBuilder<'_, '_, '_>,
    o: Option<TextStyle>,
    iter: I,
) where
    I: IntoIterator<Item = S, IntoIter = Iter>,
    Iter: Iterator<Item = S>,
    S: Into<StyledStr<'a>>,
{
    iter.into_iter().for_each(|b| render(parent, o.clone(), b));
    // .map(Into::into)
    // .map(TextBundle::from)
    // .for_each(|b| { parent.spawn(b); });
}
