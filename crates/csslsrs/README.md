<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../.github/assets/CSSlsrs_logo_dark.svg" />
  <img alt="CSSlsrs logo" src="../../.github/assets/CSSlsrs_logo_light.svg" />
</picture>

A CSS Language Service made with Rust.

(Work In Progress) ~~It provides full-featured language support for CSS, including syntax highlighting, code completion, error checking, and more.~~ Compliant with the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), CSSlsrs can be integrated into Language Servers written in Rust or JavaScript (via WebAssembly). Thanks to [Biome](https://biomejs.dev/)'s parser, we deliver a fast and efficient language service, designed to improve your productivity when working with CSS.

## Features

| Feature             | CSSlsrs | VScode | Differences            |
| ------------------- | ------- | ------ | ---------------------- |
| Hover               | 🟨       | ✅      | Lacks HTML previews    |
| Completion          | ❌       | ✅      | -                      |
| Path completion     | ❌       | ✅      | -                      |
| Definition          | ❌       | ✅      | -                      |
| References          | ❌       | ✅      | -                      |
| Document Symbols    | ❌       | ✅      | -                      |
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