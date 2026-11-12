# AGENTS.md — Gravita

## First principles

Read README.md, docs/ARCHITECTURE.md and docs/WBS.md before making architectural or product-scope changes.

Gravita is a relief-centric CAD/CAM application for artistic CNC machining. Its purpose is to create or import relief data, prepare machining geometry, generate toolpaths, simulate stock removal, validate machining safety and produce G-code.

Do not add interactive vector editing, bitmap editing, general-purpose parametric CAD, sketch constraints, 3D printing or machine-control connectivity. DXF/SVG, PNG/TIFF and STL/OBJ are import formats for domain data, not invitations to build general-purpose editors.

## Architecture rules

- The domain model and typed command API are authoritative.
- Desktop UI, CLI and MCP are adapters over the same engine API.
- Frontends must never implement domain behavior independently or mutate document state directly.
- Commands have stable IDs, typed payloads, validation, explicit effects and typed results.
- Transactions own mutations. Failed commands must not leave partial document changes.
- Domain crates must not depend on egui, wgpu, winit, CLI, MCP or application crates.
- Keep dependency direction inward-to-outward.

## Domain correctness

- Use real geometry, reliefs, tools, toolpaths, stock and postprocessors in integration tests.
- Do not mock geometry, CAM, simulation, safety validation or G-code generation.
- Test doubles are permitted only at external boundaries such as filesystem failures or clocks.
- Reject non-finite values, invalid units, invalid topology and unsafe machining parameters at boundaries.
- Define physical units and coordinate conventions explicitly.
- Never silently change machining intent or export unsupported postprocessor semantics.
- Safety validation must gate G-code export.

## Rust rules

- Forbid unsafe Rust in every crate.
- No unwrap(), expect(), panic! or unreachable! in production paths.
- Use typed errors and propagate failures with context.
- Run cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings and cargo test --workspace before declaring a change complete.

## Dependency policy

Prefer maintained, mature Rust libraries for established geometry and CAM algorithms. Before adding a dependency, evaluate its API semantics, license, maintenance, unsafe surface, dependency tree, numerical robustness, topology handling, determinism and performance. Wrap selected libraries behind narrow Gravita interfaces. Do not reimplement a mature algorithm merely to avoid a dependency.

## Implementation workflow

1. Identify the relevant WBS item.
2. Inspect existing code and tests.
3. Implement behavior behind a typed domain API.
4. Add unit and acceptance tests using real domain data.
5. Update command contracts, traceability and WBS status based on repository evidence.
6. Run focused tests, then workspace quality gates.
7. Document unsupported inputs and actual limitations.

## Documentation and reporting

- Technical documentation is written in English.
- Prefer exact contracts, source paths, commands and acceptance conditions.
- Do not invent Rust signatures, test results, dependencies, performance numbers or completed functionality.
- Distinguish specified, implemented and verified behavior.
- The product scope in this repository is the sole product authority.
