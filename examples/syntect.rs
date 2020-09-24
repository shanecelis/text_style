// Copyright (C) 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: CC0-1.0

use std::collections;
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

    /// the output method (debug, crossterm).
    #[argh(positional)]
    render_method: String,
}

type RenderMethod = Box<dyn Fn(&[Vec<text_style::StyledStr<'_>>])>;

#[cfg(feature = "ansi_term")]
fn render_ansi_term(lines: &[Vec<text_style::StyledStr<'_>>]) {
    text_style::ansi_term::render_iter(io::stdout(), lines.iter().flatten())
        .expect("ansi_term rendering failed");
}

#[cfg(feature = "crossterm")]
fn render_crossterm(lines: &[Vec<text_style::StyledStr<'_>>]) {
    text_style::crossterm::render_iter(io::stdout(), lines.iter().flatten())
        .expect("crossterm rendering failed");
}

#[cfg(feature = "cursive")]
fn render_cursive(lines: &[Vec<text_style::StyledStr<'_>>]) {
    use cursive::view::Scrollable;

    let mut s = cursive::default();
    let mut view = cursive::views::TextView::new("");
    for s in lines.iter().flatten() {
        view.append(s);
    }
    s.add_layer(view.scrollable());
    s.add_global_callback('q', |s| s.quit());
    s.run();
}

#[cfg(feature = "termion")]
fn render_termion(lines: &[Vec<text_style::StyledStr<'_>>]) {
    text_style::termion::render_iter(io::stdout(), lines.iter().flatten())
        .expect("termion rendering failed");
}

fn render_debug(lines: &[Vec<text_style::StyledStr<'_>>]) {
    for line in lines {
        println!("{:?}", line);
    }
}

fn main() {
    let ps = parsing::SyntaxSet::load_defaults_newlines();
    let ts = highlighting::ThemeSet::load_defaults();

    let args: Args = argh::from_env();

    let mut render_methods: collections::BTreeMap<_, RenderMethod> = collections::BTreeMap::new();
    render_methods.insert("debug", Box::new(render_debug));
    if cfg!(feature = "ansi_term") {
        render_methods.insert("ansi_term", Box::new(render_ansi_term));
    }
    if cfg!(feature = "crossterm") {
        render_methods.insert("crossterm", Box::new(render_crossterm));
    }
    if cfg!(feature = "cursive") {
        render_methods.insert("cursive", Box::new(render_cursive));
    }
    if cfg!(feature = "termion") {
        render_methods.insert("termion", Box::new(render_termion));
    }

    let render_method = render_methods
        .get(args.render_method.as_str())
        .expect("Unsupported render method.  Did you activate all features?");

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

    render_method(&lines);
}
