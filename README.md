# schema-lang

> Schema Lang's direction is to become the ultimate open home for all things structural-contract languages: data, tools, resources and transparency, with a SCHEMA blueprint compiler that parses, type-checks and emits verifiable contracts for everyone ([constitution](docs/constitution.md) §1).

Turn the fleet's structural contracts into a real language: a lexer, parser, checker and code generator for SCHEMA blueprints.

**Status:** foundation (phase 1). Part of the [bamr87 fleet](https://github.com/bamr87/bamr87).

## Goals

- Lexer and parser for a small blueprint DSL
- Semantic checks with good error messages
- Emit SCHEMA.md / gitorio blueprint files

## Skills learned

- Compiler front ends (lexing, parsing, ASTs)
- Type checking and diagnostics
- Rust error handling and testing
- Language design

## Fleet tie-in

Compiles blueprints for bamr87/gitorio and bamr87/SCHEMA; validates against the hub's schema_lint.py rules.

## Usage

```bash
cargo run -q -- SCHEMA.blueprint          # lex a file, one token per line
echo 'dir src {}' | cargo run -q -- -     # lex standard input
echo 'dir src {}' | cargo run -q -- --spans -   # prefix each token with line:col (1:1 Ident("dir"), ...)
cargo run -q -- --help                    # usage (stdout), exit 0
cargo run -q -- -V                        # schema-lang <version>
```

Exit codes: `0` success, `1` unreadable input or a lex error (for example `1:19: unterminated string literal`), `2` usage error (no arguments, unknown option).

## Quick start

```bash
cargo build && cargo test
```

## Documentation

The documentation index is [docs/README.md](docs/README.md): every document in the repository, one line each.

## How it grows

The roadmap lives in [PRD.md](PRD.md) and [features/features.yml](features/features.yml) (ids `SLANG-NNN`, most `status: planned`). Work arrives as pull requests from the hub's evolution and fan-out loops, the `@claude` handler, or the local `evolve` lane (off until `EVOLVE_ENABLED=true`). A human merges every PR. Rules: [docs/constitution.md](docs/constitution.md) · agent guide: [AGENTS.md](AGENTS.md).

## License

[MIT](LICENSE)
