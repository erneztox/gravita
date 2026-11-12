# xtask specification

## Purpose

cargo xtask is Gravita's deterministic developer tool. It enforces architecture, command-contract integrity, WBS traceability, fixture validity and release gates. It is not an application command bus.

## Required command interface

| Command | Required behavior / output | Exit policy |
|---|---|---|
| help | List commands and flags | 0 |
| audit layers | Validate dependency graph against explicit crate-layer policy | Nonzero on violation |
| audit commands | Compare typed command registry, schemas, documentation and WBS trace; report missing, duplicate and orphan IDs | Nonzero on mismatch |
| audit rescued-commands | Compare the selected Gravita Core Bus functional contract against typed Rust commands, payload parameters and authoritative dispatcher registration | Nonzero on mismatch |
| audit frontend | Check GUI/CLI/MCP references against public command IDs; detect direct UI mutation | Nonzero on broken references or forbidden mutation |
| audit params | Validate parameter names, types, defaults, units and schema-generation parity | Nonzero on mismatch |
| audit fixtures | Validate fixture manifest, licenses, checksums and expected artifacts | Nonzero on missing or changed fixture |
| audit safety | Run toolpath invariants and export-safety policy checks | Nonzero on any safety violation |
| wbs report | Render WBS status from structured task metadata and test evidence | 0; unknown status remains unknown |
| commands list | Emit canonical command IDs and schemas as stable JSON | Nonzero on invalid registry |
| craft run | Execute the end-to-end programmatic craft and write artifacts to an explicit output directory | Nonzero on any failed stage |
| stats | Report source lines, tests, fixtures and build timings without implying completion | 0 |
| ci | Run formatting, Clippy, tests, architecture, command, fixture and safety audits | Stop on first failure |

## Implementation requirements

- Use clap for typed subcommands and flags.
- Determine workspace root from CARGO_MANIFEST_DIR; never assume the current working directory.
- Return structured errors; do not use panic-based handling for repository or user input.
- Make generated outputs deterministic and write atomically (temporary file then rename).
- Commands that write files must accept explicit output paths and never overwrite source fixtures.
- Keep audit logic in testable modules; CLI functions are thin adapters.
- Normal audit/CI commands must not access the network.
- Derive thresholds and expected inventories from the current Gravita contracts, never from arbitrary fixed counts.

## Proposed module layout

    xtask/
      src/main.rs
      src/commands/{mod.rs,audit.rs,craft.rs,stats.rs,wbs.rs}
      src/audit/{layers.rs,commands.rs,frontends.rs,params.rs,fixtures.rs,safety.rs}
      src/report/{mod.rs,wbs.rs,commands.rs}

## Acceptance tests

- A duplicate command ID fails the command audit.
- A typed command absent from schema, documentation or WBS trace is reported.
- A schema entry with a stale parameter type, default or unit fails parity.
- A UI handler that mutates the document without dispatch fails the frontend audit.
- A missing or checksum-modified fixture fails the fixture audit.
- An unsafe toolpath cannot pass safety audit or G-code export.
- cargo xtask ci produces deterministic results on a clean checkout and a nonzero exit on every injected failure fixture.

## Rescued Core Bus contract

The selected operations are defined in `docs/contracts/rescued-commands.json`. The contract is derived from the previous Gravita Rust Core Bus handlers and contains only operations within the new product scope. It excludes interactive vector authoring and bitmap editing. Imported 2D geometry remains an input to CAM, not an editing workbench.

`cargo xtask audit rescued-commands` compares this selected contract with the new implementation. It does not require the new Gravita to reproduce the previous Core Bus in full.
