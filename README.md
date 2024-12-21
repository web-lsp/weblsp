<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/web-lsp/weblsp/refs/heads/main/.github/assets/WEBlsp_logo_dark.svg" />
  <img alt="WEBlsp logo" src="https://raw.githubusercontent.com/web-lsp/weblsp/refs/heads/main/.github/assets/WEBlsp_logo_light.svg" />
</picture>

A Language Server for the Web.

(Work In Progress) ~~It provides full-featured language support for HTML and CSS, including code completion, diagnostics, hover, and more.~~ Compliant with the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), WEBlsp is designed to work with any IDE that supports LSP. Build on top of ~~HTMLlsrs~~ and [CSSlsrs](https://github.com/web-lsp/weblsp/tree/main/crates/csslsrs), our own modern, fast, and reliable Language Services.

## Quick Links

🐛 [Report a bug](https://github.com/web-lsp/weblsp/issues), please read our [contributing guidelines](https://github.com/web-lsp/weblsp/blob/main/CONTRIBUTING.md) and [code of conduct](https://github.com/web-lsp/weblsp/blob/main/CODE_OF_CONDUCT.md) first.

🚨 [Report a security vulnerability](https://github.com/web-lsp/weblsp/security/advisories/new), and be sure to review our [security policy](https://github.com/web-lsp/weblsp/blob/main/SECURITY.md).

💬 [Join the discussion](https://github.com/web-lsp/weblsp/discussions), if you have any questions, ideas, or suggestions.

## Development

### Getting Started

To get started with WEBlsp, you will need to install the following tools:

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)
- [PNPM](https://pnpm.io/installation)
- [Just](https://just.systems/)
- [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/) (optional, for building WASM binaries)
- [wasm-opt](https://github.com/WebAssembly/binaryen) (optional, for WASM benchmarks and releases)

Useful commands:

| Command           | Description                                             |
| ----------------- | ------------------------------------------------------- |
| `just install`    | Install NPM dependencies across all crates and packages |
| `just build`      | Build WEBlsp crate and every language services          |
| `just build-wasm` | Build WASM packages                                     |
| `just test`       | Run tests for all crates and packages                   |
| `just benchmark`  | Run native, WASM, and E2E benchmarks                    |

### Crates

WEBlsp is a monorepo that contains the following crates (Rust packages):

| Name      | Description              | Crates.io | README                                  |
| --------- | ------------------------ | --------- | --------------------------------------- |
| `weblsp`  | The main Language Server | WIP       | [README.md](./crates/weblsp/README.md)  |
| `csslsrs` | CSS Language Service     | WIP       | [README.md](./crates/csslsrs/README.md) |

### Packages

Additionally, WEBlsp contains the following NPM packages (JavaScript packages):

| Name                  | Description                                | NPM | README                                                |
| --------------------- | ------------------------------------------ | --- | ----------------------------------------------------- |
| `vscode`              | WEBlsp VSCode extension                    | WIP | [README.md](./packages/vscode/README.md)              |
| `csslsrs`             | WASM CSS Language Service                  | WIP | [README.md](./packages/csslsrs/README.md)             |
| `benchmark-wasm`      | Benchmark WASM packages                    | WIP | [README.md](./packages/benchmark-wasm/README.md)      |
| `ls-tests-benchmarks` | End-to-end tests and benchmarks for WEBlsp | WIP | [README.md](./packages/ls-tests-benchmarks/README.md) |

