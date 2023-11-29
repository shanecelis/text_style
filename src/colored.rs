use std::io;
use colored::{self, Colorize};

use crate::{Color, Effects, Style, StyledStr, StyledString, AnsiMode, AnsiColor};

struct PubColoredString {
    input: String,
    fgcolor: Option<Color>,
    bgcolor: Option<Color>,
    style: colored::Style,
}

impl From<colored::Color> for Color {
    fn from(color: colored::Color) -> Color {
        use ::colored::Color::*;
        use crate::AnsiColor;
        use crate::AnsiMode::*;
        match color {
            Black => Color::Ansi { color: AnsiColor::Black, mode: Dark },
            Red => Color::Ansi { color: AnsiColor::Red, mode: Dark },
            Green => Color::Ansi { color: AnsiColor::Green, mode: Dark },
            Yellow => Color::Ansi { color: AnsiColor::Yellow, mode: Dark },
            Blue => Color::Ansi { color: AnsiColor::Blue, mode: Dark },
            Magenta => Color::Ansi { color: AnsiColor::Magenta, mode: Dark },
            Cyan => Color::Ansi { color: AnsiColor::Cyan, mode: Dark },
            White => Color::Ansi { color: AnsiColor::White, mode: Dark },
            BrightBlack => Color::Ansi { color: AnsiColor::Black, mode: Light },
            BrightRed => Color::Ansi { color: AnsiColor::Red, mode: Light },
            BrightGreen => Color::Ansi { color: AnsiColor::Green, mode: Light },
            BrightYellow => Color::Ansi { color: AnsiColor::Yellow, mode: Light },
            BrightBlue => Color::Ansi { color: AnsiColor::Blue, mode: Light },
            BrightMagenta => Color::Ansi { color: AnsiColor::Magenta, mode: Light },
            BrightCyan => Color::Ansi { color: AnsiColor::Cyan, mode: Light },
            BrightWhite => Color::Ansi { color: AnsiColor::White, mode: Light },
            TrueColor { r, g, b } => Color::Rgb { r, g, b },
        }
    }
}

impl From<Color> for colored::Color {
    fn from(color: Color) -> colored::Color {
        match color {
            Color::Ansi { color, mode } => get_ansi(color, mode),
            Color::Rgb { r, g, b } => colored::Color::TrueColor { r, g, b },
        }
    }
}

fn get_ansi(color: AnsiColor, mode: AnsiMode) -> colored::Color {
    use ::colored::Color::*;
    use AnsiColor;
    use AnsiMode::*;

    match (mode, color) {
        (Dark, AnsiColor::Black)    => Black,
        (Dark, AnsiColor::Red)      => Red,
        (Dark, AnsiColor::Green)    => Green,
        (Dark, AnsiColor::Yellow)   => Yellow,
        (Dark, AnsiColor::Blue)     => Blue,
        (Dark, AnsiColor::Magenta)  => Magenta,
        (Dark, AnsiColor::Cyan)     => Cyan,
        (Dark, AnsiColor::White)    => White,
        (Light, AnsiColor::Black)   => BrightBlack,
        (Light, AnsiColor::Red)     => BrightRed,
        (Light, AnsiColor::Green)   => BrightGreen,
        (Light, AnsiColor::Yellow)  => BrightYellow,
        (Light, AnsiColor::Blue)    => BrightBlue,
        (Light, AnsiColor::Magenta) => BrightMagenta,
        (Light, AnsiColor::Cyan)    => BrightCyan,
        (Light, AnsiColor::White)   => BrightWhite,
    }
}

impl From<colored::Style> for Effects {
    fn from(style: colored::Style) -> Effects {
        Effects {
            is_bold: style.contains(colored::Styles::Bold),
            is_italic: style.contains(colored::Styles::Italic),
            is_underline: style.contains(colored::Styles::Underline),
            is_strikethrough: style.contains(colored::Styles::Strikethrough),
        }
    }
}

fn apply(effects: &Effects, mut string: colored::ColoredString) -> colored::ColoredString {
    if effects.is_bold {
        string = string.bold();
    }
    if effects.is_italic {
        string = string.italic();
    }
    if effects.is_underline {
        string = string.underline();
    }
    if effects.is_strikethrough {
        string = string.strikethrough();
    }
    string
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

impl From<colored::ColoredString> for StyledString {
    fn from(style: colored::ColoredString) -> StyledString {
        let pstyle: PubColoredString = unsafe { std::mem::transmute(style) };
        StyledString {
            s: pstyle.input,
            style: Some(Style {
                fg: pstyle.fgcolor.map(Into::into),
                bg: pstyle.bgcolor.map(Into::into),
                effects: pstyle.style.into(),
            })
        }
    }
}

impl From<StyledString> for colored::ColoredString {
    fn from(style_string: StyledString) -> colored::ColoredString {
        // Too bad there's to From<String> for ColoredString.
        // let s: colored::ColoredString = style_string.s.into();
        let mut s: colored::ColoredString = style_string.s.as_str().into();
        if let Some(style) = style_string.style {
            if let Some(fg) = style.fg {
                s = s.color(fg);
            }
            if let Some(bg) = style.bg {
                s = s.color(bg);
            }
            s = apply(&style.effects, s);
        }
        s
    }
}

impl<'a> From<StyledStr<'a>> for colored::ColoredString {
    fn from(style_string: StyledStr<'a>) -> colored::ColoredString {
        // Too bad there's to From<String> for ColoredString.
        // let s: colored::ColoredString = style_string.s.into();
        let mut s: colored::ColoredString = style_string.s.into();
        if let Some(style) = style_string.style {
            if let Some(fg) = style.fg {
                s = s.color(fg);
            }
            if let Some(bg) = style.bg {
                s = s.on_color(bg);
            }
            s = apply(&style.effects, s);
        }
        s
    }
}

pub fn render<'a>(mut w: impl io::Write, s: impl Into<StyledStr<'a>>) -> io::Result<()> {
    write!(w, "{}", colored::ColoredString::from(s.into()))
}

pub fn render_iter<'a, I, Iter, S, W>(mut w: W, iter: I) -> io::Result<()>
where
    I: IntoIterator<Item = S, IntoIter = Iter>,
    Iter: Iterator<Item = S>,
    S: Into<StyledStr<'a>>,
    W: io::Write,
{
    for s in iter
        .into_iter()
        .map(Into::into)
        .map(colored::ColoredString::from) {
        write!(w, "{}", s)?;
    }
    Ok(())
}

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
