# Gravita architecture

**Status:** product and engineering baseline.

## Product definition

Gravita is a Rust desktop application for artistic CNC relief design and manufacture. Its core workflow is:

1. Import external 2D geometry (DXF/SVG), heightmaps (PNG/TIFF) and triangle meshes (STL/OBJ).
2. Build and edit 2.5D reliefs on heightfield data; combine, transform, sweep, turn, sculpt, smooth, draft, slice and inspect reliefs.
3. Prepare 2D regions and 3D surfaces for CAM.
4. Generate 2.5D and 3D toolpaths: profile, pocket, V-carve, drill, Z-level roughing, raster/offset/spiral relief finishing, rest machining, tabs, ramps and toolpath management.
5. Simulate stock removal and run collision, gouge and machining-safety checks.
6. Post-process validated toolpaths to controller-specific G-code.
7. Save and load the native Gravita project.

## Product boundaries

Gravita is not an interactive vector editor, bitmap editor, general-purpose parametric CAD system, sketch workbench or 3D-printing application. DXF/SVG provide imported geometry; PNG/TIFF provide heightmap data; STL/OBJ provide mesh data. None of these formats imply interactive authoring tools for their source domains.

Machine-control connectivity, controller streaming, DRM/dongle emulation and external application integrations are not part of the product.

## Domain layers

| Layer | Responsibility | Allowed dependencies |
|---|---|---|
| Geometry foundation | Units, 2D regions/loops/segments, curves, heightfields, triangle meshes, spatial acceleration | Mature geometry crates and Gravita adapters |
| Document | Project state, relief stacks, imported geometry references, tools, operations, persistence model | Geometry foundation |
| Relief kernel | Heightfield operators and mesh/heightfield conversion | Geometry and document types |
| CAM kernel | 2D/3D toolpath planning, tool models, offsets, pocket topology, V-carve, drop-cutter | Geometry, document types, vetted CAM/contour crates |
| Simulation and safety | Stock removal, collision, gouge, bounds and process validation | Geometry, CAM, tool database |
| I/O | DXF/SVG, PNG/TIFF, STL/OBJ, native project container, G-code | Domain types and format libraries |
| Engine | Typed command bus, session transactions, undo/redo and orchestration | Domain crates and I/O; no UI |
| Frontends | egui desktop, CLI and MCP | Engine public API only |
| Tooling | xtask, audits, generators and quality gates | Build/development dependencies only |

Dependency direction is inward-to-outward. Domain crates must not depend on egui, wgpu, winit, CLI, MCP or application crates. UI event handlers may create typed command requests but must not mutate document state directly.

## Command architecture

There is one command execution model and one authoritative dispatcher. CLI, desktop UI and MCP are adapters over the same typed command API. A command has a stable ID, typed payload, validation, execution result and explicit effects. JSON is an interchange boundary, not the internal domain model. Command metadata and generated schemas must be derived from or checked against typed command definitions; handwritten parallel registries are forbidden.

Transactions own mutations. A failed command must not leave partially applied document changes. Undo/redo is defined over domain transactions, not GUI widgets. Every public command is exercised through the same engine entry point used by frontends.

## Algorithm and dependency strategy

Do not recreate mature algorithms merely to keep the code self-contained. Evaluate candidate Rust crates by API semantics, maintenance, licensing, unsafe surface, dependency tree, numerical robustness, topology handling, determinism, performance and testability. Wrap selected crates behind narrow Gravita interfaces so the domain model is not coupled to a transient implementation.

Evaluate libraries for polygon booleans and offsets, Voronoi/medial-axis, mesh BVH/raycasting, heightfield processing, toolpath planning and G-code parsing/post-processing. A crate is not approved merely because it is popular; suitability must be demonstrated against Gravita fixtures.

## Numerical and physical conventions

- Use millimetres as the internal length unit and convert explicitly at file/controller boundaries.
- Keep geometric tolerance, machining tolerance, raster sample spacing and simulation resolution as distinct named quantities.
- Preserve units in typed parameters; do not pass unlabelled strings for physical values.
- Define coordinate handedness, XY origin, Z-positive convention, row/column orientation and toolpath arc direction centrally and test them with fixtures.
- Reject non-finite values, invalid tool geometry, impossible depths and unsafe Z transitions before post-processing.
- CAM acceptance requires physical invariants and independent reference fixtures, not merely non-empty output.

## Rust quality constraints

Safe Rust only (unsafe_code = forbid), no panic-based domain error handling, explicit Result propagation, strict Clippy, formatting, deterministic tests where practical and no silent fallback that changes machining intent.

## Status vocabulary

- **Specified:** documented but not yet implemented.
- **Implemented:** present in this repository and compiling.
- **Verified:** acceptance tests demonstrate the documented behavior with real domain data.
- **Deferred / Excluded:** explicitly outside the current implementation phase or product scope.

Never report a feature as implemented based only on a command name, UI label, schema or TODO.
