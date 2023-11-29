tetromino-xlock-mode
====================

**tetromino-xlock-mode** is an `xlock(1)` mode running **tetromino**
(with auto-playing AI enabled) when the screen is locked.


Usage
-----

To use the mode please make sure that your `xlock` is built with module
support. **tetromino-xlock-mode** creates a module that you can install
and which, subsequently, `xlock` can pick up and use.

To install the module, use `cargo build --release` to build it and then
install the binary:
```sh
install --no-target-directory \
  target/release/libtetromino_xlock_mode.so \
  /usr/local/lib/X11/xlock/modules/tetromino.xlk
```

The `xlock` module path may differ on your system. Path such as
`/usr/lib/X11/xlock/modules/` have also been seen.

To use the lock screen mode, use `xlock -mode tetromino`. Note that you
*may* have to regenerate `xlock` bindings and redo the above steps if
your version of the program has changed its ABI compared to the shipped
bindings. Refer to the [**tetromino-xlock-bindings**
README][tetromino-xlock-bindings-readme] for details.

[tetromino-xlock-bindings-readme]: ../bindings/README.md
