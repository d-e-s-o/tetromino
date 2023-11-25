tetromino-xlock-bindings
========================

**tetromino-xlock-bindings** provides the means for creating bindings
for `xlock(1)` -- a popular X11 screen lock. The bindings are suitable
for writing a custom "mode", e.g., in the form of a module.


Usage
-----

The crate comes with pre-generated bindings for a given version of
`xlock` that can be used directly as any other crate. `xlock` appears to
be taking compatibility into account and so there is a chance that
bindings generated for one version end up producing binaries that are
compatible with another version. It also seems as if few if any
`configure`d features affect the ABI or API, likely rendering the
default bindings suitable in a reasonable number of use cases.

That being said, the crate provides the means for generating updated
bindings as follows:
- a build with the `download-xlock-source` feature enabled will download
  the `xlock` source code into `xlock-src/`
  - the `XLOCK_VERSION` environment variable is honored, specifying the
    version to download (defaults to `5.73`)
  - alternatively, the `XLOCK_SRC_ARCHIVE_URL` environment variable can
    be used to specify a URL of a `tar.xz` that will be downloaded and
    extracted into `xlock-src/`
- if the `generate-xlock-bindings` feature is enabled, `bindings.rs`
  will be regenerated
  - by default bindings will be generated based on contents in
    `xlock-src/`
  - however, if the `XLOCK_SRC_ROOT` environment variable is set an
    attempt is made to create them based on data in the referenced
    directory

If you don't have `xlock` installed, you can build it directly from
`xlock-src` (after downloading the source code; see above). Please refer
to instructions provided in the source code.

Note that by default all `xlock` "modes" are statically linked into the
binary at build time, making it rather cumbersome to ship a new "mode".
In recent versions `xlock` comes with experimental support for modules
(shared objects), which can be discovered at start-up time. You most
likely want to configure `xlock` to include support for modules (refer
to their build instructions for how to go about that).
