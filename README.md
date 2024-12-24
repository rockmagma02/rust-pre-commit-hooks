<!-- markdownlint-disable no-inline-html first-line-heading no-emphasis-as-heading ol-prefix -->

<div align="center">

# Rust Pre-Commit Hooks

**A set of pre-commit hooks for Rust projects**

[![GitHub License](https://img.shields.io/github/license/rockmagma02/rust-pre-commit-hooks)](https://github.com/rockmagma02/rust-pre-commit-hooks/blob/main/LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/rockmagma02/rust-pre-commit-hooks)](https://github.com/rockmagma02/rust-pre-commit-hooks/releases)
[![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/rockmagma02/rust-pre-commit-hooks)](https://github.com/rockmagma02/rust-pre-commit-hooks/issues)
[![GitHub Issues or Pull Requests](https://img.shields.io/github/issues-pr/rockmagma02/rust-pre-commit-hooks)](https://github.com/rockmagma02/rust-pre-commit-hooks/pulls)
[![Static Badge](https://img.shields.io/badge/pre--commit-hooks-blue)](https://pre-commit.com/)
![GitHub Repo stars](https://img.shields.io/github/stars/rockmagma02/rust-pre-commit-hooks)

</div>

## Table of Contents

<!-- markdownlint-disable ul-indent -->

- [Implementation](#implementation)
- [Hooks](#hooks)
    - [Code Formatting](#code-formatting)
    - [Linting](#linting)
    - [Example](#example)
- [Known Issues](#known-issues)
- [License](#license)

<!-- markdownlint-enable ul-indent -->

## Implementation

This project uses a `build.rs` script to automatically install and configure necessary Rust components (rustfmt and clippy) for all installed toolchains. The build script:

1. Detects all installed Rust toolchains
2. Installs `rustfmt` and `clippy` components for each toolchain
3. Ensures rustup is properly initialized

This approach ensures that the pre-commit hooks will work correctly regardless of which Rust toolchain you're using.

## Hooks

The following pre-commit hooks are available:

### Code Formatting

- **rustfmt**: Formats individual Rust files using `rustfmt`
  - ID: `rustfmt`
  - Runs on: `.rs` files

- **rustfmt-check**: Checks if Rust files are properly formatted without modifying them
  - ID: `rustfmt-check`
  - Runs on: `.rs` files

- **cargo-fmt**: Formats all bin and lib files in the current crate
  - ID: `cargo-fmt`
  - Runs on entire project

- **cargo-fmt-check**: Checks formatting of all bin and lib files without modifying them
  - ID: `cargo-fmt-check`
  - Runs on entire project

### Linting

- **clippy**: Runs Clippy lints on all crates in the project
  - ID: `clippy`
  - Runs on entire project

- **clippy-driver**: Runs Clippy lints on individual Rust files
  - ID: `clippy-driver`
  - Runs on: `.rs` files

### Example

```yaml
# .pre-commit-config.yaml

  - repo: https://github.com/rockmagma02/rust-pre-commit-hooks
    rev: v0.1.0
    hooks:
      - id: rustfmt
      - id: rustfmt-check
      - id: cargo-fmt
      - id: cargo-fmt-check
      - id: clippy
      - id: clippy-driver
```

## Known Issues

1. When modifying (install new) Rust toolchains, you may need to reinstall the pre-commit hooks to ensure the components (rustfmt and clippy) are properly installed for the new toolchain.

```bash
pre-commit uninstall
pre-commit clean
pre-commit install
```

2. the clippy-driver hook will compile all lib and bin files in the current crate. Will generate a set of binaries in the root of the project, which we strongly recommend not to use this hook.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/rockmagma02/rust-pre-commit-hooks/blob/main/LICENSE) file for details.

The MIT License is a permissive license that allows you to use, modify, and distribute this software for any purpose, provided that the license and copyright notice are included.
