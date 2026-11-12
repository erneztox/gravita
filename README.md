# Gravita

Gravita is a pure-Rust desktop application for artistic CNC relief design and machining.

Its product scope is deliberately focused:
- 2.5D heightfield relief modeling and 3D relief operations.
- Imported 2D geometry for machining.
- Imported heightmaps and triangle meshes.
- 2.5D and 3D CAM toolpath generation.
- Stock-removal simulation and machining safety validation.
- Controller-specific G-code post-processing.
- A native project/document format.

## Project status

Gravita is in its architecture and implementation-baseline phase. The repository currently contains product specifications; Rust implementation has not yet started.

- [Architecture](docs/ARCHITECTURE.md)
- [Work breakdown structure](docs/WBS.md)
- [Programmatic end-to-end craft](docs/PROGRAMMATIC_CRAFT.md)
- [xtask specification](docs/XTASK.md)
- [Agent instructions](AGENTS.md)

## Input formats

- **DXF/SVG:** imported 2D regions, loops and curves for CAM.
- **PNG/TIFF:** imported grayscale heightmaps.
- **STL/OBJ:** imported triangle meshes and relief sources.

These are data-ingestion paths, not interactive editing workbenches.

## Out of scope

- Interactive vector drawing, node editing or vector-illustration tools.
- Bitmap painting, pixel editing or image-manipulation tools.
- General-purpose parametric solid CAD or sketch workbenches.
- 3D-printing workflows.
- Machine-control connectivity and controller streaming.

## Engineering constraints

- Rust workspace; safe Rust only (unsafe_code = forbid).
- No panic-based production control flow.
- Typed errors, strict Clippy and formatting gates.
- One typed command execution model shared by desktop UI, CLI and MCP.
- Domain crates remain independent of UI and frontend frameworks.
- Real geometry, toolpaths, simulation and post-processing in acceptance tests.
- Reuse mature Rust algorithms behind narrow domain interfaces; do not reimplement established algorithms without a concrete technical reason.

The product definition and architecture in this repository are authoritative. Do not introduce functionality outside this scope without an explicit product decision.
