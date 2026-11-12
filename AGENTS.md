# AGENTS.md — Gravita engineering rules

## Product scope

Read `docs/ARCHITECTURE.md` and `docs/WBS.md` before making architectural changes. The approved relief-centric scope is authoritative. Gravita is not a vector editor, bitmap editor, general-purpose parametric solid CAD system, or 3D-printing tool.

## Source-of-truth rules

- The typed domain model and command API are authoritative.
- GUI, CLI and MCP are adapters; they must not implement domain behavior independently.
- A menu item, command ID, schema entry or TODO is not an implementation.
- Every retained command must trace to its handler, typed payload, validation, effects, result and acceptance tests.
- Never report progress from old repository counts or stale roadmap percentages.

## Domain correctness

- Use real geometry, reliefs, tools, toolpaths, stock and postprocessors in integration tests.
- No mocks for domain workflows. Test doubles may be used only for external boundaries (filesystem failures, clocks, process execution) and must not replace geometry/CAM/simulation.
- Reject NaN/infinity, invalid units, invalid topology and unsafe machining parameters at boundaries.
- Preserve physical units and coordinate conventions explicitly.
- Never silently change machining intent or export unsupported postprocessor semantics.
- Safety validation must gate G-code export.

## Rust rules

- Forbid unsafe Rust in every crate.
- No `unwrap()`, `expect()`, `panic!`, or `unreachable!` in production paths. Tests may use assertions and explicit fixture setup.
- Use typed errors and propagate failures with context.
- Keep domain crates free of UI, CLI and MCP dependencies.
- Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` before declaring a change complete.

## Dependency policy

Prefer maintained, mature Rust libraries for established geometry/CAM algorithms. Before adding a dependency, record version, license, maintenance signals, unsafe surface, transitive dependencies, API fit, numerical behavior and benchmark/fixture evidence. Do not add SolidCraft/SolidCore family crates. Do not reimplement a mature algorithm solely to avoid a dependency.

## Implementation workflow

1. Identify the WBS item and command IDs affected.
2. Inspect existing code and tests before changing behavior.
3. Implement the domain behavior behind a typed API.
4. Add unit and acceptance tests using real domain data.
5. Update command traceability, schema and WBS status from evidence.
6. Run the narrow test, then workspace quality gates.
7. Document limitations and unsupported inputs explicitly.

## Documentation

All repository instructions and technical docs are written in English. Prefer exact contracts, source paths, commands, acceptance conditions and status labels over broad prose. Do not invent Rust signatures, test results, dependencies, performance numbers or completed functionality.
