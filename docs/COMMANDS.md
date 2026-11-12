# Command contract inventory

## Source and interpretation

This document is generated from the exact 116-entry schema snapshot at [reference/source-command-schemas.json](reference/source-command-schemas.json), fetched from `artcam-investigation/crates/engine/src/cmd/generated_schemas.json`. It preserves each command ID, label, parameter name, declared type, constraints/choices, default, and required flag. It is a source-contract inventory, **not proof that every entry is implemented or belongs in Gravita's product scope**.

The previous source has command execution handlers in `crates/engine/src/cmd/{relief,cam,sim,doc,bitmap,vector}.rs`, with shared parameter readers in `common.rs`; the command registry is separate in `registry.rs`. Gravita must map each retained command to its actual handler and tests before claiming implementation. Command aliases in the old registry are not separate product capabilities.

## Schema counts

- Total schema entries: **116**
- By namespace: `bitmap` 4; `bitmap_layer` 1; `cam` 30; `clipart` 5; `file` 1; `history` 1; `model` 1; `project` 7; `relief` 21; `relief_layer` 6; `vector` 36; `vector_layer` 3
- Source snapshot SHA-256 is not asserted here; Git blob identity is recorded by the source repository.

## Scope classification

- `bitmap.*`, `bitmap_layer.*`: **excluded** (pixel painting/flood-fill/editor behavior).
- `vector.*`: **excluded as interactive editor commands**. Vector ingestion is retained as file I/O, not as a command-driven drawing workbench.
- `cam.*`: candidate core. Retain only after reconciling each contract with actual handler semantics and the approved 2.5D/3D CAM scope.
- `relief.*`, `relief_layer.*`, `clipart.*`, `project.*`: candidate relief/document operations. Clipart/assembly semantics require explicit review against the relief-centric scope; do not assume all are accepted.
- `file.*`, `history.*`, `model.*`: infrastructure candidates; preserve only where needed for native project/document lifecycle.

## Complete parameter contracts

### `bitmap.brush_stroke` — Brush Stroke

- `color`: String; optional
- `colour`: String; optional
- `radius`: Int {"min":0,"max":null}; default=2; optional
- `x0`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `x1`: Float {"min":null,"max":null,"step":1,"unit":null}; optional
- `y0`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `y1`: Float {"min":null,"max":null,"step":1,"unit":null}; optional

### `bitmap.flood_fill` — Flood Fill

- `color`: String; optional
- `colour`: String; optional
- `x`: Int {"min":0,"max":null}; default=0; optional
- `y`: Int {"min":0,"max":null}; default=0; optional

### `bitmap.set_primary_colour` — Set Primary Colour

- `color`: String; optional
- `colour`: String; optional

### `bitmap.set_secondary_colour` — Set Secondary Colour

- `color`: String; optional
- `colour`: String; optional

### `bitmap_layer.delete` — Delete

- `id`: Int {"min":0,"max":null}; default=0; optional

### `cam.bevel_carving` — Bevel Carving

- `bevel_max_depth`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; optional
- `feed_rate`: Float {"min":0,"max":null,"step":50,"unit":"mm/min"}; default=1200; optional
- `half_angle_deg`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=45; optional
- `plunge_rate`: Float {"min":0,"max":null,"step":50,"unit":"mm/min"}; default=300; optional
- `safe_z`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=5; optional
- `spindle_rpm`: Float {"min":0,"max":null,"step":500,"unit":"rpm"}; default=18000; optional
- `start_depth`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `stepdown`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=2; optional
- `stepover`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=1; optional
- `style`: Choice ["centerline_with_bevels","multi_pass_offset"]; default="centerline_with_bevels"; optional
- `tool_id`: String; optional

### `cam.calculate_feeds_speeds` — Calculate Feeds and Speeds

- `tool_id`: String; default="Softwood"; optional

### `cam.check_collisions` — Check Toolpath Collisions

- `holder_diameter`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=35; optional
- `name`: String; optional
- `resolution`: String; optional
- `toolpath_id`: String; optional

### `cam.detect_gouging` — Detect Design Model Gouging

- `name`: String; optional
- `toolpath_id`: String; optional

### `cam.drill_bank` — Drill Banks

- `finish_depth`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=10; optional
- `pitch`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=32; optional
- `safe_z`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=5; optional
- `tool_id`: String; optional

### `cam.drill_create` — Drilling

- `cycle`: Choice ["standard","peck","chip_break","dwell"]; default="standard"; optional
- `finish_depth`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=1; optional
- `hole_diameter`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; optional
- `holes`: String; optional
- `layer`: String; optional
- `peck_depth`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=1; optional
- `safe_z`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=1; optional
- `tool_id`: String; optional

### `cam.export_linuxcnc_tbl` — Export LinuxCNC Tool Table

- `file_path`: String; optional

### `cam.import_linuxcnc_tbl` — Import LinuxCNC Tool Table

- `file_path`: String; optional

### `cam.inlay_wizard` — Inlay

- `corner_radius`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; optional
- `feed_rate`: Float {"min":0,"max":null,"step":50,"unit":"mm/min"}; default=1200; optional
- `female_depth`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; optional
- `glue_gap`: Float {"min":null,"max":null,"step":1,"unit":null}; optional
- `half_angle_deg`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=45; optional
- `kind`: String; optional
- `male_depth`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; optional
- `mirror_male`: Bool; default=false; optional
- `plunge_rate`: Float {"min":0,"max":null,"step":50,"unit":"mm/min"}; default=300; optional
- `polygon`: String; optional
- `safe_z`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=5; optional
- `spindle_rpm`: Float {"min":0,"max":null,"step":500,"unit":"rpm"}; default=18000; optional
- `tolerance`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=0.01; optional
- `tool_id`: String; optional

### `cam.optifeed` — OptiFeed Feedrate Optimizer

- `name`: String; optional
- `sim_resolution`: Float {"min":null,"max":null,"step":1,"unit":null}; optional
- `toolpath_id`: String; optional

### `cam.profile_options` — Profile Options (Bridges)

- `count`: Int {"min":0,"max":null}; default=4; optional
- `kind`: String; default="3d"; optional
- `length`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=5; optional
- `thickness`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=2; optional

### `cam.relief_3d` — Machine Relief

- `allowance`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `boundary`: String; optional
- `center_x`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `center_y`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `feed_rate`: Float {"min":0,"max":null,"step":50,"unit":"mm/min"}; default=1200; optional
- `plunge_rate`: Float {"min":0,"max":null,"step":50,"unit":"mm/min"}; default=300; optional
- `raster_angle`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=0; optional
- `safe_z`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `sample_interval`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `scallop_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `spindle_rpm`: Float {"min":0,"max":null,"step":500,"unit":"rpm"}; default=18000; optional
- `stepover`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `strategy`: String; optional
- `tool_id`: String; optional

### `cam.rest_machining_3d` — Rest Machining

- `allowance`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `blend_distance`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `boundary`: String; optional
- `center_x`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `center_y`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `feed_rate`: Float {"min":0,"max":null,"step":50,"unit":"mm/min"}; default=1200; optional
- `plunge_rate`: Float {"min":0,"max":null,"step":50,"unit":"mm/min"}; default=300; optional
- `prev_tool_id`: String; optional
- `safe_z`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `sample_interval`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `scallop_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `spindle_rpm`: Float {"min":0,"max":null,"step":500,"unit":"rpm"}; default=18000; optional
- `stepover`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `strategy`: String; optional
- `tool_id`: String; optional

### `cam.smart_engrave` — Smart Engraving

- `layer`: String; optional
- `safe_z`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=5; optional
- `tool_id`: String; default="scribe"; optional

### `cam.tool_create` — Create Tool

- `diameter`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; optional
- `group_id`: String; optional
- `name`: String; optional
- `tool`: String; optional

### `cam.tool_db_load` — Load Tool Database

- `file_path`: String; optional

### `cam.tool_db_save` — Save Tool Database

- `file_path`: String; optional

### `cam.tool_delete` — Delete Tool

- `id`: String; optional

### `cam.tool_get` — Get Tool

- `id`: String; optional
- `tool_number`: String; optional

### `cam.tool_group_create` — Create Tool Group

- `notes`: String; default=""; optional
- `parent_id`: String; default=""; optional

### `cam.tool_list` — List Tools

- `group_id`: String; optional

### `cam.tool_update` — Update Tool

- `diameter`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; optional
- `id`: String; optional
- `name`: String; optional
- `tool`: String; optional

### `cam.toolpath_delete` — Delete Toolpath

- `index`: Int {"min":0,"max":null}; default=0; optional
- `toolpath_id`: String; optional

### `cam.toolpath_get` — Get Toolpath

- `index`: Int {"min":0,"max":null}; default=0; optional
- `toolpath_id`: String; optional

### `cam.toolpath_list` — List Toolpaths

- `output_path`: String; optional

### `cam.toolpath_merge` — Merge Toolpaths

- `all`: Bool; default=false; optional
- `tolerance`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=0.01; optional

### `cam.toolpath_panelling` — Toolpath Panelling

- `overcut_distance`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=2; optional
- `panel_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=50; optional
- `panel_width`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=50; optional
- `safe_z`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=5; optional
- `toolpath_id`: String; default="horizontal"; optional

### `cam.toolpath_transform` — Transform Toolpath

- `dx`: Float {"min":null,"max":null,"step":1,"unit":null}; default=10; optional
- `dy`: Float {"min":null,"max":null,"step":1,"unit":null}; default=10; optional
- `dz`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `rotation_center`: String; optional
- `rotation_deg`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=0; optional
- `scale`: Float {"min":null,"max":null,"step":1,"unit":null}; default=1; optional
- `toolpath_id`: String; optional
- `translation`: String; optional

### `cam.vcarve` — V-Bit Carving

- `feed_rate`: Float {"min":0,"max":null,"step":50,"unit":"mm/min"}; default=1200; optional
- `half_angle_deg`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=45; optional
- `max_depth`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=5; optional
- `plunge_rate`: Float {"min":0,"max":null,"step":50,"unit":"mm/min"}; default=300; optional
- `safe_z`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=5; optional
- `spindle_rpm`: Float {"min":0,"max":null,"step":500,"unit":"rpm"}; default=18000; optional
- `start_depth`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `stepdown`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=2; optional
- `stepover`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=1; optional
- `style`: Choice ["centerline_with_bevels","multi_pass_offset"]; default="centerline_with_bevels"; optional
- `tool_id`: String; optional

### `cam.z_level_roughing` — Z Level Roughing

- `boundary`: String; optional
- `feed_rate`: Float {"min":0,"max":null,"step":50,"unit":"mm/min"}; default=1200; optional
- `finish_z`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `plunge_rate`: Float {"min":0,"max":null,"step":50,"unit":"mm/min"}; default=300; optional
- `safe_z`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `spindle_rpm`: Float {"min":0,"max":null,"step":500,"unit":"rpm"}; default=18000; optional
- `start_z`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `stepdown`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `stepover`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `stock_allowance`: String; optional
- `strategy`: String; optional
- `tool_id`: String; optional

### `clipart.instantiate` — Instantiate 3D Clipart

- `asset_id`: Int {"min":0,"max":null}; default=0; optional

### `clipart.paste_copy` — Paste Copy

- `id`: String; optional

### `clipart.paste_single` — Paste Single

- `id`: String; optional

### `clipart.reset_transform` — Reset

- `id`: String; optional

### `clipart.transform` — Transform 3D Component

- `id`: Int {"min":0,"max":null}; default=0; optional
- `rotation_deg`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=0; optional
- `scale`: Float {"min":null,"max":null,"step":1,"unit":null}; default=1; optional
- `translate_x`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `translate_y`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional
- `translate_z`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional

### `file.open` — Open

- `filename`: String; optional
- `json`: String; optional

### `history.begin_transaction` — Begin Transaction

- `name`: String; optional

### `model.set_size` — Set Size

- `height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=50; optional
- `width`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=50; optional

### `project.assembly_create_relief` — Create Relief

- `id`: String; optional

### `project.assembly_delete` — Delete

- `id`: Int {"min":0,"max":null}; default=0; optional

### `project.assembly_hide` — Hide

- `id`: Int {"min":0,"max":null}; default=0; optional

### `project.assembly_rename` — Rename

- `id`: Int {"min":0,"max":null}; default=0; optional

### `project.assembly_show` — Show

- `id`: Int {"min":0,"max":null}; default=0; optional

### `project.group_components` — Group Components

- `components`: String; optional

### `project.ungroup` — Ungroup Components

- `id`: Int {"min":0,"max":null}; default=0; optional

### `relief.add_draft` — Add Draft

- `angle_deg`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=5; optional

### `relief.auto_scale_z` — Auto Scale Z

- `target_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=10; optional

### `relief.calculate_volume` — Relief Volume

- `density`: Float {"min":null,"max":null,"step":1,"unit":null}; default=1; optional

### `relief.create_cross_section` — Create Cross-Section

- `end_x`: Float {"min":null,"max":null,"step":1,"unit":null}; optional
- `end_y`: Float {"min":null,"max":null,"step":1,"unit":null}; optional
- `sample_step`: Float {"min":0,"max":null,"step":1,"unit":null}; optional
- `start_x`: Float {"min":null,"max":null,"step":1,"unit":null}; optional

### `relief.extrude` — Extrude

- `anchor`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0.5; optional
- `combine`: String; default="add"; optional
- `drive_curve`: String; optional
- `drive_id`: String; optional
- `final_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; optional
- `invert_profile_z`: String; optional
- `profile`: String; optional
- `start_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `width`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=50; optional

### `relief.fade` — Fade Relief

- `end_ratio`: Float {"min":null,"max":null,"step":0.01,"unit":null}; default=0; optional
- `end_x`: Float {"min":null,"max":null,"step":1,"unit":null}; optional
- `end_y`: Float {"min":null,"max":null,"step":1,"unit":null}; optional
- `start_ratio`: Float {"min":null,"max":null,"step":0.01,"unit":null}; default=1; optional
- `start_x`: Float {"min":null,"max":null,"step":1,"unit":null}; optional
- `start_y`: Float {"min":null,"max":null,"step":1,"unit":null}; optional

### `relief.offset_to_back` — Offset Composite To Back

- `offset`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=2; optional

### `relief.offset_z` — Offset

- `delta`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional

### `relief.scale_to_volume` — Scale to Target Volume

- `target_volume`: Float {"min":null,"max":null,"step":1,"unit":null}; default=1000; optional

### `relief.scale_z` — Scale

- `factor`: Float {"min":null,"max":null,"step":0.01,"unit":null}; default=1; optional
- `target_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=10; optional

### `relief.sculpt_carve` — Sculpt Carve

- `brush_mode`: String; default="deposit"; optional
- `tool`: String; default="deposit"; optional

### `relief.sculpt_deposit` — Sculpt Deposit

- `brush_mode`: String; default="deposit"; optional
- `tool`: String; default="deposit"; optional

### `relief.sculpt_erase` — Sculpt Erase

- `brush_mode`: String; default="deposit"; optional
- `tool`: String; default="deposit"; optional

### `relief.sculpt_smooth` — Sculpt Smooth

- `brush_mode`: String; default="deposit"; optional
- `tool`: String; default="deposit"; optional

### `relief.sculpt_smudge` — Sculpt Smudge

- `brush_mode`: String; default="deposit"; optional
- `tool`: String; default="deposit"; optional

### `relief.shape_editor` — Shape Editor

- `all`: Bool; default=false; optional
- `angle`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; optional
- `color`: String; optional
- `combine`: String; default="add"; optional
- `entity_indices`: String; optional
- `height_limit`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=10; optional
- `layer`: String; optional
- `layer_idx`: String; optional
- `layer_name`: String; optional
- `limit`: String; default="no_limit"; optional
- `limit_value`: Float {"min":null,"max":null,"step":1,"unit":null}; default=10; optional
- `shape`: String; default="plane"; optional
- `start_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `tilt_angle`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=0; optional
- `tolerance`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=0; optional

### `relief.slice` — Slice Relief

- `thickness`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=5; optional

### `relief.smooth` — Smooth

- `strength`: Float {"min":null,"max":null,"step":1,"unit":null}; default=1; optional

### `relief.spin` — Spin

- `angle`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=360; optional
- `angle_deg`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=360; optional
- `center`: String; optional
- `combine`: String; default="add"; optional
- `final_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; optional
- `invert_profile_z`: String; optional
- `profile`: String; optional
- `scale`: Float {"min":null,"max":null,"step":1,"unit":null}; default=1; optional
- `start_angle`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=0; optional
- `start_angle_deg`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=0; optional
- `start_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `z_scale`: Float {"min":null,"max":null,"step":1,"unit":null}; default=1; optional

### `relief.turn` — Turn

- `axis`: String; optional
- `combine`: String; default="add"; optional
- `final_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; optional
- `invert_profile_z`: String; optional
- `profile`: String; optional
- `scale`: Float {"min":null,"max":null,"step":1,"unit":null}; default=1; optional
- `start_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `z_scale`: Float {"min":null,"max":null,"step":1,"unit":null}; default=1; optional

### `relief.two_rail_sweep` — Two Rail Sweep

- `combine`: String; default="add"; optional
- `cross_section_id`: String; optional
- `final_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; optional
- `invert_profile_z`: String; optional
- `profile`: String; optional
- `rail1`: String; optional
- `rail2`: String; optional
- `start_height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional

### `relief_layer.delete` — Delete

- `id`: Int {"min":0,"max":null}; default=0; optional

### `relief_layer.select` — Activate Relief Layer

- `id`: Int {"min":0,"max":null}; default=0; optional
- `name`: String; optional

### `relief_layer.set_combine_mode` — Set Combine Mode

- `combine_mode`: String; default="add"; optional
- `id`: Int {"min":0,"max":null}; default=0; optional
- `name`: String; default="add"; optional

### `relief_layer.set_opacity` — Set Relief Layer Opacity

- `opacity`: Float {"min":null,"max":null,"step":1,"unit":null}; default=1; optional

### `relief_layer.set_visibility` — Set Visibility

- `id`: Int {"min":0,"max":null}; default=0; optional
- `name`: String; optional

### `relief_layer.toggle_visibility` — Toggle Visibility

- `id`: Int {"min":0,"max":null}; default=0; optional
- `name`: String; optional

### `vector.align_nodes_x` — in X

- `nodes`: String; optional

### `vector.align_nodes_y` — in Y

- `nodes`: String; optional

### `vector.assign_to_layer` — Assign Vectors To Layer

- `from_layer`: String; optional
- `indices`: String; optional
- `target_layer`: String; optional

### `vector.convert_to_arc` — Convert span to arc

- `span`: Int {"min":0,"max":null}; default=0; optional

### `vector.convert_to_bezier` — Convert spans to a bezier

- `span`: Int {"min":0,"max":null}; default=0; optional

### `vector.convert_to_line` — Convert spans to a line

- `span`: Int {"min":0,"max":null}; default=0; optional

### `vector.create_arc` — Arc Tool

- `end_deg`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=90; optional
- `p1`: String; optional
- `p2`: String; optional
- `p3`: String; optional
- `radius`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=10; optional
- `start_deg`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=0; optional

### `vector.create_boundary` — Create Boundary From Relief

- `offset`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `threshold`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0; optional

### `vector.create_circle` — Circle Tool

- `radius`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=10; optional

### `vector.create_ellipse` — Ellipse Tool

- `radius_x`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=20; optional
- `radius_y`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=10; optional
- `rotation_deg`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=0; optional

### `vector.create_polyline` — Polyline Tool

- `points`: String; optional

### `vector.create_rectangle` — Rectangle Tool

- `corner_radius`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=0; optional
- `height`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=30; optional
- `width`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=50; optional

### `vector.create_regular_polygon` — Polygon Tool

- `radius`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=25; optional
- `rotation_deg`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=0; optional
- `sides`: Int {"min":0,"max":null}; default=5; optional

### `vector.create_star` — Star Tool

- `points`: Int {"min":0,"max":null}; default=5; optional
- `radius_inner`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=10; optional
- `radius_outer`: Float {"min":0,"max":null,"step":0.1,"unit":"mm"}; default=25; optional
- `rotation_deg`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=0; optional

### `vector.delete_node` — Delete Nodes

- `index`: String; optional

### `vector.doctor_diagnose` — Diagnose Vectors

- `layer`: Int {"min":0,"max":null}; default=0; optional

### `vector.doctor_heal` — Auto-Heal Vectors

- `layer`: Int {"min":0,"max":null}; default=0; optional

### `vector.export_layers_manifest` — Export Layers Manifest

- `path`: String; optional

### `vector.insert_midpoint` — Insert a mid node

- `span`: Int {"min":0,"max":null}; default=0; optional

### `vector.insert_node` — Insert a node

- `span`: Int {"min":0,"max":null}; default=0; optional
- `t`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0.5; optional

### `vector.insert_start_node` — Insert Start Node

- `span`: Int {"min":0,"max":null}; default=0; optional
- `t`: Float {"min":null,"max":null,"step":1,"unit":null}; default=0.5; optional

### `vector.join_coincident` — With coincident nodes

- `auto_close`: String; optional
- `entity1`: String; optional
- `entity2`: String; optional
- `entity_a`: String; optional
- `entity_b`: String; optional
- `entity_ids`: String; optional
- `index_a`: String; optional
- `index_b`: String; optional
- `indices`: String; optional
- `layer`: Int {"min":0,"max":null}; default=0; optional
- `tolerance`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=0.05; required

### `vector.join_move_ends` — Move end nodes

- `tolerance`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=10; optional

### `vector.join_with_curve` — With a smooth curve

- `tolerance`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=50; optional

### `vector.join_with_line` — With a line

- `tolerance`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=50; optional

### `vector.mirror_vertical_bottom` — Vertical (Bottom)

- `pivot_x`: Float {"min":null,"max":null,"step":1,"unit":null}; optional
- `pivot_y`: Float {"min":null,"max":null,"step":1,"unit":null}; optional

### `vector.mirror_vertical_top` — Vertical (Top)

- `pivot_x`: Float {"min":null,"max":null,"step":1,"unit":null}; optional
- `pivot_y`: Float {"min":null,"max":null,"step":1,"unit":null}; optional

### `vector.move_handle` — Move Handle

- `index`: String; optional
- `layer`: Int {"min":0,"max":null}; default=0; optional
- `pos`: String; optional
- `x`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=20; optional
- `y`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=10; optional

### `vector.move_node` — Move Node

- `index`: String; optional
- `pos`: String; optional
- `x`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=10; optional
- `y`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=10; optional

### `vector.offset` — Offset Vectors

- `distance`: Float {"min":null,"max":null,"step":0.1,"unit":"mm"}; default=10; optional
- `layer`: Int {"min":0,"max":null}; default=0; optional
- `miter_limit`: Float {"min":null,"max":null,"step":1,"unit":null}; default=4; optional

### `vector.remove_duplicate_nodes` — Remove Duplicate Nodes

- `layer`: Int {"min":0,"max":null}; default=0; optional

### `vector.remove_span` — Remove spans

- `span`: Int {"min":0,"max":null}; default=0; optional

### `vector.rotate_copy` — Rotate Copy

- `center_x`: Float {"min":null,"max":null,"step":1,"unit":null}; optional
- `center_y`: Float {"min":null,"max":null,"step":1,"unit":null}; optional
- `count`: Int {"min":0,"max":null}; default=4; optional
- `index`: Int {"min":0,"max":null}; default=0; optional
- `layer`: Int {"min":0,"max":null}; default=0; optional
- `total_angle`: Float {"min":null,"max":null,"step":0.1,"unit":"deg"}; default=360; optional

### `vector.set_start_node` — Start Node

- `index`: String; optional

### `vector.smooth_node` — Smooth Nodes

- `index`: String; optional

### `vector.split_contour` — Cut Vector

- `index`: String; optional

### `vector_layer.create` — Create New Layer

- `color`: String; optional

### `vector_layer.delete` — Delete

- `id`: Int {"min":0,"max":null}; default=0; optional

### `vector_layer.select` — Activate Layer

- `id`: Int {"min":0,"max":null}; default=0; optional
- `name`: String; optional


## Required traceability for every retained command

Each retained command must gain a row in the implementation trace with: canonical ID; source handler file and match arm; typed payload; validation rules; domain operation; document/session effects; result schema; failure cases; CLI/GUI/MCP exposure; unit and acceptance tests; and status. The source JSON alone does not provide these semantics.
