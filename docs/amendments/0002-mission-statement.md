<!-- kit: governance v0.1.0 -->
# 0002 — Missions

- **Status:** proposed
- **Date:** 2026-11-16
- **Proposed by:** Grok Bot (agent), at the maintainer's request
- **Ratified by:**
- **Affects:** constitution §1; PRD §1.1; feature SLANG-008
- **Supersedes:** none
- **Related:** [PRD.md](../../PRD.md) §1.1; feature SLANG-008

## Context

Constitution §1 Mission was a product description (or, for a new constitution, had not been written yet), not a three-mission direction statement. The fleet now requires every repo's constitution §1 to be one direction statement followed by exactly three numbered missions, adopted through a numbered amendment the maintainer ratifies by merging. The maintainer asked for those missions to be comprehensive and epic: Schema Lang as the ultimate open home for all things structural-contract languages. Mission 1 follows the maintainer's wording, adapted to this domain: provide data, tools, resources and transparency for all stakeholders, domain practitioners and the general public. The ambition is the direction, not a claim that the work is already finished. The missions are aligned with this repo's existing constitution and PRD.

## Amendment

Replace constitution §1 with the following, retitled **Missions**:

Schema Lang's direction is to become the ultimate open home for all things structural-contract languages — data, tools, resources and transparency, gathered in one place that anyone can inspect and reuse.

1. **Provide data, tools, resources and transparency for all stakeholders, language and toolchain practitioners, and the general public.** Compiler learners, SCHEMA authors and fleet agents that consume contracts are first-class audiences. The aim is a shared, inspectable commons: the same language, the same diagnostics, and the same emitted contracts for everyone.
2. **Turn structural contracts into a real language: a lexer, parser, checker and code generator for SCHEMA blueprints that anyone can read, type-check and compile.** Errors should carry span-based diagnostics, and the language should stay small enough to learn by building.
3. **Compile those blueprints for gitorio and SCHEMA, and validate them against the hub's schema lint rules, so every repo's structure stays local, cheap and verifiable.** The compiler exists to make the fleet's structural contracts machine-checkable, not to invent a parallel undocumented format.

These missions set the course; they do not claim the work is finished. They serve the PRD goals: a lexer and parser for a small blueprint DSL, semantic checks with good error messages, and emission of SCHEMA.md and gitorio blueprint files.

_Source: [PRD.md](../PRD.md) §1.1._


## Consequences

Agents and reviewers judge fit against these three missions before inventing scope, treating them as the direction of the repo rather than a description of what already ships. The same text is added as PRD §1.1 so the constitution's `_Source:` line stays accurate. The original PRD purpose / overview paragraph is left in place. This PR also aligns the README and AGENTS.md taglines with the missions and links them to constitution §1, and adds feature SLANG-008 to the feature catalog, traced to requirement MR1. Merging this PR ratifies amendment 0002. Until then the record stays `Status: proposed`.
