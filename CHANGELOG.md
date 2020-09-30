<!---
SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
SPDX-License-Identifier: CC0-1.0
-->

# Unreleased

- Add `style_mut` method to `StyledStr` and `StyledString`.
- Implement `From<Effect>` and `From<Effects>` for `Style`.
- Always use `termion::style::Reset` to clear the formatting in the `termion`
  backend.
- Add basic backend test suite.

# v0.1.0 (2020-09-24)

Initial release with support for `ansi_term`, `crossterm`, `cursive`, `syntect`
and `termion`.
