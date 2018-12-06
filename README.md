# rust-chromaprint-native

A rust wrapper around the [AcoustID Chromaprint][chromaprint] C library.

## Usage

Just add the dependency. Chromaprint will be built at compile time.

    chromaprint_native = { git = "https://github.com/0xcaff/rust-chromaprint" }

To use an already built version of chromaprint, specify the
`CHROMAPRINT_LIB_DIR` environment variable at build time. When specified, the
library in the specified folder will be used instead of building one at
compile time.

## Example

There are some example in the [tests].

[tests]: ./src/tests.rs
[chromaprint]: https://acoustid.org/chromaprint
