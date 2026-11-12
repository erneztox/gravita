# xtask specification

## Purpose

`cargo xtask` is the repository's deterministic developer tool. It must enforce architecture, command-contract integrity, WBS traceability, fixture validity and release gates. It is not a second application command bus.

## Source audit: previous implementation

The previous `xtask/src/main.rs` dispatches these commands:

| Command | Observed behavior |
|---|---|
| `layers` | Reads `cargo metadata --no-deps`, classifies crates by architectural layer and reports forbidden dependency edges / UI dependencies below the frontend layer. |
| `panel-commands` | Lexes UI Rust source and detects click handlers that mutate document/session state directly rather than dispatching commands. |
| `parity` | Reads `research/data/menus.json`, runs `gravita-cli commands`, compares menu IDs/symbols/paths/labels with implemented/draft commands, writes `docs/parity-checklist.md`; has a historical hard floor of 822. |
| `gui-parity` | Runs the CLI command registry, scans UI source for command references, reports broken references and implemented commands not wired into GUI; historical floor 403. |
| `param-audit` | Lexically scans engine command handlers for parameter access and serde deserialization, merges GUI form coverage, writes generated schemas and `docs/gui-parameters.md`; historical floor 116. |
| `stats` | Counts source lines and tests by crate. |
| `ci` | Runs fmt, Clippy, workspace tests, layers, parity, panel-commands, gui-parity and param-audit in sequence; stops on first failure. |

The previous code uses lexical scanning for some audits. Such scanners can be useful regression guards but are not authoritative Rust semantic analysis. New xtask must report scanner limitations and pair lexical checks with compile-time or test-based invariants. Old numerical floors are not portable to this clean repository.

## Required command interface

| Command | Required behavior / output | Exit policy |
|---|---|---|
| `help` | List commands and flags | 0 |
| `audit layers` | Validate dependency graph against explicit crate-layer policy | nonzero on violation |
| `audit commands` | Compare typed command registry, schemas, docs and WBS trace; list missing, duplicate and orphan IDs | nonzero on mismatch |
| `audit frontend` | Check GUI/CLI/MCP references against public command IDs; detect UI direct mutation | nonzero on broken refs or forbidden mutation |
| `audit params` | Validate parameter names/types/defaults/units and schema generation parity | nonzero on mismatch |
| `audit fixtures` | Validate fixture manifest, licenses, checksums and expected artifacts | nonzero on missing/changed fixture |
| `audit safety` | Run toolpath invariants and export-safety policy checks | nonzero on any safety violation |
| `wbs report` | Render machine-derived WBS status from structured task metadata and test evidence | 0; report unknown status as unknown |
| `commands list` | Emit canonical command IDs and schemas as stable JSON | nonzero on invalid registry |
| `craft run` | Execute the checked-in end-to-end programmatic craft; write artifacts to an explicit output directory | nonzero on any failed stage |
| `stats` | Report LOC, tests, fixtures and build timings without implying completion | 0 |
| `ci` | Run fmt, Clippy, tests, architecture, command, fixture and safety audits | stop on first failure |

## Implementation requirements

- Use `clap` for typed subcommands and flags; do not parse positional strings manually.
- Determine workspace root from `CARGO_MANIFEST_DIR`; never assume current working directory.
- Return structured errors; do not use `expect`, `unwrap`, or panic for repository/user input.
- Make generated outputs deterministic and write atomically (temporary file then rename).
- Commands that write files must accept explicit output paths and never overwrite source fixtures.
- Keep audit logic in testable modules; CLI functions are thin adapters.
- No network access in normal audit/CI commands. Dependency research is a separate documented process.
- Avoid historical floors such as 403/822/116 unless recalculated from the new repository's own explicit contract and rationale.

## Proposed module layout

```text
xtask/
  src/main.rs
  src/commands/{mod.rs,audit.rs,craft.rs,stats.rs,wbs.rs}
  src/audit/{layers.rs,commands.rs,frontends.rs,params.rs,fixtures.rs,safety.rs}
  src/report/{mod.rs,wbs.rs,commands.rs}
```

## Acceptance tests

- A duplicate command ID fails the command audit.
- A command in the typed registry but absent from schema/docs/WBS trace is reported.
- A schema entry with a stale parameter type/default/unit fails parity.
- A UI handler that mutates the document without dispatch fails frontend audit.
- A missing or checksum-modified fixture fails fixture audit.
- A known unsafe toolpath cannot pass safety audit or G-code export.
- `cargo xtask ci` produces the same pass/fail result on a clean checkout and a nonzero exit on every injected failure fixture.
