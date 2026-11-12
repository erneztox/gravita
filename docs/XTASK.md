# xtask specification

cargo xtask is Gravita's deterministic developer tool. It enforces architecture, command-contract integrity, WBS traceability, fixture validity and release gates. It is not an application command bus.

## Required command interface

| Command | Required behavior / output | Exit policy |
|---|---|---|
| help | List commands and flags | 0 |
| audit layers | Validate dependency graph against explicit crate-layer policy | Nonzero on violation |
| audit commands | Compare typed command registry, schemas, documentation and WBS trace; report missing, duplicate and orphan IDs | Nonzero on mismatch |
| audit rescued-commands | Compare the rescued-operation JSON contract against typed Rust payloads, Command implementations, parameter fields/types and authoritative dispatcher registration | Nonzero on mismatch |
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

## Rescued legacy command contract

The complete extraction of the old engine Bus is stored at `docs/inventory/legacy-command-bus.json`. The new product's selected operations are defined independently in `docs/contracts/rescued-commands.json`.

`cargo xtask audit rescued-commands` checks the latter against the new implementation. Each rescued operation declares its ID as `Command::ID`, exposes a typed payload struct, implements `Command`, and is registered in the authoritative dispatcher with `CommandBus::register::<Type>()`. The audit checks IDs, payload fields/types and dispatcher registration.
