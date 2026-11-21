# PDF Translator for macOS

Local source snapshot for review. The desktop build identifies itself as
PDF Translator 2.10.2, build 11. This snapshot matches the Rust and workbench
sources used for the current Apple Silicon app.

The original prototype used
[PDFMathTranslate-next 2.10.2](https://github.com/PDFMathTranslate-next/PDFMathTranslate-next/releases/tag/v2.10.2).
The current app uses an AppKit/WebKit shell and a Rust backend. Native PDF
processing uses [BabelDOC 0.7.1](https://github.com/funstory-ai/BabelDOC/tree/d922e2b0c5a353bde8731921eb442c07ae75849d).
Its vendored sources and license are in `vendor/babeldoc`.

## Build and run locally

Build on macOS with Rust and the macOS SDK:

```sh
cargo build --release --locked
```

Run against the resources from the matching locally installed app:

```sh
HERMENEUEIN_RESOURCES="/path/to/PDF Translator.app/Contents/Resources" \
HERMENEUEIN_DATA="$(mktemp -d)" \
  ./target/release/hermeneuein
```

The runtime resource directory contains `web/`, `models/` and `fonts/`.
`packaging/runtime-assets.sha256` records the font and model hashes from the
matching local build. The workbench source and its PDF.js dependency are in
`web/`. The macOS bundle metadata is in `packaging/Info.plist`.

## Distribution notes

This review snapshot contains source, dependency notices, the bundled PDF.js
distribution, and resource checksums. The signed app, font and model binaries,
film-inspired backgrounds, local configuration, and private test documents are
not included. `upstream-notices/` contains the notices carried by the local
build. A public binary release needs a verified resource manifest and a
corresponding source distribution for its AGPL components.
