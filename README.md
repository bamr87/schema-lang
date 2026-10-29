# schema-lang

> A Rust DSL and compiler for gitorio/SCHEMA blueprints: parse, type-check and emit SCHEMA.md contracts.

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

## Quick start

```bash
cargo build && cargo test
```

## How it grows

The roadmap lives in [PRD.md](PRD.md) and [features/features.yml](features/features.yml) (ids `SLANG-NNN`, most `status: planned`). Work arrives as pull requests from the hub's evolution and fan-out loops, the `@claude` handler, or the local `evolve` lane (off until `EVOLVE_ENABLED=true`). A human merges every PR. Rules: [docs/constitution.md](docs/constitution.md) · agent guide: [AGENTS.md](AGENTS.md).

## License

[MIT](LICENSE)
