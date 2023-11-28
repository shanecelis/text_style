use bevy::{self,
           prelude::{*, Color as bevy_Color},
};
use crate::{AnsiColor, AnsiMode, Color, Style, StyledStr, StyledString};

pub trait TextRender {
    fn render(&self, style: TextStyle) -> TextBundle;
}

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
        s.style.and_then(|style| style.fg.map(|fg| TextStyle { color: fg.into(), ..text_style.clone() }))
                .unwrap_or(text_style));
    let bg: Option<bevy_Color> = s.style.and_then(|style| style.bg.map(Into::into));
    match bg {
        None => bundle,
        Some(color) => bundle.with_background_color(color)
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
        s.style.and_then(|style| style.fg.map(|fg| TextStyle { color: fg.into(), ..text_style.clone() }))
                .unwrap_or(text_style));
    let bg: Option<bevy_Color> = s.style.and_then(|style| style.bg.map(Into::into));
    match bg {
        None => bundle,
        Some(color) => bundle.with_background_color(color)
    }
}

impl<'a> From<StyledStr<'a>> for TextBundle {
    fn from(s: StyledStr<'a>) -> TextBundle {
        with_style_str(s, TextStyle::default())
    }
}

pub fn render<'a>(parent: &mut ChildBuilder<'_, '_, '_>, o: Option<TextStyle>, s: impl Into<StyledStr<'a>>) {
    let mut bundle = with_style_str(s.into(), o.unwrap_or(TextStyle::default()));
    parent.spawn(bundle);
}

pub fn render_iter<'a, I, Iter, S>(parent: &mut ChildBuilder<'_, '_, '_>, o: Option<TextStyle>, iter: I)
where
    I: IntoIterator<Item = S, IntoIter = Iter>,
    Iter: Iterator<Item = S>,
    S: Into<StyledStr<'a>>,
{
    iter.into_iter()
        .for_each(|b| render(parent, o.clone(), b));
        // .map(Into::into)
        // .map(TextBundle::from)
        // .for_each(|b| { parent.spawn(b); });
}

// pub fn render(parent: &mut ChildBuilder<'_, '_, '_>, s: impl Into<StyledString>) {
//     let bundle: TextBundle = s.into().into();
//     parent.spawn(bundle);
// }

// pub fn render_iter<I, Iter, S, W>(parent: &mut ChildBuilder<'_, '_, '_>, iter: I)
// where
//     I: IntoIterator<Item = S, IntoIter = Iter>,
//     Iter: Iterator<Item = S>,
//     S: Into<StyledString>,
// {
//     iter.into_iter()
//         // .map(Into::into)
//         // .map(TextBundle::from)
//         .for_each(|b| render(parent, b));
// }

// impl From<Style> for TextBundle {
//     fn from(style: Style) -> TextBundle {

//         let color = convert(self.1.get_dyncolors_fg());
//         style.color = color;
//         let bundle = TextBundle::from_section(
//                 format!("{}", self), // Is there a better way?
//                 style);
//         bundle
//         ansi_term::Style {
//             foreground: style.fg.map(Into::into),
//             background: style.bg.map(Into::into),
//             is_bold: style.effects.is_bold,
//             is_italic: style.effects.is_italic,
//             is_underline: style.effects.is_underline,
//             is_strikethrough: style.effects.is_strikethrough,
//             ..Default::default()
//         }
//     }
// }

// fn convert(dyn_colors: DynColors) -> Color {
//     match dyn_colors {

//         DynColors::Rgb(r, g, b) => Color::rgb_u8(r, g, b),
//         DynColors::Ansi(ansi) =>
//             match ansi {
//                 AnsiColors::Black => Color::BLACK,
//                 AnsiColors::Red => Color::rgb_u8(204, 0, 0),
//                 AnsiColors::Green => Color::rgb_u8(78, 154, 6),
//                 AnsiColors::Yellow => Color::rgb_u8(196, 160, 0),
//                 AnsiColors::Blue => Color::rgb_u8(114, 159, 207),
//                 AnsiColors::Magenta => Color::rgb_u8(117, 80, 123),
//                 AnsiColors::Cyan => Color::rgb_u8(6, 152, 154),
//                 AnsiColors::White => Color::rgb_u8(211, 215, 207),
//                 AnsiColors::BrightBlack => Color::rgb_u8(85, 87, 83),
//                 AnsiColors::BrightRed => Color::rgb_u8(239, 41, 41),
//                 AnsiColors::BrightGreen => Color::rgb_u8(138, 226, 52),
//                 AnsiColors::BrightYellow => Color::rgb_u8(252, 233, 79),
//                 AnsiColors::BrightBlue => Color::rgb_u8(50, 175, 255),
//                 AnsiColors::BrightMagenta => Color::rgb_u8(173, 127, 168),
//                 AnsiColors::BrightCyan => Color::rgb_u8(52, 226, 226),
//                 AnsiColors::BrightWhite => Color::rgb_u8(255, 255, 255),
//                 AnsiColors::Default => todo!(),
//             },
//         DynColors::Css(_css) => todo!(),
//         DynColors::Xterm(_xterm) => todo!(),
//     }
// }

// impl<Color: owo_colors::Color, T: std::fmt::Display> TextRender for FgColorDisplay<'_, Color, T> {
//     fn render(&self, mut style: TextStyle) -> TextBundle {
//         let color = convert(Color::into_dyncolors());
//         style.color = color;
//         let bundle = TextBundle::from_section(
//                 format!("{}", self), // Is there a better way?
//                 style);
//         bundle
//     }
// }

// impl<Color: DynColor, T: std::fmt::Display> TextRender for FgDynColorDisplay<'_, Color, T> {
//     fn render(&self, mut style: TextStyle) -> TextBundle {
//         let color = convert(self.1.get_dyncolors_fg());
//         style.color = color;
//         let bundle = TextBundle::from_section(
//                 format!("{}", self), // Is there a better way?
//                 style);
//         bundle
//     }
// }

// impl<Color: DynColor, T: std::fmt::Display> TextRender for BgDynColorDisplay<'_, Color, T> {
//     fn render(&self, style: TextStyle) -> TextBundle {
//         let color = convert(self.1.get_dyncolors_bg());
//         let mut bundle = TextBundle::from_section(
//                 format!("{}", self),
//                 style);
//         bundle.background_color = color.into();
//         bundle
//     }
// }

// impl<Color: owo_colors::Color, T: std::fmt::Display> TextRender for BgColorDisplay<'_, Color, T> {
//     fn render(&self, style: TextStyle) -> TextBundle {
//         let color = convert(Color::into_dyncolors());
//         let mut bundle = TextBundle::from_section(
//                 format!("{}", self),
//                 style);
//         bundle.background_color = color.into();
//         bundle
//     }
// }

// impl<Fg: DynColor, Bg: DynColor, T: std::fmt::Display> TextRender for ComboDynColorDisplay<'_, Fg, Bg, T> {
//     fn render(&self, mut style: TextStyle) -> TextBundle {
//         let fg = convert(self.1.get_dyncolors_fg());
//         style.color = fg;
//         let bg = convert(self.2.get_dyncolors_bg());
//         let mut bundle = TextBundle::from_section(
//                 format!("{}", self),
//                 style);
//         bundle.background_color = bg.into();
//         bundle
//     }
// }

// impl<Fg: owo_colors::Color, Bg: owo_colors::Color, T: std::fmt::Display> TextRender for ComboColorDisplay<'_, Fg, Bg, T> {
//     fn render(&self, mut style: TextStyle) -> TextBundle {
//         let fg = convert(Fg::into_dyncolors());
//         style.color = fg;
//         let bg = convert(Bg::into_dyncolors());
//         let mut bundle = TextBundle::from_section(
//                 format!("{}", self),
//                 style);
//         bundle.background_color = bg.into();
//         bundle
//     }
// }

// impl TextRender for &str {
//     fn render(&self, style: TextStyle) -> TextBundle {
//         TextBundle::from_section(
//             self.to_owned(),
//             style)
//     }
// }

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // UI camera
//     commands.spawn(Camera2dBundle::default());
//     // Text with one section
//     commands.spawn((
//         // Create a TextBundle that has a Text with a single section.
//         TextBundle::from_section(
//             // Accepts a `String` or any type that converts into a `String`, such as `&str`
//             "hello\nbevy!",
//             TextStyle {
//                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                 font_size: 100.0,
//                 color: Color::WHITE,
//             },
//         ) // Set the alignment of the Text
//         .with_text_alignment(TextAlignment::Center)
//         // Set the style of the TextBundle itself.
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             bottom: Val::Px(5.0),
//             right: Val::Px(15.0),
//             ..default()
//         }),
//         ColorText,
//     ));
//     // Text with multiple sections
//     commands.spawn((
//         // Create a TextBundle that has a Text with a list of sections.
//         TextBundle::from_sections([
//             TextSection::new(
//                 "FPS: ",
//                 TextStyle {
//                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                     font_size: 60.0,
//                     color: Color::WHITE,
//                 },
//             ),
//             TextSection::from_style(TextStyle {
//                 font: asset_server.load("fonts/FiraMono-Medium.ttf"),
//                 font_size: 60.0,
//                 color: Color::GOLD,
//             }),
//         ]),
//         FpsText,
//     ));

//     let style = TextStyle {
//                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                     font_size: 60.0,
//                     color: Color::WHITE,
//                 };
//     commands.spawn(NodeBundle {
//         style: Style {
//             align_items: AlignItems::FlexEnd,
//             top: Val::Px(100.0),
//             width: Val::Px(100.0),
//             ..default()
//         },
//         ..default()
//     }).with_children(|parent| {
//         let a : [&dyn TextRender; 8]  = [
//             &"fg".color(DynColors::Rgb(255, 0, 0)),
//             &"fg".red(),
//             &" ",
//             &"bg".on_color(DynColors::Rgb(0, 255, 0)),
//             &"bg".on_green(),
//             &"  ",
//             &"fgbg".color(DynColors::Rgb(0,0,255)).on_color(DynColors::Rgb(255, 255, 255)),
//             &"fgbg".blue().on_white(),
//         ];
//         for c in do_renders(a.into_iter(), style) {
//             parent.spawn(c);
//         }
//     });
// }

// fn do_renders<'a>(items: impl Iterator<Item = &'a dyn TextRender> + 'a,
//                   style: TextStyle) -> impl Iterator<Item=TextBundle> + 'a {
//     return items.map(move |item| item.render(style.clone()))
// }

// fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
//     for mut text in &mut query {
//         let seconds = time.elapsed_seconds();

//         // Update the color of the first and only section.
//         text.sections[0].style.color = Color::Rgba {
//             red: (1.25 * seconds).sin() / 2.0 + 0.5,
//             green: (0.75 * seconds).sin() / 2.0 + 0.5,
//             blue: (0.50 * seconds).sin() / 2.0 + 0.5,
//             alpha: 1.0,
//         };
//     }
// }

// fn text_update_system(
//     diagnostics: Res<DiagnosticsStore>,
//     mut query: Query<&mut Text, With<FpsText>>) {
//     for mut text in &mut query {
//         if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
//             if let Some(value) = fps.smoothed() {
//                 // Update the value of the second section
//                 text.sections[1].value = format!("{value:.2}");
//             }
//         }
//     }
// }
