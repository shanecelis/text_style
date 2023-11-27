// Copyright (C) 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: CC0-1.0

//! This example uses `syntect` to syntac highlight a file and then uses one of the `text_style`
//! backends to render the highlighted file.
//!
//! To run this example, you have to activate all features using the `--all-features` option.  For
//! example, to highlight this file with the `termion` backend:
//!
//! ```
//! $ cargo run --example syntect --all-features examples/syntect.rs termion
//! ```

use std::fs;
use std::io;

use argh::FromArgs;
use syntect::{easy, highlighting, parsing, util};

/// Highlight a file and print it using the given method.
#[derive(FromArgs)]
struct Args {
    /// the name of the input file.
    #[argh(positional)]
    input: String,

    /// the output method (debug, ansi_term, crossterm, cursive, termion).
    #[argh(positional)]
    backend: String,
}

fn render<'a, 's, I>(backend: &str, strings: I)
where
    's: 'a,
    I: Iterator<Item = &'a text_style::StyledStr<'s>>,
{
    match backend {
        #[cfg(feature = "ansi_term")]
        "ansi_term" => {
            text_style::ansi_term::render_iter(io::stdout(), strings)
                .expect("ansi_term rendering failed");
        }
        #[cfg(feature = "crossterm")]
        "crossterm" => {
            text_style::crossterm::render_iter(io::stdout(), strings)
                .expect("crossterm rendering failed");
        }
        #[cfg(feature = "cursive")]
        "cursive" => {
            use cursive::view::Scrollable as _;

            let mut s = cursive::default();
            let mut view = cursive::views::TextView::new("");
            for s in strings {
                view.append(s);
            }
            s.add_layer(view.scrollable());
            s.add_global_callback('q', |s| s.quit());
            s.run();
        }
        #[cfg(feature = "termion")]
        "termion" => {
            text_style::termion::render_iter(io::stdout(), strings)
                .expect("termion rendering failed");
        }
        "debug" => {
            for s in strings {
                println!("{:?}", s);
            }
        }
        _ => {
            panic!("Unsupported backend {}", backend);
        }
    }
}

fn main() {
    let ps = parsing::SyntaxSet::load_defaults_newlines();
    let ts = highlighting::ThemeSet::load_defaults();

    let args: Args = argh::from_env();

    let syntax = ps
        .find_syntax_for_file(&args.input)
        .expect("Could not read input file")
        .unwrap_or_else(|| ps.find_syntax_plain_text());
    let mut h = easy::HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let s = fs::read_to_string(&args.input).expect("Could not read input file");
    let mut lines = Vec::new();
    for line in util::LinesWithEndings::from(&s) {
        let ranges = h.highlight(line, &ps);
        let styled_strs: Vec<text_style::StyledStr<'_>> =
            ranges.into_iter().map(Into::into).collect();
        lines.push(styled_strs);
    }

    render(&args.backend, lines.iter().flatten());
}
