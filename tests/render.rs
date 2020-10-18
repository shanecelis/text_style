// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

fn render<F, E>(f: F) -> String
where
    F: Fn(&mut Vec<u8>) -> Result<(), E>,
    E: std::fmt::Debug,
{
    let mut v = Vec::new();
    f(&mut v).expect("Failed to render string");
    String::from_utf8(v).expect("Output is invalid UTF-8")
}

fn assert_output(expected: &[&str], actual: &str) {
    if !expected.contains(&actual) {
        let mut msg = "Unexpected output:\n  expected one of:\n".to_owned();
        for s in expected {
            msg.push_str(&format!("    {:?}\n", s));
        }
        msg.push_str("  found:\n");
        msg.push_str(&format!("    {:?}", actual));
        panic!(msg);
    }
}

macro_rules! test_cases {
    ([$input:expr; $output:expr] $( $( #[$attr:meta] )* $name:ident, )+) => {
        $(
            #[test]
            $( #[$attr] )*
            fn $name() {
                let input = $input;
                let expected_output = $output;
                let output = crate::render(|v| text_style::$name::render(v, &input));
                crate::assert_output(expected_output, &output);
            }
         )*
    };
}

mod bold {
    const OUTPUT: &[&'static str] = &["\x1b[1mtest\x1b[0m", "\x1b[1mtest\x1b[m"];

    fn input() -> text_style::StyledStr<'static> {
        text_style::StyledStr::plain("test").bold()
    }

    test_cases! { [input(); OUTPUT]
        ansi_term,
        crossterm,
        termion,
    }

    #[test]
    fn cursive() {
        use cursive::utils::markup;

        let input = input();
        let output = markup::StyledString::styled("test", cursive::theme::Effect::Bold);
        assert_eq!(output, markup::StyledString::from(input));
    }

    #[test]
    fn genpdf() {
        use genpdf::style;

        let input = input();
        let output = style::StyledStr::from(input.clone());
        assert_eq!(output.s, input.s);
        assert!(output.style.is_bold());
    }
}

mod italic {
    const OUTPUT: &[&'static str] = &["\x1b[3mtest\x1b[0m", "\x1b[3mtest\x1b[m"];

    fn input() -> text_style::StyledStr<'static> {
        text_style::StyledStr::plain("test").italic()
    }

    test_cases! { [input(); OUTPUT]
        ansi_term,
        crossterm,
        termion,
    }

    #[test]
    fn cursive() {
        use cursive::utils::markup;

        let input = input();
        let output = markup::StyledString::styled("test", cursive::theme::Effect::Italic);
        assert_eq!(output, markup::StyledString::from(input));
    }

    #[test]
    fn genpdf() {
        use genpdf::style;

        let input = input();
        let output = style::StyledStr::from(input.clone());
        assert_eq!(output.s, input.s);
        assert!(output.style.is_italic());
    }
}

mod underline {
    const OUTPUT: &[&'static str] = &["\x1b[4mtest\x1b[0m", "\x1b[4mtest\x1b[m"];

    fn input() -> text_style::StyledStr<'static> {
        text_style::StyledStr::plain("test").underline()
    }

    test_cases! { [input(); OUTPUT]
        ansi_term,
        crossterm,
        termion,
    }

    #[test]
    fn cursive() {
        use cursive::utils::markup;

        let input = input();
        let output = markup::StyledString::styled("test", cursive::theme::Effect::Underline);
        assert_eq!(output, markup::StyledString::from(input));
    }
}

mod strikethrough {
    const OUTPUT: &[&'static str] = &["\x1b[9mtest\x1b[0m", "\x1b[9mtest\x1b[m"];

    fn input() -> text_style::StyledStr<'static> {
        text_style::StyledStr::plain("test").strikethrough()
    }

    test_cases! { [input(); OUTPUT]
        ansi_term,
        crossterm,
        termion,
    }

    #[test]
    fn cursive() {
        use cursive::utils::markup;

        let input = input();
        let output = markup::StyledString::styled("test", cursive::theme::Effect::Strikethrough);
        assert_eq!(output, markup::StyledString::from(input));
    }
}

mod fg {
    const OUTPUT: &[&'static str] = &[
        "\x1b[31mtest\x1b[39m",
        "\x1b[31mtest\x1b[0m",
        "\x1b[31mtest\x1b[m",
        "\x1b[38;5;1mtest\x1b[39m",
        "\x1b[38;5;1mtest\x1b[0m",
        "\x1b[38;5;1mtest\x1b[m",
    ];

    fn input() -> text_style::StyledStr<'static> {
        text_style::StyledStr::plain("test").with(text_style::AnsiColor::Red.dark())
    }

    test_cases! { [input(); OUTPUT]
        ansi_term,
        crossterm,
        termion,
    }

    #[test]
    fn cursive() {
        use cursive::{theme, utils::markup};

        let input = input();
        let output = markup::StyledString::styled(
            "test",
            theme::ColorStyle::new(
                theme::Color::Dark(theme::BaseColor::Red),
                theme::Color::TerminalDefault,
            ),
        );
        assert_eq!(output, markup::StyledString::from(input));
    }

    #[test]
    fn genpdf() {
        use genpdf::style;

        let input = input();
        let output = style::StyledStr::from(input.clone());
        assert_eq!(output.s, input.s);
        assert_eq!(output.style.color(), Some(style::Color::Rgb(170, 0, 0)));
    }
}

mod bg {
    const OUTPUT: &[&'static str] = &[
        "\x1b[41mtest\x1b[49m",
        "\x1b[41mtest\x1b[0m",
        "\x1b[41mtest\x1b[m",
        "\x1b[48;5;1mtest\x1b[49m",
        "\x1b[48;5;1mtest\x1b[0m",
        "\x1b[48;5;1mtest\x1b[m",
    ];

    fn input() -> text_style::StyledStr<'static> {
        text_style::StyledStr::plain("test").on(text_style::AnsiColor::Red.dark())
    }

    test_cases! { [input(); OUTPUT]
        ansi_term,
        crossterm,
        termion,
    }

    #[test]
    fn cursive() {
        use cursive::{theme, utils::markup};

        let input = input();
        let output = markup::StyledString::styled(
            "test",
            theme::ColorStyle::new(
                theme::Color::TerminalDefault,
                theme::Color::Dark(theme::BaseColor::Red),
            ),
        );
        assert_eq!(output, markup::StyledString::from(input));
    }
}
