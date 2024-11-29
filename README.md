<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./.github/assets/WEBlsp_logo_dark.svg" />
  <img alt="WEBlsp logo" src="./.github/assets/WEBlsp_logo_light.svg" />
</picture>

A better Language Server for the Web, made with Rust.

(Work In Progress) ~~It provides full-featured language support for HTML and CSS, including syntax highlighting, code completion, error checking, and more.~~ Compliant with the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), WEBlsp is designed to work with any IDE that supports LSP. Build on top of ~~HTMLlsrs~~ and [CSSlsrs](https://github.com/web-lsp/csslsrs), our own modern, fast, and reliable Language Services.

## Crates

WEBlsp is a monorepo that contains the following crates (Rust packages):

| Name      | Description              | Crates.io | README                                  |
| --------- | ------------------------ | --------- | --------------------------------------- |
| `weblsp`  | The main Language Server | WIP       | [README.md](./crates/weblsp/README.md)  |
| `csslsrs` | CSS Language Service     | WIP       | [README.md](./crates/csslsrs/README.md) |

## Packages

Additionally, WEBlsp contains the following NPM packages (JavaScript packages):

| Name             | Description               | NPM | README                                           |
| ---------------- | ------------------------- | --- | ------------------------------------------------ |
| `vscode`         | WEBlsp VSCode extension   | WIP | [README.md](./packages/vscode/README.md)         |
| `csslsrs`        | WASM CSS Language Service | WIP | [README.md](./packages/csslsrs/README.md)        |
| `benchmark-wasm` | Benchmark WASM packages   | WIP | [README.md](./packages/benchmark-wasm/README.md) |
