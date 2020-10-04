<!---
SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
SPDX-License-Identifier: CC0-1.0
-->

# Unreleased

- Add `Strikethrough` effect.
- Implement `From<&str>` for `StyledStr` and `From<String>` for `StyledString`.

# v0.2.0 (2020-10-03)

This minor release refactors `Effects`, adds setters to `Style` and changes the
arguments of `render_iter`.

- Add setter methods to `Style`.
- Accept `IntoIterator` in `render_iter` functions (instead of directly using
  `Iterator`).
- Refactor `Effects`:
  - Change `Effects` to a struct with boolean fields instead of an enum set.
  - Implement `From<syntect::highlighting::FontStyle>` for `Effects`.
  - Drop the `enumset` dependency.

# v0.1.1 (2020-09-30)

This patch release makes sure that the font style is properly reset in the
`termion` backend and adds some utility methods.

- Add `style_mut` method to `StyledStr` and `StyledString`.
- Implement `From<Effect>` and `From<Effects>` for `Style`.
- Always use `termion::style::Reset` to clear the formatting in the `termion`
  backend.
- Add basic backend test suite.

# v0.1.0 (2020-09-24)

Initial release with support for `ansi_term`, `crossterm`, `cursive`, `syntect`
and `termion`.
