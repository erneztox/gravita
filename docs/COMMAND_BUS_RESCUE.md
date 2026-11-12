# Rescuing the previous Gravita Core Bus

This work uses the previous Gravita engine as an engineering reference. Its command bus was implemented in Rust; it is not an ArtCAM command bus.

## Scope

The previous engine is inspected to understand the operations it implemented, their behavior, their inputs and how the application invoked them. The new Gravita product deliberately keeps only the operations required by its current product definition.

The rescued contract in `docs/contracts/rescued-commands.json` is therefore a **selected subset**, not a port of the previous command registry. It covers document persistence, relief modeling and layers, imported geometry and meshes, tool management, CAM, simulation and G-code post-processing.

Interactive vector authoring and bitmap editing are excluded. The geometry import operation remains because the new product accepts imported DXF/SVG geometry as CAM input; it does not expose vector drawing, node editing or a bitmap workbench.

## Functional contract

Each operation records:

- its stable Gravita command ID and human-readable purpose;
- the corresponding command handled by the previous Rust Core Bus;
- the Rust handler source file;
- the parameter names and JSON value types read by that handler;
- the source expression used to identify each parameter.

The parameter list is based on the handler implementation, not on ArtCAM registry metadata or generated ArtCAM schemas. Where the old handler accepts dynamic JSON values, the contract preserves that boundary as `Any`, `Array` or `Object` instead of inventing a more specific type.

The contract is a product decision: operations outside the new Gravita scope are not required to be implemented.

## New implementation audit

`cargo xtask audit rescued-commands` checks the selected contract against the new Rust source tree. It reports missing command IDs, duplicate IDs, missing typed payloads, missing parameters, type-family mismatches, undocumented payload fields and missing dispatcher registrations.

The new implementation is not required to reproduce the previous engine's internal structure. The audit checks the new command API and its authoritative dispatcher against the selected functional contract.

Run:

```sh
cargo xtask audit rescued-commands
cargo xtask audit rescued-commands --json target/audits/rescued-commands.json
```

The audit is expected to report pending operations while the new engine implementation is still absent. That is a baseline, not an implementation claim.
