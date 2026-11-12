# Work breakdown structure (WBS)

This is the implementation plan for the product defined in ARCHITECTURE.md. Every item starts **Specified**. Mark an item Implemented only when code exists in this repository; Verified requires acceptance evidence.

| ID | Work package / deliverable | Depends on | Acceptance evidence | Status |
|---|---|---|---|---|
| G0.1 | Repository policy, CI baseline, license and dependency policy | — | Clean checkout; fmt, Clippy, tests and documentation checks run in CI | Specified |
| G0.2 | Crate graph and dependency-layer audit | G0.1 | xtask rejects forbidden dependency edges and frontend leaks | Specified |
| G0.3 | Command IDs, payloads, results, errors and transaction conventions | G0.1 | Typed command registry and schema parity tests | Specified |
| G1.1 | Geometry adapter layer: units, points, bounds and curves | G0.2 | Unit and coordinate fixtures; selected crate wrappers | Specified |
| G1.2 | Region topology: outer loops, holes/islands, line/arc/curve segments | G1.1 | DXF/SVG fixtures preserve topology and orientation | Specified |
| G1.3 | Heightfield representation, sampling and interpolation | G1.1 | Known planes, ramps, peaks, bounds and resampling fixtures | Specified |
| G1.4 | Triangle mesh representation, validation and acceleration | G1.1 | STL/OBJ fixtures, degenerate-face and ray-hit tests | Specified |
| G2.1 | Native document/project model and relief stacks | G1.2-G1.4 | Create, mutate, serialize, reload and compare semantic state | Specified |
| G2.2 | Transaction journal, undo/redo and rollback | G2.1,G0.3 | Multi-command transaction and injected-failure rollback tests | Specified |
| G3.1 | DXF/SVG ingestion to canonical regions | G1.2,G2.1 | Real imports, units, arcs, nested holes and malformed-input tests | Specified |
| G3.2 | PNG/TIFF heightmap import (8/16-bit) | G1.3,G2.1 | Pixel-to-height calibration, orientation and no-data tests | Specified |
| G3.3 | STL/OBJ import and mesh-to-relief sampling | G1.4,G1.3,G2.1 | Real mesh fixture, ray/drop sampling, bounds and orientation tests | Specified |
| G4.1 | Relief shape generation and combine modes | G1.3,G2.1 | Plane/cone/sphere fixtures; add/subtract/max/min invariants | Specified |
| G4.2 | Extrude, spin/turn and two-rail sweep | G1.2,G1.3,G4.1 | Geometric reference fixtures and numerical tolerances | Specified |
| G4.3 | Smooth, scale, invert, fade, draft, slice and sculpt operations | G1.3,G2.2 | Boundary, parameter and determinism tests per operation | Specified |
| G4.4 | Relief compositing and closed mesh export | G1.3,G1.4,G4.1 | Manifold checks where required and mesh-bound tests | Specified |
| G5.1 | Tool model, material/process data and tool database | G2.1 | Typed tool geometry; serialization and feeds/speeds validation | Specified |
| G5.2 | 2D profile with inside/outside/on, tabs, ramps and stepdowns | G1.2,G5.1 | Offset and cut-side fixtures; depth schedule; no gouging | Specified |
| G5.3 | Pocketing with nested islands and residual cleanup | G1.2,G5.1 | Nested-region fixtures; complete machinable-area coverage | Specified |
| G5.4 | V-carve and medial-axis depth mapping | G1.2,G5.1 | Independent fixtures for cutter angle and channel width | Specified |
| G5.5 | Drilling and hole-pattern cycles | G1.2,G5.1 | Standard, peck, chip-break and dwell path assertions | Specified |
| G5.6 | 3D roughing and relief finishing (raster/offset/spiral) | G1.3,G1.4,G5.1 | Surface fixtures; scallop, allowance and stepover invariants | Specified |
| G5.7 | Rest machining and multi-tool sequencing | G5.6,G5.1 | Prior-tool stock model and residual-material coverage tests | Specified |
| G5.8 | Toolpath transforms, merge, panelling and sequencing | G5.2-G5.7 | Geometric equivalence, continuity, bounds and safe transitions | Specified |
| G6.1 | Stock model and material-removal simulation | G5.2-G5.8 | Deterministic stock deltas and cut-volume fixtures | Specified |
| G6.2 | Collision, gouge, holder and machine-bound checks | G6.1,G5.1 | Deliberate collision and safe-path fixtures; false-negative tests | Specified |
| G6.3 | Cycle-time and feed optimization | G6.1,G5.1 | Known path timing and bounded optimizer regression tests | Specified |
| G7.1 | Postprocessor architecture and dialect capability model | G5.2-G5.8 | Dialect conformance fixtures; unsupported features fail explicitly | Specified |
| G7.2 | GRBL and LinuxCNC postprocessors | G7.1 | Golden G-code fixtures and parser round-trip checks | Specified |
| G7.3 | Mach3/Mach4, Fanuc and Haas postprocessors | G7.1 | Golden fixtures per dialect and explicit capability tests | Specified |
| G7.4 | G-code safety validation and export package | G6.2,G7.2,G7.3 | Unsafe/unverified paths cannot be exported; reproducible output | Specified |
| G8.1 | Typed command bus and public session API | G0.3,G2.2,G3-G7 | All frontends dispatch through one engine path | Specified |
| G8.2 | CLI for import, relief, CAM, simulation and export | G8.1 | Headless end-to-end craft using real files and kernels | Specified |
| G8.3 | MCP adapter over the same command API | G8.1 | Protocol tests invoke the same commands and return typed results | Specified |
| G8.4 | egui desktop shell, project tree, parameter panels and viewport | G8.1 | UI actions dispatch commands; no direct document mutation | Specified |
| G9.1 | Programmatic craft example and reusable test harness | G1-G8 | Example creates document, builds/imports relief, computes CAM, simulates and exports without mocks | Specified |
| G9.2 | Golden files, adversarial validation and regression suite | G3-G7 | Real fixture corpus and reproducible numerical baselines | Specified |
| G9.3 | Performance profiling and large-model limits | G1-G7 | Published datasets and performance budgets | Specified |
| G10.1 | Release packaging, Linux distribution and user documentation | G0-G9 | Reproducible build and installation test on supported Linux targets | Specified |

## Command contract requirements

The command API is designed from Gravita's domain workflows. Each command must have a canonical ID, typed payload, validation rules, transactional effects, typed result, frontend exposure policy and acceptance tests. The registry and JSON schemas must be generated from or mechanically checked against the same typed definitions. Every command must map to one or more WBS items; orphan commands and undocumented public commands fail the repository audit.

## Release gates

1. Geometry and document gates pass before relief work is integrated.
2. Relief operations pass domain acceptance before CAM consumes them.
3. CAM operations require validated topology, tool geometry and reference fixtures.
4. Simulation and safety checks must pass before a toolpath can be marked exportable.
5. Postprocessors reject unsupported semantics rather than silently flattening them.
6. The end-to-end craft runs through the public engine API and produces inspectable artifacts.
