<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/web-lsp/weblsp/refs/heads/main/.github/assets/CSSlsrs_logo_dark.svg" />
  <img alt="WEBlsp logo" src="https://raw.githubusercontent.com/web-lsp/weblsp/refs/heads/main/.github/assets/CSSlsrs_logo_light.svg" />
</picture>

A CSS Language Service made with Rust.

(Work In Progress) ~~It provides full-featured language support for HTML and CSS, including code completion, diagnostics, hover, and more.~~ Compliant with the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), CSSlsrs can be integrated into Language Servers written in Rust or JavaScript/TypeScript (via WebAssembly). Thanks to [Biome](https://biomejs.dev/)'s parser, we deliver a fast and efficient language service, designed to improve your productivity when working with CSS.

## Quick Links

📖 [Main repository](https://github.com/web-lsp/weblsp/), with all related crates and packages.

🐛 [Report a bug](https://github.com/web-lsp/weblsp/issues), please read our [contributing guidelines](https://github.com/web-lsp/weblsp/blob/main/CONTRIBUTING.md) and [code of conduct](https://github.com/web-lsp/weblsp/blob/main/CODE_OF_CONDUCT.md) first.

🚨 [Report a security vulnerability](https://github.com/web-lsp/weblsp/security/advisories/new), and be sure to review our [security policy](https://github.com/web-lsp/weblsp/blob/main/SECURITY.md).

💬 [Join the discussion](https://github.com/web-lsp/weblsp/discussions), if you have any questions, ideas, or suggestions.

## Features

| Feature             | CSSlsrs | VScode | Differences            |
| ------------------- | ------- | ------ | ---------------------- |
| Hover               | 🟨       | ✅      | Lacks HTML previews    |
| Completion          | ❌       | ✅      | -                      |
| Path completion     | ❌       | ✅      | -                      |
| Definition          | ❌       | ✅      | -                      |
| References          | ❌       | ✅      | -                      |
| Document Symbols    | ✅       | ✅      | Supports more symbols  |
| Document Highlights | ❌       | ✅      | -                      |
| Code Actions        | ❌       | ✅      | -                      |
| Code Lens           | ❌       | ✅      | -                      |
| Rename              | ❌       | ✅      | -                      |
| Colors              | ✅       | ✅      | -                      |
| Color Presentation  | ✅       | ✅      | Supports for LCH & Lab |
| Folding             | ✅       | ✅      | -                      |
| Selection Range     | ❌       | ✅      | -                      |
| Validation          | ❌       | ✅      | -                      |
| Custom data         | ❌       | ✅      | -                      |
| Super-set of CSS    | ❌       | ✅      | -                      |

## Getting Started

WIP.

## Building

WIP.
