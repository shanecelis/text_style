<!---
Copyright (C) 2020 Robin Krahl <robin.krahl@ireas.org>
SPDX-License-Identifier: CC0-1.0
-->

# text-style-rs

The `text-style` crate provides types and conversions for styled text.

Many crates produce or consume styled text.  But converting between these
crates requires lots of boilerplate code.  This crate defines a subset of
common types for styled text and conversion methods for multiple crates.
This makes it easy to convert between the different formats and to prepare
output that can be rendered by any of the supported crates.

```rust
let s = text_style::StyledStr::plain("test").bold();

let mut w = std::io::stdout();
text_style::ansi_term::render(&mut w, &s).expect("Rendering failed");
text_style::crossterm::render(&mut w, &s).expect("Rendering failed");
text_style::termion::render(&mut w, &s).expect("Rendering failed");
```

For more information, see the [API documentation](https://docs.rs/text-style).

## Features

This crate has the following features:

- `ansi_term`: convert to [`ansi_term`](https://lib.rs/ansi_term) types
- `crossterm`: convert to [`crossterm`](https://lib.rs/crossterm) types
- `cursive`: convert to [`cursive`](https://lib.rs/cursive) types
- `syntect`: convert from [`syntect`](https://lib.rs/syntect) types
- `termion`: convert to [`termion`](https://lib.rs/termion) types

All features are disabled per default.

## Minimum Supported Rust Version

This crate supports Rust 1.42.0 or later.

## Contributing

Contributions to this project are welcome!  Please submit patches to the
mailing list [~ireas/public-inbox@lists.sr.ht][] ([archive][]) using the
`[PATCH text-style-rs]` subject prefix.  For more information, see the
[Contributing Guide][].

## Contact

For bug reports, feature requests and other messages, please send a mail to
[~ireas/public-inbox@lists.sr.ht][] ([archive][]) using the `[text-style-rs]`
prefix in the subject.

## License

This project is dual-licensed under the [Apache-2.0][] and [MIT][] licenses.
The documentation and examples contained in this repository are licensed under
the [Creative Commons Zero][CC0] license.  You can find a copy of the license
texts in the `LICENSES` directory.

`merge-rs` complies with [version 3.0 of the REUSE specification][reuse].

[~ireas/public-inbox@lists.sr.ht]: mailto:~ireas/public-inbox@lists.sr.ht
[archive]: https://lists.sr.ht/~ireas/public-inbox
[Contributing Guide]: https://man.sr.ht/~ireas/guides/contributing.md
[Apache-2.0]: https://opensource.org/licenses/Apache-2.0
[MIT]: https://opensource.org/licenses/MIT
[CC0]: https://creativecommons.org/publicdomain/zero/1.0/
[reuse]: https://reuse.software/practices/3.0/
