# Recovering and rescuing the legacy command bus

This work has two distinct outputs and a one-way flow:

1. `inventory/legacy-command-bus.json` records the complete old engine Bus and how callers used it.
2. `contracts/rescued-commands.json` defines the smaller set of operations selected for the new Gravita product. It is a new functional contract, not a copy of the old registry.
3. `cargo xtask audit rescued-commands` checks that the new typed Rust commands, their payload fields and their registration in the authoritative dispatcher match that contract.

## What was extracted

Source repository: [erneztox/artcam-investigation](https://github.com/erneztox/artcam-investigation), branch `main`.

The old engine exposes `Session::execute(command, params)`. It resolves canonical dotted IDs, numeric command IDs and `ID_*` source symbols, then dispatches in this order: document, vector, bitmap, relief, CAM and simulation.

The session automatically wraps ordinary commands in a history transaction when no transaction is active. It commits successful execution, rolls back returned errors and restores snapshots if execution panics. History and read/query operations are excluded from automatic transactions by an explicit allowlist.

The full extraction contains **404 unique registry entries** and **116 generated parameter schemas**: 111 match registered commands and 5 are orphan schema entries. The inventory preserves the complete schema objects, registry metadata, handler-module references, source file SHAs, command resolution and transaction behavior. A missing schema is recorded as `not_described`; it is never interpreted as proof that a command takes no parameters. Registry status is preserved as source metadata, not treated as proof of behavior.

## Rescue contract

The first rescue contract contains **63 operations** from document/project persistence, 2D geometry ingestion, mesh ingestion/export, relief construction and manipulation, relief-layer control, tool management, CAM profile/pocket/drilling/3D machining, toolpath management, simulation and G-code post-processing.

It deliberately excludes interactive vector authoring, bitmap editing and presentation/UI commands. The 2D geometry import operation is retained because Gravita needs imported DXF/SVG geometry as CAM input; that does not bring back a vector editor.

Where a generated legacy schema exists, parameter names, types, required flags and defaults are carried forward. For important operations without generated schemas (geometry import, mesh import/export, profile/pocket generation, simulation and G-code export), parameters were extracted from the actual handler code. The new IDs are normalized and do not have to match old command IDs. Each operation retains its legacy ID, handler module and parameter provenance.

This is the initial rescue contract, not a claim that every useful legacy operation has already been selected. The complete inventory makes the remaining candidates visible for deliberate inclusion. In particular, the five orphan schemas are preserved separately and are not silently attached to unrelated commands.

## New implementation convention audited by xtask

A rescued operation declares its stable ID in the `Command` trait implementation and its parameters in a typed payload:

```rust
pub struct SmoothRelief {
    pub strength: f64,
}

impl Command for SmoothRelief {
    const ID: &'static str = "relief.height.smooth";
    // typed validation and execution contract
}
```

The authoritative engine dispatcher registers it explicitly:

```rust
CommandBus::register::<SmoothRelief>();
```

The audit verifies:
- every contract ID has an `impl Command` with the matching associated `ID`;
- the command has a typed Rust payload;
- the authoritative dispatcher registers the payload;
- every declared parameter exists with a compatible Rust type;
- no undocumented payload field silently expands the contract.

Run:

```sh
cargo xtask audit rescued-commands
cargo xtask audit rescued-commands --json target/audits/rescued-commands.json
```

The clean Gravita repository currently contains product/architecture documents only, not application crates. Therefore this first audit baseline is expected to fail with all 63 operations missing. That is an accurate baseline: it does not manufacture implementation evidence or mark specified work as implemented.
