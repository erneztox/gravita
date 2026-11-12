# Recovering and rescuing the legacy command bus

This work has two distinct outputs and a one-way flow:

1. `inventory/legacy-command-bus.json` records the complete old engine Bus and how callers used it.
2. `contracts/rescued-commands.json` defines the smaller set of operations selected for the new Gravita product. It is a new functional contract, not a copy of the old registry.
3. `cargo xtask audit rescued-commands` checks that the new typed Rust commands, their payload fields and their registration in the authoritative dispatcher match that contract.

## What was extracted

Source repository: [erneztox/artcam-investigation](https://github.com/erneztox/artcam-investigation), branch `main`.

The old engine exposes `Session::execute(command, params)`. It resolves canonical dotted IDs, numeric command IDs and `ID_*` source symbols, then dispatches in this order: document, vector, bitmap, relief, CAM and simulation.

The session automatically wraps ordinary commands in a history transaction when no transaction is active. It commits successful execution, rolls back returned errors and restores snapshots if execution panics. History and read/query operations are excluded from automatic transactions by an explicit allowlist.

The full extraction contains **404 unique registry entries** and **116 generated parameter schemas**. A missing legacy schema is recorded as `not_described`; it is never interpreted as proof that a command takes no parameters. Registry status is preserved as source metadata, not treated as proof of behavior.

## Rescue contract

The first rescue contract contains **53 operations** from document sizing/opening, relief construction and manipulation, relief-layer control, tool management, CAM generation and toolpath management, and simulation checks. It excludes interactive vector authoring, bitmap editing and presentation/UI commands.

The selected operations have generated legacy schemas, so their parameter names, types, required flags and defaults are grounded in the old engine's schema file. The new IDs are deliberately normalized and do not have to match old command IDs. Each operation retains its legacy ID and handler module as provenance.

This is a first contract baseline, not a claim that the old schema file covered every useful operation. The complete inventory makes the uncovered commands visible for a later deliberate addition; no parameterless signature is inferred from a missing schema.

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
- every contract ID has exactly one `impl Command` with the matching associated `ID`;
- the command has a typed Rust payload;
- the authoritative dispatcher registers the payload;
- every declared parameter exists with a compatible Rust type;
- no undocumented payload field silently expands the contract.

Run:

```sh
cargo xtask audit rescued-commands
cargo xtask audit rescued-commands --json target/audits/rescued-commands.json
```

The clean Gravita repository currently contains product/architecture documents only, not application crates. Therefore this first audit baseline is expected to fail with all 53 operations missing. That is an accurate baseline: it does not manufacture implementation evidence or mark specified work as implemented.
