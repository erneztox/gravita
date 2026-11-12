# Gravita architecture

**Status:** implementation baseline for the clean repository. The approved product pivot dated 2026-11-11 in the source investigation repository is authoritative over older README, roadmap, GUI, and crate documentation.

## Product definition

Gravita is a Rust desktop application for artistic CNC relief design and manufacture. Its core workflow is:

1. Import external 2D geometry (DXF/SVG), heightmaps (PNG/TIFF, 8/16-bit as supported), and triangle meshes (STL/OBJ).
2. Build and edit 2.5D reliefs on heightfield data; combine, transform, sweep, turn, sculpt, smooth, draft, slice, and inspect reliefs.
3. Prepare 2D regions and 3D surfaces for CAM.
4. Generate 2.5D and 3D toolpaths: profile, pocket, V-carve, drill, Z-level roughing, raster/offset/spiral relief finishing, rest machining, tabs/ramps, and toolpath management.
5. Simulate stock removal and run collision/gouge/safety checks.
6. Post-process verified toolpaths to controller-specific G-code.
7. Save/load the native Gravita project.

## Explicit non-goals

No interactive vector drawing, node editing, Bézier handle editing, sketch constraints, raster painting, pixel brush, flood fill, indexed-palette editor, generic parametric solid CAD, 3D-printing workflow, ArtCAM binary compatibility, legacy serial-port machine driver, DRM/dongle emulation, or external SolidCraft family integration. DXF/SVG are imported geometry, not a vector editor. PNG/TIFF are heightmap inputs, not a bitmap canvas. STL/OBJ are mesh inputs.

## Domain layers

| Layer | Responsibility | Allowed dependencies |
|---|---|---|
| Geometry foundation | Units, 2D regions/loops/segments, curves, heightfields, triangle meshes, spatial acceleration | Mature geometry crates and small Gravita adapters |
| Document | Project state, relief stacks, imported geometry references, tools, operations, persistence model | Geometry foundation |
| Relief kernel | Heightfield operators and mesh/heightfield conversion | Geometry, document types |
| CAM kernel | 2D and 3D toolpath planning, tool models, offsets, pocket topology, V-carve, drop-cutter | Geometry, document types, vetted CAM/contour crates |
| Simulation and safety | Stock removal, collision, gouge, bounds and process validation | Geometry, CAM, tool database |
| I/O | DXF/SVG, PNG/TIFF, STL/OBJ, native project container, G-code read/write | Domain types; format libraries |
| Engine | Typed command bus, session transactions, undo/redo, orchestration | Domain crates, I/O; no UI |
| Frontends | egui desktop, CLI, MCP | Engine public API only |
| Tooling | xtask, audits, generators, quality gates | Build/dev dependencies only |

Dependency direction is inward-to-outward. Domain crates must not depend on egui, wgpu, winit, CLI, MCP, or application crates. UI event handlers may create typed command requests but must not mutate document state directly.

## Command architecture

There is one command execution model and one authoritative dispatcher. CLI, desktop UI, and MCP are adapters over the same typed command API. A command has a stable ID, typed payload, validation, execution result, and explicit effects. JSON is an interchange boundary, not the internal domain model. Command metadata and generated schemas must be derived from or checked against typed command definitions; handwritten parallel registries are forbidden.

Transactions own mutations. A failed command must not leave partially applied document changes. Undo/redo is defined over domain transactions, not GUI widgets. Every public command is exercised through the same engine entry point used by frontends.

## Reuse policy

Do not recreate mature algorithms just to keep the code self-contained. Evaluate candidate Rust crates by: API semantics against the domain requirement; maintenance/activity; licensing; unsafe surface and dependency tree; numeric robustness; topology handling; determinism; performance; and testability. Wrap selected crates behind narrow Gravita interfaces so the domain model is not coupled to a transient implementation. Candidate areas include analytic offsets/contours, polygon boolean operations, Voronoi/medial-axis, mesh BVH/raycasting, heightfield processing, and G-code parsing/post-processing. A crate is not approved merely because it appears in the old Cargo manifest or roadmap.

## Numerical and physical conventions

- Define one internal length unit (millimetres) and explicit conversion at file/controller boundaries.
- Keep geometric tolerance, machining tolerance, raster sample spacing, and simulation resolution as distinct named quantities.
- Preserve units in typed parameters; do not pass unlabelled strings for physical values.
- Define coordinate handedness, XY origin, Z-positive convention, row/column orientation, and toolpath arc direction centrally and test them with fixtures.
- Reject non-finite values, invalid tool geometry, impossible depths, and unsafe Z transitions before post-processing.
- CAM acceptance requires physical invariants and independent reference fixtures, not merely non-empty output.

## Rust quality constraints

Safe Rust only (`unsafe_code = "forbid"`), no panic-based domain error handling, explicit `Result` propagation, strict Clippy, formatting, deterministic tests where practical, and no silent fallback that changes machining intent.

## Status vocabulary

- **Source-confirmed:** observed in the previous repository code or schema.
- **Specified:** documented here but not yet implemented in this repository.
- **Implemented:** present in this repository and compiling.
- **Verified:** acceptance tests demonstrate the documented behavior with real domain data.
- **Deferred / Excluded:** explicitly not in current product scope.

Never carry over an old roadmap percentage as Gravita progress.
