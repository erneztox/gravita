# Full Engine Command Bus inventory

## Source

Extracted from `crates/engine/src/cmd/registry.rs` in [erneztox/artcam-investigation](https://github.com/erneztox/artcam-investigation). The old registry contains **404 entries / 404 unique command IDs**. Status counts: Implemented=403, Draft=1. This is distinct from the 116 parameter schemas preserved in [COMMANDS.md](COMMANDS.md).

Handler files inspected: `crates/engine/src/cmd/bitmap.rs`, `crates/engine/src/cmd/vector.rs`, `crates/engine/src/cmd/doc.rs`, `crates/engine/src/cmd/relief.rs`, `crates/engine/src/cmd/cam.rs`, `crates/engine/src/cmd/sim.rs`, `crates/engine/src/cmd/common.rs`. Handler column means the exact command ID string occurs in that module; it is not a semantic proof that the command is fully implemented. Schema field count is `—` where the 116-entry generated schema has no entry for that command.

## Scope policy

The approved architecture pivot excludes interactive vector editor, bitmap painting, palette/color linking, and image-canvas commands. Imported vector geometry and its document/layer organization are still allowed as I/O and CAM inputs; do not reintroduce interactive drafting under another name. Remaining commands are candidates pending handler-level semantic review, tests, and WBS mapping. A source registry status of `Implemented` is historical metadata, not proof of acceptance in Gravita.

## All registered commands

| Canonical source ID | Legacy numeric ID | Label | Menu path | Source status | Schema fields | Handler literal evidence | Initial Gravita disposition |
|---|---:|---|---|---|---:|---|---|
| `model.new` | 50179 | New Model... | File > New > Model... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `model.new_alt` | 58354 | New Model... | File > New > Model... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `model.new_project` | 50181 | New Project | File > New > Project | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `model.new_project_alt` | 58355 | New Project | File > New > Project | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `model.new_pixels` | 50180 | Model (Specify Pixel Size)... | File > New > Model (Specify Pixel Size)... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `model.new_pixels_alt` | 58360 | Model (Specify Pixel Size)... | File > New > Model (Specify Pixel Size)... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `file.open` | 57601 | Open... | File > Open... | Implemented | 2 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `file.save` | 57603 | Save | File > Save | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `file.save_as` | 57604 | Save As... | File > Save As... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `file.close` | 57602 | Close | File > Close | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `file.close_model` | 50175 | Close Model | File > Close Model | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `file.close_model_alt` | 58432 | Close Model | File > Close Model | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.save` | 50402 | Save | pop-up > Save | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.save_alt` | 58391 | Save | pop-up > Save | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.save_as` | 50403 | Save As... | pop-up > Save As... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.save_as_alt` | 58392 | Save As... | pop-up > Save As... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.close` | 50401 | Close | pop-up > Close | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.close_alt` | 58393 | Close | pop-up > Close | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `model.set_size` | 50218 | Set Size... | Model > Set Size... | Implemented | 2 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `edit.undo` | 57643 | Undo | Edit > Undo | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `edit.redo` | 57644 | Redo | Edit > Redo | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `edit.cut` | 57635 | Cut | Edit > Cut | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `edit.copy` | 57634 | Copy | Edit > Copy | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `edit.paste` | 57637 | Paste | Edit > Paste | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `edit.delete` | 50147 | Delete | Edit > Delete | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `edit.copy_relief` | 50144 | Copy Relief | Edit > Copy Relief | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `edit.copy_bitmap` | 50143 | Copy Bitmap | Edit > Copy Bitmap | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `edit.select_all` | 50161 | Select All | Edit > Select All | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `edit.select_layer` | 50162 | Select All On Layer | Edit > Select All On Layer | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `history.undo` | — | History Undo |  | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `history.redo` | — | History Redo |  | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `history.begin_transaction` | — | Begin Transaction |  | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `history.commit_transaction` | — | Commit Transaction |  | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `history.rollback_transaction` | — | Rollback Transaction |  | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `history.status` | — | History Status |  | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `history.clear` | — | Clear History |  | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `vector.create_circle` | 58139 | Circle Tool | Vectors > Create > Circle | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.create_rectangle` | 57825 | Rectangle Tool | Vectors > Create > Rectangle | Implemented | 3 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.create_polyline` | 57823 | Polyline Tool | Vectors > Create > Polyline | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.create_arc` | 57830 | Arc Tool | Vectors > Create > Arc | Implemented | 6 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.create_ellipse` | 57887 | Ellipse Tool | Vectors > Create > Ellipse | Implemented | 3 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.create_regular_polygon` | 58141 | Polygon Tool | Vectors > Create > Regular Polygon | Implemented | 3 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.create_star` | 58140 | Star Tool | Vectors > Create > Star | Implemented | 4 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.weld` | 50649 | Weld | Vectors > Merge Vectors > Weld | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.subtract` | 50647 | Subtract | Vectors > Merge Vectors > Subtract | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.intersect` | 50646 | Intersect | Vectors > Merge Vectors > Intersect | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.offset` | 50290 | Offset Vectors... | Vectors > Offset... | Implemented | 3 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.fillet` | 50643 | Fillet Vectors... | Vectors > Fillet... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.fillet_arc` | 58242 | Fillet vector with an arc | Vectors > Fillet with an arc | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.trim` | 50654 | Trim Vectors... | Vectors > Trim... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.trim_vector` | 58243 | Trim vector | Vectors > Trim vector | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.merge_trim` | 50648 | Trim | Vectors > Merge Vectors > Trim | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.merge_trim_alt` | 58138 | Trim | Vectors > Merge Vectors > Trim (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align` | — | Align Vectors | Vectors > Align Vectors | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_left` | 50638 | Left | Vectors > Align Vectors > Left | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_left_alt` | 58089 | Left | Vectors > Align Vectors > Left (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_horizontal_centre` | 50656 | Horizontal Centre | Vectors > Align Vectors > Horizontal Centre | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_horizontal_centre_alt` | 58091 | Horizontal Centre | Vectors > Align Vectors > Horizontal Centre (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_right` | 50657 | Right | Vectors > Align Vectors > Right | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_right_alt` | 58092 | Right | Vectors > Align Vectors > Right (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_top` | 50658 | Top | Vectors > Align Vectors > Top | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_top_alt` | 58093 | Top | Vectors > Align Vectors > Top (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_vertical_centre` | 50639 | Vertical Centre | Vectors > Align Vectors > Vertical Centre | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_vertical_centre_alt` | 58094 | Vertical Centre | Vectors > Align Vectors > Vertical Centre (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_bottom` | 50655 | Bottom | Vectors > Align Vectors > Bottom | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_bottom_alt` | 58096 | Bottom | Vectors > Align Vectors > Bottom (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_centre` | 50636 | Centre | Vectors > Align Vectors > Centre | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_centre_alt` | 58131 | Centre | Vectors > Align Vectors > Centre (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_centre_page` | 50637 | Centre In Page | Vectors > Align Vectors > Centre In Page | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_centre_page_alt` | 58133 | Centre In Page | Vectors > Align Vectors > Centre In Page (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.group` | 50200 | Group Vector(s) | Vectors > Group Vector(s) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.group_alt` | 57847 | Group Vector(s) | Edit > Group Vector(s) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.ungroup` | 50629 | Ungroup Vector(s) | Vectors > Ungroup Vector(s) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.ungroup_alt` | 57848 | Ungroup Vector(s) | Edit > Ungroup Vector(s) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.ungroup_all` | 50630 | Ungroup All | Vectors > Ungroup All | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.ungroup_all_alt` | 58251 | Ungroup All | Vectors > Ungroup All (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.fit_arcs` | 50057 | Fit Arcs to Vector(s)... | Vectors > Fit Arcs to Vector(s)... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.fit_arcs_alt` | 58182 | Fit Arcs to Vector(s)... | Vectors > Fit Arcs to Vector(s)... (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.spline` | 50542 | Spline Vectors... | Vectors > Spline Vectors... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.spline_alt` | 57815 | Spline Vectors... | Vectors > Spline Vectors... (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.create_border` | 50124 | Create Border... | Vectors > Create Border... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.create_border_alt` | 58261 | Create Border... | Vectors > Create Border... (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.wrap` | 50677 | Wrap... | Vectors > Wrap... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.wrap_alt` | 58245 | Wrap... | Vectors > Wrap... (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.convert` | 50117 | Convert to... | Vectors > Convert to... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.convert_alt` | 58260 | Convert to... | Vectors > Convert to... (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.create_boundary` | 50120 | Create Boundary From Relief... | Vectors > Create Boundary From Relief... | Implemented | 2 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.library` | 50645 | Vector Library... | Vectors > Vector Library... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.library_alt` | 58247 | Vector Library... | Vectors > Vector Library... (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.properties` | 50384 | Properties... | Vectors > Properties... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.properties_alt` | 57824 | Properties... | Vectors > Properties... (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.create_sheet` | 50014 | Create Sheet | Vectors > Sheets > Create Sheet | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.create_sheet_alt` | 58292 | Create Sheet | Vectors > Sheets > Create Sheet (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.delete_sheet` | 50017 | Delete Active Sheet | Vectors > Sheets > Delete > Delete Active Sheet | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.delete_sheet_alt` | 58291 | Delete Active Sheet | Vectors > Sheets > Delete > Delete Active Sheet (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.sheet_report` | 50020 | Sheet Report - Active | Vectors > Sheets > Sheet Report - Active | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.sheet_report_alt` | 58294 | Sheet Report - Active | Vectors > Sheets > Sheet Report - Active (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.all_sheets_report` | 50055 | Sheet Report - All | Vectors > Sheets > Sheet Report - All | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.all_sheets_report_alt` | 58435 | Sheet Report - All | Vectors > Sheets > Sheet Report - All (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.load_layer` | 50666 | Load Layer... | Vectors > Load Layer... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.load_layer_alt` | 58318 | Load Layer... | Vectors > Load Layer... (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.save_layer` | 50675 | Save Layer... | Vectors > Save Layer... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.save_layer_alt` | 58312 | Save Layer... | Vectors > Save Layer... (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.export_layer` | 50779 | Export... | Vector Layers > Export... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.import` | 50177 | Import... | Vectors > Import... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.import_alt` | 57846 | Import... | Vectors > Import... (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.export` | 50176 | Export... | Vectors > Export... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.assign_to_layer` | — | Assign Vectors To Layer | Vectors > Assign To Layer | Implemented | 3 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.export_layers_manifest` | — | Export Layers Manifest | Vectors > Export Layers Manifest | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.export_alt` | 57888 | Export... | Vectors > Export... (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.flood_fill_vectors` | 50635 | Flood Fill Vectors | Vectors > Flood Fill Vectors | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.flood_fill_vectors_alt` | 57850 | Flood Fill Vectors | Vectors > Flood Fill Vectors (Alt) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.vector_to_bitmap` | 50500 | Vector To Bitmap | Vectors > Vector To Bitmap | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.vector_to_bitmap_alt` | 57947 | Copy Vectors Into Bitmap | Vectors > Copy Vectors Into Bitmap | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.bitmap_to_vector` | 50018 | Bitmap To Vector... | Vectors > Bitmap To Vector... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.bitmap_to_vector_alt` | 57851 | Fit Vectors To Colour Boundaries | Vectors > Fit Vectors To Colour Boundaries | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.node_edit_mode` | 50284 | Node Editing | Vectors > Node Editing | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.insert_node` | 50358 | Insert a node | Pop-Up > Insert a node | Implemented | 2 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.insert_midpoint` | 50357 | Insert a mid node | Pop-Up > Insert a mid node | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.delete_node` | 50370 | Delete Nodes | Pop-Up > Delete Nodes | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.smooth_node` | 50392 | Smooth Nodes | Pop-Up > Smooth Nodes | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.set_start_node` | 50394 | Start Node | Pop-Up > Start Node | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.insert_start_node` | 50381 | Insert Start Node | Pop-Up > Insert Start Node | Implemented | 2 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.split_contour` | 50393 | Cut Vector | Pop-Up > Cut Vector | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.remove_span` | 50386 | Remove spans | Pop-Up > Remove spans | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.convert_to_line` | 50397 | Convert spans to a line | Pop-Up > Convert spans to a line | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.convert_to_arc` | 50395 | Convert span to arc | Pop-Up > Convert span to arc | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.convert_to_bezier` | 50396 | Convert spans to a bezier | Pop-Up > Convert spans to a bezier | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_nodes_x` | 50052 | in X | Pop-Up > Align Nodes > in X | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.align_nodes_y` | 50053 | in Y | Pop-Up > Align Nodes > in Y | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.reverse` | 50475 | Reverse Vector(s) | Pop-Up > Reverse Vector(s) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.join_move_ends` | 50000 | Move end nodes | Vectors > Join Vectors > Move end nodes | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.join_with_curve` | 50001 | With a smooth curve | Vectors > Join Vectors > With a smooth curve | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.join_with_line` | 50002 | With a line | Vectors > Join Vectors > With a line | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.join_coincident` | 50007 | With coincident nodes | Vectors > Join Vectors > With coincident nodes | Implemented | 11 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.close_move_ends` | 50003 | Move end node | Vectors > Close Vector > Move end node | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.close_with_line` | 50004 | With a line | Vectors > Close Vector > With a line | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.close_with_curve` | 50005 | With a smooth curve | Vectors > Close Vector > With a smooth curve | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.move_node` | — | Move Node |  | Implemented | 4 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.move_handle` | — | Move Handle |  | Implemented | 5 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.transform` | 50019 | Transform Vector(s)... | Vectors > Transform... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.transform_dialog` | 58250 | Vector Transform | Cursor > Vector Transform | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.block_copy` | 58106 | Block Copy / Rotate | Actions > Block Copy / Rotate | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.block_copy_grid` | 50095 | Block Copy | Edit > Block Copy | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.rotate_copy` | 50118 | Rotate Copy | Edit > Rotate Copy | Implemented | 6 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.paste_along_curve` | 50312 | Paste Vector(s) on Curve... | Vectors > Paste Vector(s) on Curve... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.paste_vectors_on_curve` | 58187 | Paste Vector(s) on Curve... | Vectors > Paste Vector(s) on Curve... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.mirror` | 50264 | Mirror... | Vectors > Mirror... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.mirror_vectors` | 58180 | Mirror Vectors | Secrets > Mirror Vectors | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.mirror_horizontal_centre` | 50667 | Horizontal (Centre) | Vectors > Mirror > Horizontal (Centre) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.mirror_horizontal_left` | 50668 | Horizontal (Left) | Vectors > Mirror > Horizontal (Left) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.mirror_horizontal_right` | 50669 | Horizontal (Right) | Vectors > Mirror > Horizontal (Right) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.mirror_vertical_centre` | 50672 | Vertical (Centre) | Vectors > Mirror > Vertical (Centre) | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.mirror_vertical_top` | 50673 | Vertical (Top) | Vectors > Mirror > Vertical (Top) | Implemented | 2 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.mirror_vertical_bottom` | 50671 | Vertical (Bottom) | Vectors > Mirror > Vertical (Bottom) | Implemented | 2 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.doctor` | 50642 | Vector Doctor... | Vect&ors > Vector Doctor... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.doctor_wizards` | 58249 | Vector Doctor... | V&ectors > Vector Doctor... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.doctor_toolpane` | 2509 | Vector Doctor Toolpane |  | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.doctor_diagnose` | — | Diagnose Vectors |  | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.doctor_heal` | — | Auto-Heal Vectors |  | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.remove_duplicate_nodes` | — | Remove Duplicate Nodes |  | Implemented | 1 | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.clip_to_boundaries` | 50640 | Clip vectors to boundaries | Vect&ors > Clip vectors to boundaries | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.clip_to_boundaries_wizards` | 58252 | Clip vectors to boundaries | V&ectors > Clip vectors to boundaries | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.text_tool` | 57803 | Contour Text | Actions > Contour Text | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.text` | 50653 | Vector Text | Vectors > Vector Text | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.fit_text_to_curve` | 58115 | Fit Text To Curve... | Fit Text To Curve... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.fit_text` | 50183 | Fit Text | Vectors > Fit Text | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.edit_text` | 57761 | Text | Actions > Text | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.edit_text_core` | 50163 | Edit Text | Vectors > Edit Text | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.fit_text_toolpane` | 2524 | Fit Text To Curve Toolpane |  | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.nest_vectors` | 50280 | Nest Vectors... | Vect&ors > Nest Vectors... | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.nest_vectors_wizards` | 58174 | Nest Vectors | Actions > Nest Vectors | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `vector.nest_toolpane` | 2525 | Nest Vectors Toolpane |  | Implemented | — | `vector.rs` | EXCLUDE — editor/bitmap/color workflow |
| `relief.reset` | 50455 | Reset | Reliefs > Reset | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.invert_z` | 50431 | Invert > Z Only | Reliefs > Invert > Z Only | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.invert_male_female` | 50432 | Invert > Male / Female | Reliefs > Invert > Male / Female | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.auto_scale_z` | 50360 | Auto Scale Z | pop-up > Auto Scale Z | Implemented | 1 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.fade` | 50466 | Fade Relief... | Reliefs > Fade Relief... | Implemented | 6 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.add_draft` | 50407 | Add Draft... | Reliefs > Add Draft... | Implemented | 1 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.slice` | 50461 | Slice Relief... | Reliefs > Slice Relief... | Implemented | 1 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.calculate_volume` | — | Relief Volume... | Reliefs > Relief Volume... | Implemented | 1 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.surface_area` | 50464 | Surface Area Under Colour... | Reliefs > Surface Area Under Colour... | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.create_cross_section` | 50417 | Create Cross-Section... | Reliefs > Create Cross-Section... | Implemented | 4 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.scale_to_volume` | — | Scale to Target Volume... | Reliefs > Scale to Target Volume... | Implemented | 1 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.scale_z` | 50460 | Scale... | Reliefs > Scale... | Implemented | 2 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.smooth` | 50434 | Smooth... | Reliefs > Smooth... | Implemented | 1 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.offset_z` | 50449 | Offset... | Reliefs > Offset... | Implemented | 1 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.calculate_add` | 50409 | Add | Reliefs > Calculate > Add | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.calculate_subtract` | 50413 | Subtract | Reliefs > Calculate > Subtract | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.calculate_merge_high` | 50410 | Merge Highest | Reliefs > Calculate > Merge Highest | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.calculate_merge_low` | 50411 | Merge Lowest | Reliefs > Calculate > Merge Lowest | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.calculate_replace` | 50412 | Replace | Reliefs > Calculate > Replace | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.create_mesh` | 50420 | Create Triangle Mesh... | Reliefs > Create Triangle Mesh... | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.create_mesh_alt` | 58220 | Create Triangle Mesh... | Reliefs > Create Triangle Mesh... (Alt) | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.invert_to_back` | 50433 | Invert Composite To Back | Reliefs > Invert Composite To Back | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.offset_to_back` | 58322 | Offset Composite To Back | Reliefs > Offset Composite To Back | Implemented | 1 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.toggle_stack` | 50577 | Toggle Back Relief | View > 3D View > Toggle Back Relief | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.import_3d` | 50224 | Import 3D Model... | Reliefs > Import 3D Model... | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.import_3d_alt` | 58078 | Load 3D Model | Secrets > Load 3D Model | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `mesh.export` | 50256 | Export... | pop-up > Export... | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `mesh.export_alt` | 58374 | Export... | pop-up > Export... (Alt) | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `model.create_mesh` | 50127 | Create Mesh... | pop-up > Create Mesh... | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `model.create_mesh_alt` | 58482 | Create Mesh... | pop-up > Create Mesh... (Alt) | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.save_layer` | 50458 | Save Layer... | Reliefs > Save Layer... | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.save_layer_alt` | 57685 | Save Layer... | Reliefs > Save Layer... (Alt) | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.import` | 50440 | Import... | Reliefs > Import... | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.import_alt` | 57927 | Import... | Reliefs > Import... (Alt) | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.load_layer` | 50472 | Load Layer... | Reliefs > Load Layer... | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.load_layer_alt` | 58317 | Load Layer... | Reliefs > Load Layer... (Alt) | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.export` | 50568 | Export... | pop-up > Export... | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.import` | 50567 | Import... | pop-up > Import... | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `vector_layer.create` | 50777 | Create New Layer | Vectors > Create New Layer | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `vector_layer.delete` | 50783 | Delete | pop-up > Delete | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `vector_layer.duplicate` | 50780 | Duplicate | pop-up > Duplicate | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `vector_layer.rename` | 50239 | Rename | pop-up > Rename | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `vector_layer.toggle_visibility` | 50237 | Toggle Visibility | pop-up > Toggle Visibility | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `vector_layer.lock` | 50274 | Lock Vectors | pop-up > Lock Vectors | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `vector_layer.set_color` | 50322 | Set Colour... | pop-up > Set Colour... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `vector_layer.move_up` | 50247 | Move Up | pop-up > Move Up | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `vector_layer.move_down` | 50248 | Move Down | pop-up > Move Down | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `vector_layer.merge_visible` | 50781 | Merge Visible | pop-up > Merge Visible | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `vector_layer.toggle_all_visibility` | 50784 | Toggle All Visibility | pop-up > Toggle All Visibility | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `vector_layer.select_all` | 50162 | Select All On Layer | Edit > Select All On Layer | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `vector_layer.select` | — | Activate Layer | Vectors > Activate Layer | Implemented | 2 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `bitmap_layer.create` | 50082 | Create New Layer | Bitmaps > Create New Layer | Implemented | — | `doc.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap_layer.delete` | 50564 | Delete | pop-up > Delete | Implemented | 1 | `doc.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap_layer.clear` | 50562 | Clear | pop-up > Clear | Implemented | — | `doc.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap_layer.toggle_all_visibility` | 50565 | Toggle All Visibility | pop-up > Toggle All Visibility | Implemented | — | `doc.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap_layer.duplicate` | 50772 | Duplicate | Bitmaps > Duplicate Layer | Implemented | — | `doc.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap_layer.rename` | 50235 | Rename | Bitmaps > Rename Layer | Implemented | — | `doc.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap_layer.create_relief` | 50563 | Create Relief | pop-up > Create Relief | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.reduce_colors` | 112 | Reduce Number... | Bitmaps > Colour > Reduce Number... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `colour.reduce` | 50110 | Reduce Number... | Bitmaps > Colour > Reduce Number... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.merge_colours` | 113 | Merge | Bitmaps > Colour > Merge | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `colour.merge` | 50109 | Merge | Bitmaps > Colour > Merge | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.link_colours` | 50209 | Link / Unlink | Bitmaps > Colour > Link / Unlink | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `colour.link` | 57755 | Link / Unlink | Bitmaps > Colour > Link / Unlink | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.link_all_colours` | 50108 | Link All Colours | Bitmaps > Colour > Link All Colours | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `colour.link_all` | 57939 | Link All Colours | Bitmaps > Colour > Link All Colours | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.unlink_all_colours` | 50112 | Reset All Links | Bitmaps > Colour > Reset All Links | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `colour.unlink_all` | 57918 | Reset All Links | Bitmaps > Colour > Reset All Links | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.add_colour` | 50051 | Add Colour(s)... | Bitmaps > Colour > Add Colour(s)... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `colour.add` | 57696 | Add Colour(s)... | Bitmaps > Colour > Add Colour(s)... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.pick_colour` | 57742 | Pick Colour | Actions > Pick Colour | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.mark_edge_pixels` | 50211 | Mark Edge | Bitmaps > Colour > Mark Edge | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `colour.mark_edge` | 57778 | Mark Edge | Bitmaps > Colour > Mark Edge | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.adjust_resolution` | 50207 | Adjust Resolution... | Model > Adjust Resolution... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `model.adjust_resolution` | 61365 | Adjust Resolution... | Model > Adjust Resolution... | Implemented | — | `bitmap.rs` | CANDIDATE — requires pivot and handler review |
| `bitmap.set_primary_colour` | — | Set Primary Colour |  | Implemented | 2 | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.set_secondary_colour` | — | Set Secondary Colour |  | Implemented | 2 | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.clear` | 50208 | Clear | Bitmaps > Clear | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `image.clear` | 57972 | Clear | Bitmaps > Clear | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.thicken` | 50213 | Thicken... | Bitmaps > Colour > Thicken... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `colour.thicken` | 57779 | Thicken... | Bitmaps > Colour > Thicken... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.thin` | 50214 | Thin... | Bitmaps > Colour > Thin... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `colour.thin` | 57771 | Thin... | Bitmaps > Colour > Thin... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.deskew` | 50219 | Deskew... | Bitmaps > Deskew... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `image.deskew` | 57731 | Deskew... | Bitmaps > Deskew... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.save_palette` | 50212 | Save Palette... | Bitmaps > Colour > Save Palette... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `colour.save_palette` | 57773 | Save Palette... | Bitmaps > Colour > Save Palette... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.load_palette` | 50210 | Load Palette... | Bitmaps > Colour > Load Palette... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `colour.load_palette` | 57772 | Load Palette... | Bitmaps > Colour > Load Palette... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.save_layer` | 50087 | Save Layer... | Bitmaps > Save Layer... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap_layer.save` | 58310 | Save Layer... | Bitmaps > Save Layer... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.load_layer` | 50086 | Load Layer... | Bitmaps > Load Layer... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap_layer.load` | 58316 | Load Layer... | Bitmaps > Load Layer... | Implemented | — | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.flood_fill` | — | Flood Fill |  | Implemented | 4 | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `bitmap.brush_stroke` | — | Brush Stroke |  | Implemented | 7 | `bitmap.rs` | EXCLUDE — editor/bitmap/color workflow |
| `view.toggle_rulers` | 50484 | Show Rulers | Bitmaps > Views > Show Rulers | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.toggle_rulers_alt` | 57996 | Show Rulers | Bitmaps > Views > Show Rulers | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.toggle_guidelines` | 50201 | Show Guidelines | Bitmaps > Views > Show Guidelines | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.toggle_guidelines_alt` | 57997 | Show Guidelines | Bitmaps > Views > Show Guidelines | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.delete_guidelines` | 50135 | Delete All Guidelines | Bitmaps > Views > Delete All Guidelines | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.delete_guidelines_alt` | 57999 | Delete All Guidelines | Bitmaps > Views > Delete All Guidelines | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.toggle_snap_guides` | 50540 | Snap To Guidelines | Bitmaps > Views > Snap To Guidelines | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.toggle_snap_guides_alt` | 58081 | Snap To Guidelines | Bitmaps > Views > Snap To Guidelines | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.toggle_snap_objects` | 50541 | Snap To Objects | Bitmaps > Views > Snap To Objects | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.toggle_snap_objects_alt` | 61280 | Snap To Objects | Bitmaps > Views > Snap To Objects | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.snap_grid_settings` | 50021 | Snap Grid Settings... | Bitmaps > Views > Snap Grid Settings... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.snap_grid_settings_alt` | 58088 | Snap Grid Settings... | Bitmaps > Views > Snap Grid Settings... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.new_view` | 50732 | New View | Bitmaps > Views > New View | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.new_view_alt` | 57688 | New View | Bitmaps > Views > New View | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.delete_view` | 50730 | Delete | Bitmaps > Views > Delete | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.delete_view_alt` | 57764 | Delete | Bitmaps > Views > Delete | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.rename_view` | 50164 | Rename... | Bitmaps > Views > Rename... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.rename_view_alt` | 57700 | Rename... | Bitmaps > Views > Rename... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.2d` | 50727 | 2D View | Window > 2D View | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.3d` | 50729 | 3D View | Window > 3D View | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.tile_vertically` | 57652 | Tile Vertically | Window > Tile Vertically | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.tile_horizontally` | 57651 | Tile Horizontally | Window > Tile Horizontally | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `view.toggle_2d_3d` | 58129 | Toggle between 2D - 3D Views | Window > Toggle between 2D - 3D Views | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `window.tool_settings` | 50136 | Display Tool Settings | Window > Display Tool Settings | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `window.project_tree` | — | Project Tree | Window > Project Tree | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `window.toggle_dock` | — | Toggle Dock | Window > Toggle Dock | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `window.reset_layout` | — | Reset Layout | Window > Reset Layout | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `dialog.new_model` | — | New Model Dialog | File > New Model... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `dialog.shape_editor` | — | Shape Editor Dialog | Relief > Create > Shape Editor... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `dialog.cam_pocket` | — | 2D Pocket Toolpath Dialog | Toolpaths > 2D Machining > 2D Area Clearance (Pocket)... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `dialog.cam_profile` | — | 2D Profile Toolpath Dialog | Toolpaths > 2D Machining > 2D Profiling... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `dialog.cam_roughing` | — | 3D Roughing Toolpath Dialog | Toolpaths > 3D Machining > Z-Level Roughing... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `dialog.cam_finishing` | — | 3D Relief Finishing Toolpath Dialog | Toolpaths > 3D Machining > 3D Relief Finishing... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `dialog.cam_export` | — | Save / Export Toolpath Dialog | Toolpaths > Toolpath Management > Save / Export Toolpath... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `dialog.close` | — | Close Active Dialog |  | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.create` | 50566 | New | pop-up > New | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief.create_layer` | 50471 | Create New Layer | Reliefs > Create New Layer | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.select` | — | Activate Relief Layer | Reliefs > Activate Layer | Implemented | 2 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.delete` | 50573 | Delete | pop-up > Delete | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.duplicate` | 50569 | Duplicate | pop-up > Duplicate | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.rename` | 50238 | Rename | pop-up > Rename | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.toggle_visibility` | 50236 | Toggle Visibility | pop-up > Toggle Visibility | Implemented | 2 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.set_visibility` | — | Set Visibility |  | Implemented | 2 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.toggle_all_visibility` | 50574 | Toggle Relief Layer Visibility | pop-up > Toggle All Visibility | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.move_up` | 50240 | Move Up | pop-up > Move Up | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.move_down` | 50241 | Move Down | pop-up > Move Down | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.set_combine_add` | 50242 | Add | pop-up > Add | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.set_combine_subtract` | 50243 | Subtract | pop-up > Subtract | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.set_combine_merge_high` | 50244 | Merge High | pop-up > Merge High | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.set_combine_merge_low` | 50245 | Merge Low | pop-up > Merge Low | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.set_combine_multiply` | 50246 | Multiply | pop-up > Multiply | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.set_combine_mode` | — | Set Combine Mode |  | Implemented | 3 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.merge_visible` | 50571 | Merge Visible | pop-up > Merge Visible | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.compute_composite` | — | Compute Composite Relief |  | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief_layer.set_opacity` | — | Set Relief Layer Opacity |  | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.assembly_new` | 50072 | New | pop-up > New | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.assembly_delete` | 50066 | Delete | pop-up > Delete | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.assembly_rename` | 50075 | Rename | pop-up > Rename | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.assembly_show` | 50067 | Show | pop-up > Show | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.assembly_hide` | 50069 | Hide | pop-up > Hide | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.assembly_create_relief` | 50063 | Create Relief | pop-up > Create Relief | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `clipart.library` | 50414 | Relief Clipart Library... | Reliefs > Relief Clipart Library... | Implemented | — | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `clipart.paste_single` | 50383 | Paste Single | pop-up > Paste Single | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `clipart.paste_copy` | 50382 | Paste Copy | pop-up > Paste Copy | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `clipart.reset_transform` | 50387 | Reset | pop-up > Reset | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `clipart.instantiate` | — | Instantiate 3D Clipart |  | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `clipart.transform` | — | Transform 3D Component |  | Implemented | 6 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.group_components` | — | Group Components |  | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `project.ungroup` | — | Ungroup Components |  | Implemented | 1 | `doc.rs` | CANDIDATE — requires pivot and handler review |
| `relief.shape_editor` | 50425 | Shape Editor | Reliefs > Shape Editor... | Implemented | 15 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.two_rail_sweep` | 58111 | Two Rail Sweep... | Reliefs > Swept Profiles > Two Rail Sweep... | Implemented | 8 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.extrude` | 58029 | Extrude... | Reliefs > Swept Profiles > Extrude... | Implemented | 9 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.spin` | 58031 | Spin... | Reliefs > Swept Profiles > Spin... | Implemented | 12 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.turn` | 58032 | Turn... | Reliefs > Swept Profiles > Turn... | Implemented | 8 | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `relief.sculpt` | 58062 | Relief Sculpt | Secrets > Relief Sculpt | Implemented | — | `relief.rs` | CANDIDATE — requires pivot and handler review |
| `cam.tool_database` | 50623 | Tool Database | Toolpaths > Tool Database | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.tool_db_load` | — | Load Tool Database | Toolpaths > Load Tool Database | Implemented | 1 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.tool_db_save` | — | Save Tool Database | Toolpaths > Save Tool Database | Implemented | 1 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.tool_create` | — | Create Tool | Toolpaths > Create Tool | Implemented | 4 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.tool_update` | — | Update Tool | Toolpaths > Update Tool | Implemented | 4 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.tool_delete` | — | Delete Tool | Toolpaths > Delete Tool | Implemented | 1 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.tool_group_create` | — | Create Tool Group | Toolpaths > Create Tool Group | Implemented | 2 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.tool_list` | — | List Tools | Toolpaths > List Tools | Implemented | 1 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.tool_get` | — | Get Tool | Toolpaths > Get Tool | Implemented | 2 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.calculate_feeds_speeds` | — | Calculate Feeds and Speeds | Toolpaths > Calculate Feeds and Speeds | Implemented | 1 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.export_linuxcnc_tbl` | — | Export LinuxCNC Tool Table | Toolpaths > Export LinuxCNC Tool Table | Implemented | 1 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.import_linuxcnc_tbl` | — | Import LinuxCNC Tool Table | Toolpaths > Import LinuxCNC Tool Table | Implemented | 1 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.calculate_toolpath` | 50399 | Create 2D Profile Toolpath | Toolpaths > 2D Toolpaths > Profile | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.profile_2d` | — | Create 2D Profile Toolpath | Toolpaths > 2D Machining > 2D Profiling... | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.profile_options` | 50121 | Profile Options (Bridges) | Toolpaths > Profile Options | Implemented | 4 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.toolpath_list` | — | List Toolpaths | Toolpaths > List Toolpaths | Implemented | 1 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.toolpath_get` | — | Get Toolpath | Toolpaths > Get Toolpath | Implemented | 2 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.toolpath_delete` | 50594 | Delete Toolpath | Toolpaths > Delete Toolpath | Implemented | 2 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.pocket_2d` | 50059 | Area Clearance... | &Toolpaths > New 2D Toolpath > Area Clearance... | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.drill_create` | 50737 | Drilling... | &Toolpaths > New 2D Toolpath > Drilling... | Implemented | 8 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.drill_bank` | 50140 | Drill Banks... | &Toolpaths > New 2D Toolpath > Drill Banks... | Implemented | 4 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.smart_engrave` | 50166 | Smart Engraving... | &Toolpaths > New 2D Toolpath > Smart Engraving... | Implemented | 3 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.vcarve` | 50634 | V-Bit Carving... | &Toolpaths > New 2D Toolpath > V-Bit Carving... | Implemented | 11 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.bevel_carving` | 50080 | Bevel Carving... | &Toolpaths > New 2D Toolpath > Bevel Carving... | Implemented | 11 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.inlay_wizard` | 50226 | Inlay... | &Toolpaths > New 2D Toolpath > Inlay... | Implemented | 14 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.z_level_roughing` | 50794 | Z Level Roughing... | &Toolpaths > New 3D Toolpath > Z Level Roughing... | Implemented | 12 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.relief_3d` | 50766 | Machine Relief... | &Toolpaths > New 3D Toolpath > Machine Relief... | Implemented | 14 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.rest_machining_3d` | 50769 | Rest Machining... | &Toolpaths > New 3D Toolpath > Rest Machining... | Implemented | 15 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.toolpath_merge` | 50586 | Merge Toolpaths... | &Toolpaths > Merge Toolpaths... | Implemented | 2 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.material_setup` | 50609 | Material Setup... | &Toolpaths > Material Setup... | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.delete_material` | 50595 | Delete Material | &Toolpaths > Delete Material | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.toolpath_transform` | 50587 | Transform Toolpath | &Toolpaths > Transform Toolpath | Implemented | 8 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.toolpath_batch_calculate` | 50590 | &Batch Calculate Toolpaths... | &Toolpaths > &Batch Calculate Toolpaths... | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.toolpath_panelling` | 50663 | Toolpath Panelling | &Toolpaths > Toolpath Panelling | Implemented | 5 | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.save_toolpath` | 50617 | &Save Toolpath As... | &Toolpaths > &Save Toolpath As... | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.save_toolpath_alt` | 58228 | &Save Toolpath As... | &Toolpaths > &Save Toolpath As... (Alt) | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.popup_save_toolpath` | 50353 | Save Toolpath... | pop-up > Save Toolpath... | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.popup_save_toolpath_alt` | 58196 | Save Toolpath... (Alt) | pop-up > Save Toolpath... (Alt) | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.save_template` | 50615 | Save Toolpaths As Template... | &Toolpaths > Save Toolpaths As Template... | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.save_template_alt` | 58230 | Save Toolpaths As Template... (Alt) | &Toolpaths > Save Toolpaths As Template... (Alt) | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.post_processor_list` | — | List Post Processors |  | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.simulate_toolpath` | 50619 | Simulate Toolpath | &Toolpaths > Simulation > Simulate Toolpath | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.simulate_toolpath_alt` | 58043 | Simulate Toolpath (Alt) | &Toolpaths > Simulate Toolpath | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.simulate_all` | 50620 | Simulate &All Toolpaths | &Toolpaths > Simulation > Simulate All Toolpaths | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.simulate_all_alt` | 58211 | Simulate &All Toolpaths (Alt) | &Toolpaths > Simulate All Toolpaths | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.reset_simulation` | 50613 | Reset Simulation | &Toolpaths > Simulation > Reset Simulation | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.reset_simulation_alt` | 58044 | Reset Simulation (Alt) | &Toolpaths > Reset Simulation | Implemented | — | `cam.rs` | CANDIDATE — requires pivot and handler review |
| `cam.delete_simulation` | 50596 | Delete Simulation | &Toolpaths > Simulation > Delete Simulation | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.delete_simulation_alt` | 58045 | Delete Simulation (Alt) | &Toolpaths > Delete Simulation | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.popup_simulate` | 50355 | Simulate Toolpath | pop-up > Simulate Toolpath | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.popup_simulate_alt` | 58194 | Simulate Toolpath (Alt) | pop-up > Simulate Toolpath (Alt) | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.popup_simulate_group` | 50356 | Simulate Toolpath Group | pop-up > Simulate Toolpath Group | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.popup_simulate_group_alt` | 58201 | Simulate Toolpath Group (Alt) | pop-up > Simulate Toolpath Group (Alt) | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.check_collisions` | — | Check Toolpath Collisions | &Toolpaths > Simulation > Check Collisions | Implemented | 4 | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.detect_gouging` | — | Detect Design Model Gouging | &Toolpaths > Simulation > Detect Gouging | Implemented | 2 | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.create_relief_from_simulation` | 50593 | Create Relief From Simulation | &Toolpaths > Simulation > Create Relief From Simulation | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.create_relief_from_simulation_alt` | 58410 | Create Relief From Simulation | &Toolpaths > Create Relief From Simulation | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.popup_create_relief_from_simulation` | 50593 | Create Relief Layer | pop-up > Create Relief Layer | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.optifeed` | — | OptiFeed Feedrate Optimizer... | &Toolpaths > Simulation > OptiFeed... | Implemented | 3 | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.optifeed_settings` | — | OptiFeed Settings... | &Toolpaths > Simulation > OptiFeed Settings... | Implemented | — | `sim.rs` | CANDIDATE — requires pivot and handler review |
| `cam.feature_machining_3d` | 50768 | Feature Machining... | &Toolpaths > New 3D Toolpath > Feature Machining... | Draft | — | `cam.rs` | DRAFT — source registry status |

## Acceptance requirement

Every retained command must be reconciled with the actual handler match arm and its downstream domain function. Record typed payload, aliases, defaults, validation, document/session effects, result type, error cases, frontend exposure and acceptance tests. Consolidate aliases into canonical operations. Do not carry over a command simply because it is registered or labelled Implemented.
