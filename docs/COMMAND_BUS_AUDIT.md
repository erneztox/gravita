# Core Bus audit against the Gravita product scope

**Status:** selected contract audited against the current architecture and WBS; implementation pending.

## Selection result

| Area | Operations |
|---|---:|
| Document and history | 11 |
| Relief modeling and layers | 43 |
| Imported machining geometry | 1 |
| CAM and tool management | 31 |
| Simulation | 7 |
| Post-processing | 2 |
| Safety requirements | 2 |
| **Total** | **97** |

Of these, 93 are backed by executable handlers in the previous Gravita Rust engine and 4 are new-product requirements absent from that engine. The contract contains 457 parameter entries. It records handler aliases so multiple legacy entry points can map to one canonical operation in the new API.

## Scope covered

- **Document:** create, open, save, save-as, close, resize, undo/redo and explicit transaction lifecycle.
- **Input:** DXF/SVG geometry, PNG relief heightmaps, STL/OBJ meshes, and native project persistence.
- **Relief:** shape generation from imported geometry, extrusion, two-rail sweep, spin/turn, height transforms, smoothing/fading/draft/slicing, sculpt strokes, volume/surface calculations, mesh generation and export.
- **Relief stacks:** create/select/delete/duplicate/rename, visibility, ordering, combine modes, merge and composite calculation.
- **CAM:** tool database, tool CRUD/groups, feeds/speeds, LinuxCNC tool tables, profile, pocket, V-carve, drilling and hole banks, Z-level roughing, 3D relief finishing, rest machining, toolpath list/get/delete/merge/transform/panelling and metrics.
- **Simulation:** single/all-toolpath stock simulation, reset/delete, collision checks, gouge detection and feed optimization.
- **Post-processing:** controller dialect discovery and G-code generation.

## Required additions beyond the previous engine

| Requirement | Why it is new |
|---|---|
| TIFF heightmap input | Previous relief importer reads PNG; Gravita's G3.2 scope explicitly requires PNG and TIFF. |
| Toolpath sequencing | Previous engine merges/transforms/panels paths but has no explicit sequence/safe-transition operation (WBS G5.8). |
| Machine travel bounds | Previous collision checker does not define a machine travel envelope check (WBS G6.2). |
| Mandatory export validation | Previous G-code export does not require a passing safety report; Gravita must enforce the WBS G7.4 release gate. |
| G-code validation | Previous postprocessor generates output but has no explicit parsed G-code validation stage (WBS G7.4). |

The four new operations are explicitly marked `new_product_requirement` in the contract. TIFF is recorded as an additional required format on the heightmap import operation. Offset relief finishing is also required by WBS G5.6, but the previous handler only implements raster and spiral; the new CAM kernel must add offset finishing. The G-code export contract requires both `safety.export.validate` and `postprocessor.gcode.validate` to pass before the file is written.

## Excluded legacy functionality

Interactive vector editing and vector-layer operations; bitmap editing and palette/image operations; vector/bitmap conversions; UI/view/window/dialog commands; clipart and assembly workbenches; and legacy CAM wizards not present in the product WBS.

The previous shape-generation command mixes geometry-driven generation with a bitmap-color mode. Only the geometry-driven path is selected. The previous `cam.feature_machining_3d` handler returns `Disabled`, so it is not treated as implemented functionality.

The legacy postprocessor supports GRBL, LinuxCNC, Mach3, Mach4, Fanuc and Haas, plus generic ISO G-code. This satisfies the six controller families named by WBS G7.2/G7.3.

## Conclusion

The selected contract is a sound functional baseline for the product as currently specified. It does not require carrying the previous editor workbenches into Gravita. The remaining work is to implement the four new requirements and TIFF support, then verify each operation with the WBS acceptance fixtures. The contract remains a candidate until the new Rust implementation compiles and passes those tests.


## Execution-model findings

The previous engine exposes one `Session::execute` entry point. It routes requests through the document, vector, bitmap, relief, CAM and simulation handlers. The selected Gravita contract keeps the domain operations needed by the new product and removes the vector/bitmap editing handlers.

Mutating calls are normally wrapped in an automatic history transaction. A returned error triggers rollback; the previous engine also snapshots several session components and catches panics to restore state. Gravita should preserve failure atomicity through its typed transaction model and ordinary `Result` propagation, without copying panic-based control flow.

The old `schema.rs` reads a generated JSON registry for schema introspection (`command_schema` / `command_signature`). The execution path itself dispatches to handlers that read JSON fields directly; it does not validate every call against that generated schema first. That is why this contract is derived from handler behavior rather than copying the old generated schema file.

The workflow runner executes ordered steps by calling the same `Session::execute` entry point and collects generated artifacts. Gravita should preserve that single execution path for desktop, CLI, MCP and programmatic workflows, with typed payloads at the engine boundary.


## Legacy behavior that must not be copied

The previous PNG heightmap I/O supports 8-bit and 16-bit grayscale, plus RGB/RGBA conversion. Gravita still needs TIFF decoding.

Some legacy handler defaults are unsuitable for a production contract:

- Heightmap import without a path or byte buffer re-exports and re-imports the current active relief.
- Mesh import without a path or bytes synthesizes a sample tetrahedron; the same handler also creates a clipart asset and instance before rasterizing the mesh.
- Unknown 3D finishing strategies fall back to raster.
- Unknown G-code dialect names fall back to Mach3.
- Collision checking defaults gouge checking to off unless explicitly requested.

New Gravita should require real import input, store meshes in its own document model, reject unsupported strategy/dialect values, and make gouge, holder and machine-bound checks mandatory before export. Mesh-to-heightfield sampling remains in scope, but clipart and assembly behavior does not.
