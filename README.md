[![pipeline](https://github.com/d-e-s-o/tetromino/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/d-e-s-o/tetromino/actions/workflows/test.yml)
[![crates.io](https://img.shields.io/crates/v/tetromino.svg)](https://crates.io/crates/tetromino)

tetromino
=========

- [Changelog](CHANGELOG.md)

**tetromino** is a graphical Tetris clone. It is also a rewrite of a
previous C++ based [Tetris clone][tetris] that improves on a few
shortcomings.


Usage
-----

**tetromino** uses the following key bindings:

| Key(s) | Function                                 |
|--------|------------------------------------------|
| 1      | Rotate stone left                        |
| 2      | Rotate stone right                       |
| h      | Move stone left                          |
| l      | Move stone right                         |
| j      | Move stone down                          |
| Space  | Drop the stone immediately               |
| q      | Quit the game                            |
| Return | Restart the game                         |
| F3     | Pause (and resume) the game              |


Certain aspects of the game can be configured via its configuration
file. This file is expected at `$XDG_CONFIG_DIR/tetromino/config.toml`.
Please refer to the help text (`tetromino --help`) for details on what
can be configured and how.


[tetris]: https://github.com/d-e-s-o/tetris
