# CSSlsrs

A CSS Language Service made with Rust.

(Work In Progress) ~~It provides full-featured language support for CSS, including syntax highlighting, code completion, error checking, and more.~~ Compliant with the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), CSSlsrs can be integrated into Language Servers written in Rust or JavaScript (via WebAssembly). Thanks to [Biome](https://biomejs.dev/)'s parser, we deliver a fast and efficient language service, designed to improve your productivity when working with CSS.

## Features

- [x] Document store
- [x] CSS Parser (`biome-css-parser`)
- [x] WASM support
- Features
  - [ ] Hover
  - [ ] Completion
    - [ ] Path completion
  - [ ] Definition
  - [ ] References
  - [ ] Document Symbols
  - [ ] Document Highlights
  - [ ] Code Actions
  - [ ] Code Lens
  - [ ] Rename
  - [x] Colors
		- [ ] Color Presentation
  - [x] Folding
  - [ ] Selection Range
  - [ ] Validation
- [ ] Support for custom data
- [ ] Benchmarks with `vscode-css-languageservice`
- [ ] Support for super-set of CSS (like SCSS, SASS, LESS)
