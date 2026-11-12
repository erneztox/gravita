# Command-to-handler trace (source investigation)

This trace correlates the 116 schema IDs with exact string literals found in the old engine command modules. It is deliberately conservative: a literal occurrence is evidence of a handler path, not proof of full correctness, reachability, or product acceptance. IDs without an exact literal in the inspected files remain unresolved and require investigation of aliases, dynamic dispatch, or generated routing.

**Inspected modules:** `crates/engine/src/cmd/bitmap.rs`, `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/sim.rs`, `crates/engine/src/cmd/registry.rs`.

**Disposition rule:** bitmap painting and interactive vector editor commands are excluded by the approved architecture pivot. Other commands are candidates only. In particular, a command's schema does not establish that the command is needed in the product. `vector_layer.*` is kept separate from `vector.*`: imported-geometry layer organization may remain useful without implementing an editor.

| Command ID | Source handler evidence | Initial disposition |
|---|---|---|
| `bitmap.brush_stroke` | `crates/engine/src/cmd/bitmap.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `bitmap.flood_fill` | `crates/engine/src/cmd/bitmap.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `bitmap.set_primary_colour` | `crates/engine/src/cmd/bitmap.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `bitmap.set_secondary_colour` | `crates/engine/src/cmd/bitmap.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `bitmap_layer.delete` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `cam.bevel_carving` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.calculate_feeds_speeds` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.check_collisions` | `crates/engine/src/cmd/sim.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.detect_gouging` | `crates/engine/src/cmd/sim.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.drill_bank` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.drill_create` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.export_linuxcnc_tbl` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.import_linuxcnc_tbl` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.inlay_wizard` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.optifeed` | `crates/engine/src/cmd/sim.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.profile_options` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.relief_3d` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.rest_machining_3d` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.smart_engrave` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.tool_create` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.tool_db_load` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.tool_db_save` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.tool_delete` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.tool_get` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.tool_group_create` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.tool_list` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.tool_update` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.toolpath_delete` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.toolpath_get` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.toolpath_list` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.toolpath_merge` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.toolpath_panelling` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.toolpath_transform` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.vcarve` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `cam.z_level_roughing` | `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `clipart.instantiate` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `clipart.paste_copy` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `clipart.paste_single` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `clipart.reset_transform` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `clipart.transform` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `file.open` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `history.begin_transaction` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `model.set_size` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `project.assembly_create_relief` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `project.assembly_delete` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `project.assembly_hide` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `project.assembly_rename` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `project.assembly_show` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `project.group_components` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `project.ungroup` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.add_draft` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.auto_scale_z` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.calculate_volume` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.create_cross_section` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.extrude` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.fade` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.offset_to_back` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.offset_z` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.scale_to_volume` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.scale_z` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.sculpt_carve` | `crates/engine/src/cmd/relief.rs` | CANDIDATE — handler literal observed |
| `relief.sculpt_deposit` | `crates/engine/src/cmd/relief.rs` | CANDIDATE — handler literal observed |
| `relief.sculpt_erase` | `crates/engine/src/cmd/relief.rs` | CANDIDATE — handler literal observed |
| `relief.sculpt_smooth` | `crates/engine/src/cmd/relief.rs` | CANDIDATE — handler literal observed |
| `relief.sculpt_smudge` | `crates/engine/src/cmd/relief.rs` | CANDIDATE — handler literal observed |
| `relief.shape_editor` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.slice` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.smooth` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.spin` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.turn` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief.two_rail_sweep` | `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief_layer.delete` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief_layer.select` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief_layer.set_combine_mode` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief_layer.set_opacity` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief_layer.set_visibility` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `relief_layer.toggle_visibility` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `vector.align_nodes_x` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.align_nodes_y` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.assign_to_layer` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.convert_to_arc` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.convert_to_bezier` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.convert_to_line` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.create_arc` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.create_boundary` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.create_circle` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.create_ellipse` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.create_polyline` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.create_rectangle` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.create_regular_polygon` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.create_star` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.delete_node` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.doctor_diagnose` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.doctor_heal` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.export_layers_manifest` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.insert_midpoint` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.insert_node` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.insert_start_node` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.join_coincident` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.join_move_ends` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.join_with_curve` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.join_with_line` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.mirror_vertical_bottom` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.mirror_vertical_top` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.move_handle` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.move_node` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.offset` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.remove_duplicate_nodes` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.remove_span` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.rotate_copy` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.set_start_node` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.smooth_node` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector.split_contour` | `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/registry.rs` | EXCLUDE — editor/pixel workflow |
| `vector_layer.create` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `vector_layer.delete` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |
| `vector_layer.select` | `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/registry.rs` | CANDIDATE — handler literal observed |

## Follow-up required

1. For each candidate, inspect the full match arm and called domain functions; record payload parsing, defaults, validation, mutations, return JSON and error paths.
2. Resolve commands whose implementation is shared behind aliases (for example simulation commands in `sim.rs`) and collapse aliases to canonical operations.
3. Inspect registry metadata and actual command execution routing; confirm status and whether the command is reachable from the public engine API.
4. Add acceptance tests using real project/geometry/toolpath data. No candidate becomes retained merely because a handler string exists.
5. For every excluded command, ensure no frontend or test accidentally depends on it; delete obsolete tests rather than carrying dead editor functionality forward.
