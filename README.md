# alt-libsqlite3-sys

This repository provides an alternative version for the `libsqlite3-sys` crate, which can be found at [rusqlite/rusqlite on GitHub](https://github.com/rusqlite/rusqlite) in the `libsqlite3-sys` directory.

The purpose is to provide a newer version of the SQLite3 sources by more frequently syncing with the current amalgamation release of SQLite 3.

In order to use this repository in your project, you can simply set up a patch in your `Cargo.toml`:

```toml
[patch.crates-io]
libsqlite3-sys = { git = "https://github.com/videogame-hacker/alt-libsqlite3-sys-rs.git", tag = "3.38.5-1" }
```
