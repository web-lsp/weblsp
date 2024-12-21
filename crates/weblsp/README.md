<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/web-lsp/weblsp/refs/heads/main/.github/assets/WEBlsp_logo_dark.svg" />
  <img alt="WEBlsp logo" src="https://raw.githubusercontent.com/web-lsp/weblsp/refs/heads/main/.github/assets/WEBlsp_logo_light.svg" />
</picture>

A Language Server for the Web.

(Work In Progress) ~~It provides full-featured language support for HTML and CSS, including code completion, diagnostics, hover, and more.~~ Compliant with the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), WEBlsp is designed to work with any IDE that supports LSP. 

This crate contains the Language Server implementation in Rust. WEBlsp receives requests from the client, and dispatches them to the appropriate language service (e.g., [CSSlsrs](https://github.com/web-lsp/weblsp/tree/main/crates/csslsrs)), before sending the response back to the client.

## Quick Links

📖 [Main repository](https://github.com/web-lsp/weblsp/), with all related crates and packages.

🐛 [Report a bug](https://github.com/web-lsp/weblsp/issues), please read our [contributing guidelines](https://github.com/web-lsp/weblsp/blob/main/CONTRIBUTING.md) and [code of conduct](https://github.com/web-lsp/weblsp/blob/main/CODE_OF_CONDUCT.md) first.

🚨 [Report a security vulnerability](https://github.com/web-lsp/weblsp/security/advisories/new), and be sure to review our [security policy](https://github.com/web-lsp/weblsp/blob/main/SECURITY.md).

💬 [Join the discussion](https://github.com/web-lsp/weblsp/discussions), if you have any questions, ideas, or suggestions.
