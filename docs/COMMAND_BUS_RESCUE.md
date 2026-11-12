# Gravita command contract and Core Bus audit

This audit uses the previous Gravita engine as a Rust implementation reference. The previous Core Bus is not a porting target: the new product keeps only operations required by its own architecture and WBS.

## Result

The selected contract contains 97 canonical operations:

- 93 operations grounded in executable handlers from the previous Gravita Rust engine.
- 4 explicit new-product requirements where the previous engine did not provide the required behavior.
- 457 parameter entries, derived from the handler branches or defined explicitly for new requirements.

The contract is in `docs/contracts/rescued-commands.json`. It records source handler files and all legacy command aliases for each selected operation. It does not reproduce the previous engine's full command catalog.

## Product boundary

Included workflows are document lifecycle and transactions; DXF/SVG geometry ingestion; PNG heightmap ingestion/export; STL/OBJ mesh ingestion/export; relief generation, transformation, sculpting and layer compositing; tool database and stock setup; 2D/3D CAM; toolpath management; simulation; machining safety; and G-code post-processing.

DXF/SVG remain input formats for machining geometry. The new product does not expose vector authoring, node editing, vector Boolean/trim/offset tools, vector text, or vector/bitmap conversion. PNG/TIFF are heightmap inputs, not a bitmap editing workbench.

The previous shape-generation handler contains both geometry-driven and bitmap-color modes. The contract retains only the geometry-driven path.

## Coverage gaps carried into the new product

The previous engine is a useful starting point, but it does not fully satisfy the current product specification:

1. **TIFF heightmaps:** the old relief importer reads PNG. Gravita must add TIFF ingestion in its I/O layer.
2. **Toolpath sequencing:** the old engine can merge, transform and panel toolpaths, but has no explicit operation for sequencing and safe transitions.
3. **Machine travel bounds:** the old collision checker covers tool/holder collisions and optional gouge checks, not a configured machine travel envelope.
4. **Mandatory export gate:** the old G-code command generates output without requiring a successful safety report. Gravita must block export until stock, collision, gouge and machine-bound validation passes.
5. **G-code validation:** the old postprocessor generates dialect-specific output but does not expose the G-code parser/validation stage required by WBS G7.4.
6. **Offset relief finishing:** the old 3D relief finishing handler supports raster and spiral strategies; Gravita's WBS G5.6 also requires offset finishing.

These are represented as four new operations in the contract: toolpath sequencing, machine-bounds checking, pre-export safety validation and generated-G-code validation. TIFF is recorded as an additional required format on the heightmap import operation, and offset relief finishing is recorded as a new required strategy on the existing 3D finishing operation. G-code export is explicitly blocked until both safety validation and parsed G-code validation pass.

## Deliberate exclusions

- Interactive vector creation/editing, node manipulation, transforms, Boolean operations, trimming, nesting, text and vector-layer management.
- Bitmap layers, painting, brushes, flood fill, palette/color operations, bitmap filters, and vector/bitmap conversion.
- View, window, dialog and layout commands.
- Clipart and assembly workbenches.
- Legacy bevel, inlay and smart-engraving wizards not included in the current WBS.
- The old `cam.feature_machining_3d` branch is disabled in the source and is not counted as implemented functionality.
- `cam.profile_options` is not a separate operation: bridge, lead-in/out and ramp parameters are already part of profile toolpath creation.

## Postprocessor coverage

The previous postprocessor has dialect implementations for GRBL, LinuxCNC, Mach3, Mach4, Fanuc and Haas, plus a generic ISO mode. This covers the six controller families named in WBS G7.2/G7.3.

## New implementation audit

`cargo xtask audit rescued-commands` checks the selected contract against typed Rust command payloads, execution implementations and dispatcher registration. It intentionally audits only this selected contract; excluded legacy operations are not porting failures.

The current Gravita repository is still at its specification baseline and has no application crates, so the audit will report the selected commands as pending until the implementation is added. The contract is a product baseline, not a claim that the engine has already been implemented or verified.


## How the previous command bus is used

The previous engine has a single `Session::execute` entry point that dispatches to document, vector, bitmap, relief, CAM and simulation handlers. The selected contract keeps the relevant domain behavior but does not carry the editor handlers into Gravita.

Mutating commands are normally grouped in automatic history transactions. Errors trigger rollback, and the previous implementation additionally snapshots session state to recover from panics. The new engine should preserve atomic command execution using the typed transaction system and `Result` propagation rather than copying panic recovery as normal control flow.

The previous workflow runner executes each declared step through that same session entry point. Gravita's desktop, CLI, MCP and programmatic workflows should likewise share one typed engine API.
