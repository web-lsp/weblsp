<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../.github/assets/WEBlsp_logo_dark.svg" />
  <img alt="WEBlsp logo" src=".../../.github/assets/WEBlsp_logo_light.svg" />
</picture>

A better Language Server for the Web, made with Rust.

(Work In Progress) ~~It provides full-featured language support for HTML and CSS, including syntax highlighting, code completion, error checking, and more.~~ Compliant with the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), WEBlsp is designed to work with any IDE that supports LSP. Build on top of ~~HTMLlsrs~~ and [CSSlsrs](https://github.com/web-lsp/csslsrs), our own modern, fast, and reliable Language Services.

## TODO

- [x] LSP basic server
- [ ] Handle request by language (HTML, CSS)
- [x] CSS Language Service (CSSlsrs) integration
- [ ] HTML Language Service (HTMLlsrs) integration
- [ ] Publish VSCode extension

## Useful commands

- `cargo build` - Build the project
- `cd ./packages/vscode && pnpm run compile` - Compile the vscode extension
