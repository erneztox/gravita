# Gravita

Gravita is a Rust desktop CAD/CAM application for artistic CNC relief production.

The product scope is relief-centric: 2.5D heightfield modeling, imported 2D geometry, relief and mesh operations, 2.5D/3D toolpath generation, stock simulation, machining safety checks, and G-code post-processing.

This repository is the clean implementation workspace. It does not inherit the history or source tree of the earlier `artcam-investigation` repository.

## Current status

The repository is being established from a source audit of [erneztox/artcam-investigation](https://github.com/erneztox/artcam-investigation), with the approved architecture pivot treated as authoritative. The first engineering baseline is documented under `docs/`.

- [Architecture](docs/ARCHITECTURE.md)
- [Work breakdown structure](docs/WBS.md)
- [Command contract inventory](docs/COMMANDS.md)
- [Command schema snapshot](docs/reference/source-command-schemas.json)
- [xtask specification](docs/XTASK.md)
- [Programmatic vertical slice](docs/PROGRAMMATIC_CRAFT.md)
- [Source audit and decisions](docs/SOURCE_AUDIT.md)
- [Agent instructions](AGENTS.md)

## Non-goals

- Interactive vector drawing or node editing.
- Bitmap painting or image-editing tools.
- Parametric sketching or a general-purpose solid CAD workbench.
- 3D printing workflows.
- Recreating ArtCAM's original project-file compatibility.

DXF and SVG are import formats for 2D CAM geometry. PNG/TIFF are heightmap inputs. STL/OBJ are mesh inputs.

## Engineering constraints

- Rust workspace; safe Rust only (`unsafe_code = "forbid"`).
- No panic-based domain control flow.
- Strict Clippy and formatting gates.
- One typed command execution model shared by GUI, CLI, and MCP.
- Real domain data in acceptance tests; no mocked geometry, toolpaths, simulation, or post-processing.
- Reuse mature Rust geometry/CAM crates when their semantics and maintenance status fit; do not build replacement algorithms merely for ownership.

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the authoritative design and [docs/WBS.md](docs/WBS.md) for implementation sequencing.
