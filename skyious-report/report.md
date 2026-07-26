# Skyious Rust Test — .

- **Duration:** 137177ms
- **Total findings:** 4687
- **Errors:** 781 | **Warnings:** 1347 | **Info:** 2559

## By Category

| Category | Count |
|----------|-------|
| compiler | 38 |
| concurrency | 4 |
| correctness | 2444 |
| memory | 40 |
| perf | 279 |
| safety | 7 |
| security | 39 |
| style | 1835 |
| ub | 1 |

## All Findings

| Severity | Category | Runner | File | Line | Message |
|----------|----------|--------|------|------|---------|
| WARN | safety | unsafe_tracker | ./crates/aether-caelum/src/style/compact_length.rs:59 | 59 | integer/address cast to raw pointer — validate address validity |
| WARN | safety | unsafe_tracker | ./crates/aether-caelum/src/style/compact_length.rs:66 | 66 | integer/address cast to raw pointer — validate address validity |
| WARN | safety | unsafe_tracker | ./crates/aether-caelum/src/style/compact_length.rs:105 | 105 | integer/address cast to raw pointer — validate address validity |
| WARN | safety | unsafe_tracker | ./crates/aether-caelum/src/style/compact_length.rs:136 | 136 | integer/address cast to raw pointer — validate address validity |
| WARN | safety | unsafe_tracker | ./crates/aether-caelum/src/style/compact_length.rs:142 | 142 | integer/address cast to raw pointer — validate address validity |
| WARN | safety | unsafe_tracker | ./crates/aether-caelum/src/style/compact_length.rs:178 | 178 | integer/address cast to raw pointer — validate address validity |
| INFO | compiler | unsafe_tracker | ./temp_canvas.rs:0 | 0 | failed to parse file (may use macros not supported by syn) |
| INFO | perf | perf | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:47 | 47 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:56 | 56 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:67 | 67 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:68 | 68 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:90 | 90 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./crates/aether-dom/src/lib.rs:75 | 75 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./korlang/src/vm/mod.rs:43 | 43 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./src/engine/js/events.rs:15 | 15 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./src/engine/js/js_bridge.rs:65 | 65 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./src/engine/js/js_bridge.rs:67 | 67 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./src/engine/js/js_bridge.rs:73 | 73 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./src/engine/js/js_bridge.rs:73 | 73 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./src/engine/js/selector.rs:17 | 17 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./src/engine/net/mod.rs:86 | 86 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./src/engine/pipeline/fetcher.rs:45 | 45 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./src/engine/style.rs:7 | 7 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./src/engine/style.rs:7 | 7 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./src/engine/style.rs:17 | 17 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./src/engine/style.rs:17 | 17 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./src/ui/kor_renderer.rs:9 | 9 | .clone() call — consider whether borrowing is sufficient |
| INFO | perf | perf | ./tests/sidebar_tests.rs:88 | 88 | .clone() call — consider whether borrowing is sufficient |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/block.rs:36 | 36 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/flexbox.rs:220 | 220 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/flexbox.rs:222 | 222 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/flexbox.rs:248 | 248 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/float.rs:175 | 175 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/alignment.rs:51 | 51 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:125 | 125 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:132 | 132 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/placement.rs:22 | 22 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/placement.rs:24 | 24 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/placement.rs:71 | 71 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/placement.rs:126 | 126 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:61 | 61 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:32 | 32 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:39 | 39 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:102 | 102 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:21 | 21 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:39 | 39 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:57 | 57 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:63 | 63 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:71 | 71 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:82 | 82 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:117 | 117 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:209 | 209 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:235 | 235 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/grid_track.rs:90 | 90 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/grid_track.rs:95 | 95 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/grid_track.rs:144 | 144 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:63 | 63 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:68 | 68 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:92 | 92 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/named.rs:20 | 20 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/named.rs:95 | 95 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/named.rs:97 | 97 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/named.rs:99 | 99 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/compute/grid/types/named.rs:101 | 101 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/lib.rs:69 | 69 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/available_space.rs:30 | 30 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/available_space.rs:121 | 121 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/compact_length.rs:12 | 12 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/compact_length.rs:16 | 16 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/compact_length.rs:91 | 91 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/compact_length.rs:166 | 166 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/compact_length.rs:216 | 216 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/compact_length.rs:224 | 224 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/compact_length.rs:235 | 235 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/compact_length.rs:242 | 242 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/compact_length.rs:250 | 250 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:16 | 16 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:21 | 21 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:31 | 31 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:39 | 39 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:48 | 48 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:56 | 56 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:80 | 80 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:85 | 85 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:90 | 90 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:100 | 100 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:108 | 108 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:115 | 115 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:124 | 124 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:132 | 132 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:177 | 177 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:182 | 182 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:187 | 187 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:192 | 192 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:202 | 202 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:210 | 210 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:217 | 217 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:226 | 226 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/dimension.rs:234 | 234 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/grid.rs:234 | 234 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/grid.rs:239 | 239 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/grid.rs:355 | 355 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style/grid.rs:365 | 365 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:25 | 25 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:47 | 47 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:57 | 57 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:105 | 105 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:115 | 115 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:125 | 125 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:135 | 135 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:159 | 159 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:169 | 169 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:179 | 179 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:189 | 189 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:213 | 213 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:223 | 223 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:233 | 233 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:244 | 244 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:268 | 268 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:278 | 278 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:288 | 288 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/style_helpers.rs:299 | 299 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/tree/cache.rs:33 | 33 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/tree/caelum_tree.rs:157 | 157 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/tree/caelum_tree.rs:181 | 181 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/tree/caelum_tree.rs:286 | 286 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/tree/layout.rs:205 | 205 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/tree/layout.rs:210 | 210 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/tree/layout.rs:241 | 241 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/tree/layout.rs:316 | 316 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/tree/layout.rs:326 | 326 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/tree/node.rs:16 | 16 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/tree/node.rs:23 | 23 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/tree/node.rs:35 | 35 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/tree/node.rs:48 | 48 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/print.rs:63 | 63 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/print.rs:65 | 65 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:183 | 183 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:184 | 184 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:185 | 185 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:186 | 186 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:195 | 195 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:196 | 196 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:197 | 197 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:198 | 198 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:208 | 208 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:209 | 209 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:210 | 210 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:211 | 211 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:225 | 225 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:226 | 226 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:227 | 227 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:228 | 228 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:237 | 237 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/resolve.rs:238 | 238 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-caelum/src/util/sys.rs:11 | 11 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-css/src/parser.rs:22 | 22 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-css/src/parser.rs:61 | 61 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-css/src/resolver.rs:12 | 12 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-css/src/resolver.rs:21 | 21 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-css/src/resolver.rs:29 | 29 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-css/src/resolver.rs:72 | 72 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-css/src/resolver.rs:75 | 75 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-css/src/resolver.rs:78 | 78 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-css/src/resolver.rs:81 | 81 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-css/src/style_value.rs:66 | 66 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-html/src/lib.rs:58 | 58 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./crates/aether-html/src/lib.rs:60 | 60 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./korlang/src/compiler/mod.rs:15 | 15 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./korlang/src/compiler/mod.rs:21 | 21 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./korlang/src/compiler/mod.rs:26 | 26 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./korlang/src/compiler/mod.rs:29 | 29 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./korlang/src/compiler/mod.rs:41 | 41 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./korlang/src/compiler/mod.rs:44 | 44 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./korlang/src/compiler/mod.rs:49 | 49 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./korlang/src/compiler/mod.rs:55 | 55 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./korlang/src/compiler/parser.rs:81 | 81 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./korlang/src/compiler/parser.rs:101 | 101 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./korlang/src/vm/mod.rs:90 | 90 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/js/js_bridge.rs:82 | 82 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/js/js_bridge.rs:87 | 87 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/js/js_bridge.rs:93 | 93 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/js/mod.rs:34 | 34 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/js/mod.rs:48 | 48 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/js/mod.rs:64 | 64 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/js/mod.rs:77 | 77 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/js/storage.rs:11 | 11 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/js/storage.rs:36 | 36 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/korlang.rs:21 | 21 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/korlang.rs:24 | 24 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/korlang.rs:26 | 26 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/korlang.rs:36 | 36 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/korlang.rs:49 | 49 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/net/mock.rs:18 | 18 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/net/mod.rs:45 | 45 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/net/mod.rs:47 | 47 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/pipeline/extractor.rs:109 | 109 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/pipeline/fetcher.rs:34 | 34 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/pipeline/fetcher.rs:51 | 51 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/pipeline/fetcher.rs:76 | 76 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/pipeline/layout.rs:10 | 10 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/pipeline/navigator.rs:16 | 16 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/style.rs:7 | 7 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/style.rs:11 | 11 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/style.rs:17 | 17 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/engine/style.rs:21 | 21 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/kor_renderer.rs:9 | 9 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/mod.rs:51 | 51 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/mod.rs:56 | 56 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/mod.rs:67 | 67 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/mod.rs:76 | 76 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/mod.rs:87 | 87 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/screens/browser.rs:231 | 231 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/screens/browser.rs:233 | 233 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/screens/palette.rs:35 | 35 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/screens/palette.rs:51 | 51 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/screens/palette.rs:161 | 161 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/screens/palette.rs:206 | 206 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/screens/settings.rs:96 | 96 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/screens/settings.rs:103 | 103 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/screens/settings.rs:114 | 114 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/screens/settings.rs:124 | 124 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/style.rs:114 | 114 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/style.rs:134 | 134 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/style.rs:153 | 153 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/style.rs:157 | 157 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/style.rs:178 | 178 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./src/ui/style.rs:182 | 182 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/css_regression.rs:10 | 10 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/css_regression.rs:20 | 20 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/js_bridge_test.rs:10 | 10 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/js_bridge_test.rs:18 | 18 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/korlang_advanced_tests.rs:14 | 14 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/korlang_advanced_tests.rs:19 | 19 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/layout_stress.rs:73 | 73 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/layout_stress.rs:74 | 74 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/layout_stress.rs:85 | 85 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/layout_stress.rs:86 | 86 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/layout_stress.rs:97 | 97 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/layout_stress.rs:98 | 98 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/layout_stress.rs:110 | 110 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/layout_stress.rs:111 | 111 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/rendering_pipeline_tests.rs:14 | 14 | expression statement with unused result — may discard error |
| INFO | correctness | correctness | ./tests/rendering_pipeline_tests.rs:24 | 24 | expression statement with unused result — may discard error |
| WARN | security | security | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:623 | 623 | unconditional panic — `unreachable!()` may hide logic bugs |
| WARN | security | security | ./crates/aether-caelum/src/compute/grid/types/named.rs:280 | 280 | unconditional panic — `unreachable!()` may hide logic bugs |
| WARN | security | security | ./crates/aether-caelum/src/compute/grid/types/named.rs:287 | 287 | unconditional panic — `unreachable!()` may hide logic bugs |
| WARN | security | security | ./crates/aether-caelum/src/compute/leaf.rs:137 | 137 | unconditional panic — `unreachable!()` may hide logic bugs |
| WARN | security | security | ./crates/aether-caelum/src/style/compact_length.rs:456 | 456 | unconditional panic — `unreachable!()` may hide logic bugs |
| WARN | security | security | ./crates/aether-caelum/src/style/dimension.rs:151 | 151 | unconditional panic — `unreachable!()` may hide logic bugs |
| WARN | security | security | ./crates/aether-caelum/src/util/resolve.rs:36 | 36 | unconditional panic — `unreachable!()` may hide logic bugs |
| WARN | security | security | ./crates/aether-caelum/src/util/resolve.rs:50 | 50 | unconditional panic — `unreachable!()` may hide logic bugs |
| WARN | security | security | ./crates/aether-caelum/src/util/resolve.rs:65 | 65 | unconditional panic — `unreachable!()` may hide logic bugs |
| ERROR | security | security | ./crates/aether-css/build.rs:39 | 39 | format string vulnerability — `format!` first arg is not a string literal |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/block.rs:622 | 622 | variable `width` is shadowed in the same block |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/block.rs:856 | 856 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/block.rs:1167 | 1167 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/block.rs:1179 | 1179 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/common/alignment.rs:75 | 75 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/common/alignment.rs:82 | 82 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/common/alignment.rs:97 | 97 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/common/alignment.rs:98 | 98 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/common/alignment.rs:99 | 99 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/flexbox.rs:1097 | 1097 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/flexbox.rs:1100 | 1100 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/flexbox.rs:1303 | 1303 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/flexbox.rs:1314 | 1314 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/flexbox.rs:1576 | 1576 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/flexbox.rs:1675 | 1675 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/flexbox.rs:2255 | 2255 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/flexbox.rs:2263 | 2263 | division by non-literal — possible division by zero |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/float.rs:298 | 298 | variable `start_idx` is shadowed in the same block |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/float.rs:333 | 333 | infinite loop: no `break` found in loop body |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/float.rs:410 | 410 | variable `start_idx` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/float.rs:423 | 423 | variable `end_idx` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/float.rs:498 | 498 | variable `start_idx` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/grid/alignment.rs:37 | 37 | variable `track_alignment` is shadowed in the same block |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/grid/alignment.rs:302 | 302 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:161 | 161 | division by non-literal — possible division by zero |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:263 | 263 | variable `track_iter` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/grid/mod.rs:597 | 597 | variable `maybe_col_indexes` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/grid/placement.rs:142 | 142 | variable `idx` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/grid/placement.rs:195 | 195 | variable `primary_axis` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/grid/placement.rs:196 | 196 | variable `secondary_axis` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/grid/placement.rs:211 | 211 | variable `idx` is shadowed in the same block |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/grid/placement.rs:327 | 327 | infinite loop: no `break` found in loop body |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/grid/placement.rs:408 | 408 | infinite loop: no `break` found in loop body |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/grid/placement.rs:427 | 427 | infinite loop: no `break` found in loop body |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:232 | 232 | division by non-literal — possible division by zero |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:625 | 625 | variable `track` is shadowed in the same block |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1105 | 1105 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1209 | 1209 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1305 | 1305 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1351 | 1351 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1393 | 1393 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1397 | 1397 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:25 | 25 | division by non-literal — possible division by zero |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/grid/types/named.rs:147 | 147 | variable `current_line` is shadowed in the same block |
| ERROR | correctness | logic | ./crates/aether-caelum/src/compute/leaf.rs:147 | 147 | division by non-literal — possible division by zero |
| INFO | correctness | logic | ./crates/aether-caelum/src/compute/leaf.rs:149 | 149 | variable `size` is shadowed in the same block |
| ERROR | correctness | logic | ./crates/aether-caelum/src/geometry.rs:559 | 559 | division by non-literal — possible division by zero |
| INFO | correctness | logic | ./crates/aether-caelum/src/lib.rs:275 | 275 | variable `s` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/lib.rs:277 | 277 | variable `s` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/lib.rs:285 | 285 | variable `s` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/lib.rs:300 | 300 | variable `s` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/lib.rs:303 | 303 | variable `s` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/style/mod.rs:1226 | 1226 | variable `name` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/style/mod.rs:1227 | 1227 | variable `name` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/style/mod.rs:1228 | 1228 | variable `name` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/style/mod.rs:1229 | 1229 | variable `name` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-caelum/src/style/mod.rs:1230 | 1230 | variable `name` is shadowed in the same block |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:158 | 158 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:319 | 319 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:327 | 327 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:361 | 361 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:362 | 362 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:363 | 363 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:364 | 364 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:372 | 372 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:373 | 373 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:374 | 374 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:382 | 382 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:383 | 383 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:384 | 384 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:385 | 385 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:394 | 394 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:395 | 395 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/parser.rs:396 | 396 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/style_value.rs:46 | 46 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/style_value.rs:47 | 47 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/style_value.rs:48 | 48 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/style_value.rs:52 | 52 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/style_value.rs:53 | 53 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/style_value.rs:54 | 54 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/style_value.rs:58 | 58 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/style_value.rs:59 | 59 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/style_value.rs:60 | 60 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/style_value.rs:61 | 61 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-css/src/style_value.rs:264 | 264 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./crates/aether-html/src/lib.rs:56 | 56 | variable `text` is shadowed in the same block |
| INFO | correctness | logic | ./examples/korlang_repl.rs:15 | 15 | variable `input` is shadowed in the same block |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:79 | 79 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:100 | 100 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:133 | 133 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:155 | 155 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:200 | 200 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:202 | 202 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:209 | 209 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:211 | 211 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:218 | 218 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:222 | 222 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:230 | 230 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:234 | 234 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:242 | 242 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:246 | 246 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:257 | 257 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:265 | 265 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./korlang/src/compiler/parser.rs:284 | 284 | `?` operator used in non-Result-returning function |
| ERROR | correctness | logic | ./korlang/src/vm/mod.rs:138 | 138 | division by non-literal — possible division by zero |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:253 | 253 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:254 | 254 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:257 | 257 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:258 | 258 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:260 | 260 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:263 | 263 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:264 | 264 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:265 | 265 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:317 | 317 | variable `rest` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:323 | 323 | variable `rest` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:329 | 329 | variable `rest` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:913 | 913 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:913 | 913 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:914 | 914 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:915 | 915 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:920 | 920 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:920 | 920 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:921 | 921 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:922 | 922 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1017 | 1017 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1018 | 1018 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1610 | 1610 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1619 | 1619 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1628 | 1628 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1637 | 1637 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1646 | 1646 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1655 | 1655 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1664 | 1664 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1673 | 1673 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1682 | 1682 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1691 | 1691 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1700 | 1700 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1710 | 1710 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1719 | 1719 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1729 | 1729 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1738 | 1738 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1747 | 1747 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1756 | 1756 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1765 | 1765 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1774 | 1774 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1783 | 1783 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1792 | 1792 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1802 | 1802 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1811 | 1811 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1820 | 1820 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1830 | 1830 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1839 | 1839 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1849 | 1849 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1858 | 1858 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1868 | 1868 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1875 | 1875 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1882 | 1882 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1889 | 1889 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1896 | 1896 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1903 | 1903 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1910 | 1910 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1917 | 1917 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1924 | 1924 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1931 | 1931 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1938 | 1938 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1946 | 1946 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1966 | 1966 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1977 | 1977 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1987 | 1987 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:1998 | 1998 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:2007 | 2007 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:2016 | 2016 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:2025 | 2025 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:2036 | 2036 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:2048 | 2048 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:2055 | 2055 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:2063 | 2063 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:2070 | 2070 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:2080 | 2080 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:2090 | 2090 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:2097 | 2097 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/js_bridge.rs:2104 | 2104 | variable `b1` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/js/selector.rs:5 | 5 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/korlang.rs:7 | 7 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/korlang.rs:11 | 11 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/net/mod.rs:83 | 83 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/net/mod.rs:112 | 112 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/net/mod.rs:113 | 113 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/engine/net/mod.rs:132 | 132 | variable `last` is shadowed in the same block |
| INFO | correctness | logic | ./src/engine/net/mod.rs:528 | 528 | variable `hostname` is shadowed in the same block |
| ERROR | correctness | logic | ./src/engine/pipeline/fetcher.rs:329 | 329 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./src/engine/pipeline/layout.rs:18 | 18 | division by non-literal — possible division by zero |
| ERROR | correctness | logic | ./src/engine/pipeline/layout.rs:112 | 112 | division by non-literal — possible division by zero |
| INFO | correctness | logic | ./src/ui/kor_renderer.rs:145 | 145 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/ui/kor_renderer.rs:146 | 146 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/ui/kor_renderer.rs:147 | 147 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/ui/kor_renderer.rs:150 | 150 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/ui/kor_renderer.rs:151 | 151 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/ui/kor_renderer.rs:152 | 152 | `?` operator used in non-Result-returning function |
| INFO | correctness | logic | ./src/ui/screens/browser.rs:795 | 795 | variable `total_h` is shadowed in the same block |
| INFO | correctness | logic | ./tests/integration_test.rs:84 | 84 | variable `first_p` is shadowed in the same block |
| INFO | correctness | logic | ./tests/integration_test.rs:89 | 89 | variable `highlight` is shadowed in the same block |
| INFO | correctness | logic | ./tests/integration_test.rs:97 | 97 | variable `container` is shadowed in the same block |
| INFO | correctness | logic | ./tests/integration_test.rs:100 | 100 | variable `first_p` is shadowed in the same block |
| WARN | memory | runtime | ./crates/aether-caelum/src/style_helpers.rs:306 | 306 | direct recursion in `fit_content` — risk of stack overflow |
| WARN | memory | runtime | ./crates/aether-caelum/src/tree/caelum_tree.rs:861 | 861 | direct recursion in `mark_dirty_recursive` — risk of stack overflow |
| WARN | memory | runtime | ./crates/aether-caelum/src/util/print.rs:61 | 61 | direct recursion in `write_node` — risk of stack overflow |
| WARN | memory | runtime | ./crates/aether-css/src/parser.rs:407 | 407 | direct recursion in `parse_color_function` — risk of stack overflow |
| WARN | memory | runtime | ./korlang/src/compiler/formatter.rs:43 | 43 | direct recursion in `format_node` — risk of stack overflow |
| WARN | memory | runtime | ./korlang/src/compiler/mod.rs:80 | 80 | direct recursion in `emit_expr` — risk of stack overflow |
| WARN | memory | runtime | ./src/engine/js/js_bridge.rs:146 | 146 | direct recursion in `parse_complex` — risk of stack overflow |
| WARN | memory | runtime | ./src/engine/js/js_bridge.rs:161 | 161 | direct recursion in `inner` — risk of stack overflow |
| WARN | memory | runtime | ./src/engine/korlang.rs:67 | 67 | direct recursion in `register_default_callbacks` — risk of stack overflow |
| WARN | memory | runtime | ./src/engine/pipeline/extractor.rs:188 | 188 | direct recursion in `collect` — risk of stack overflow |
| WARN | memory | runtime | ./src/engine/pipeline/extractor.rs:376 | 376 | direct recursion in `extract_elements` — risk of stack overflow |
| WARN | memory | runtime | ./src/engine/pipeline/extractor.rs:622 | 622 | direct recursion in `collect` — risk of stack overflow |
| WARN | memory | runtime | ./src/engine/pipeline/extractor.rs:675 | 675 | direct recursion in `walk` — risk of stack overflow |
| WARN | memory | runtime | ./src/engine/pipeline/fetcher.rs:34 | 34 | direct recursion in `extract_styles` — risk of stack overflow |
| WARN | memory | runtime | ./src/engine/pipeline/fetcher.rs:51 | 51 | direct recursion in `extract_links` — risk of stack overflow |
| WARN | memory | runtime | ./src/engine/pipeline/fetcher.rs:88 | 88 | direct recursion in `extract_scripts` — risk of stack overflow |
| WARN | memory | runtime | ./src/ui/kor_renderer.rs:23 | 23 | direct recursion in `convert_object` — risk of stack overflow |
| WARN | memory | runtime | ./tests/js_bridge_test.rs:30 | 30 | direct recursion in `find_in_dom` — risk of stack overflow |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/block.rs:154 | 154 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/common/alignment.rs:75 | 75 | integer division with runtime right operand — possible division by zero |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/common/alignment.rs:82 | 82 | integer division with runtime right operand — possible division by zero |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/common/alignment.rs:97 | 97 | integer division with runtime right operand — possible division by zero |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/common/alignment.rs:98 | 98 | integer division with runtime right operand — possible division by zero |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/common/alignment.rs:99 | 99 | integer division with runtime right operand — possible division by zero |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/common/content_size.rs:30 | 30 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/common/content_size.rs:30 | 30 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/float.rs:77 | 77 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/float.rs:78 | 78 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/float.rs:78 | 78 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/float.rs:119 | 119 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/float.rs:119 | 119 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/float.rs:119 | 119 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/alignment.rs:24 | 24 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/alignment.rs:74 | 74 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/alignment.rs:74 | 74 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:67 | 67 | cast to u16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:67 | 67 | cast to unsigned type u16 — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:44 | 44 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:44 | 44 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:49 | 49 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:49 | 49 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:87 | 87 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:88 | 88 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:89 | 89 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:143 | 143 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/placement.rs:46 | 46 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/placement.rs:55 | 55 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/placement.rs:57 | 57 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/placement.rs:58 | 58 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:25 | 25 | integer division with runtime right operand — possible division by zero |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:120 | 120 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:36 | 36 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:63 | 63 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:71 | 71 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:76 | 76 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:82 | 82 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:82 | 82 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:109 | 109 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:113 | 113 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:117 | 117 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:117 | 117 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:131 | 131 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:132 | 132 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:160 | 160 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:160 | 160 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:58 | 58 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:63 | 63 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:68 | 68 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:78 | 78 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:92 | 92 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:92 | 92 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:92 | 92 | cast to i16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:98 | 98 | cast to u16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:98 | 98 | cast to unsigned type u16 — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:99 | 99 | cast to u16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:99 | 99 | cast to unsigned type u16 — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/geometry.rs:349 | 349 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/geometry.rs:349 | 349 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/available_space.rs:102 | 102 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/available_space.rs:111 | 111 | subtraction with runtime right operand — possible underflow without checked_sub |
| ERROR | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:59 | 59 | shift by 32 — may exceed 32-bit width |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:59 | 59 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:73 | 73 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:79 | 79 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:91 | 91 | cast to unsigned type u32 — possible sign loss if source is signed |
| ERROR | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:91 | 91 | shift by 32 — may exceed 32-bit width |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:91 | 91 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:98 | 98 | cast to unsigned type u64 — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:98 | 98 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:105 | 105 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:136 | 136 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:166 | 166 | cast to unsigned type u32 — possible sign loss if source is signed |
| ERROR | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:172 | 172 | shift by 32 — may exceed 32-bit width |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:172 | 172 | cast to unsigned type u64 — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:172 | 172 | cast to unsigned type u64 — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:178 | 178 | cast to unsigned type usize — possible sign loss if source is signed |
| ERROR | correctness | overflow | ./crates/aether-caelum/src/style/compact_length.rs:178 | 178 | shift by 32 — may exceed 32-bit width |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/grid.rs:75 | 75 | cast to u16 may truncate value from larger integer type |
| WARN | correctness | overflow | ./crates/aether-caelum/src/style/grid.rs:75 | 75 | cast to unsigned type u16 — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/cache.rs:85 | 85 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/cache.rs:91 | 91 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/layout.rs:286 | 286 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/layout.rs:286 | 286 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/layout.rs:286 | 286 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/layout.rs:286 | 286 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/layout.rs:292 | 292 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/layout.rs:292 | 292 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/layout.rs:292 | 292 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/layout.rs:292 | 292 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/layout.rs:318 | 318 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/layout.rs:328 | 328 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/node.rs:35 | 35 | cast to unsigned type u64 — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/tree/node.rs:41 | 41 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./crates/aether-caelum/src/util/math.rs:66 | 66 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/util/math.rs:92 | 92 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/util/math.rs:129 | 129 | subtraction with runtime right operand — possible underflow without checked_sub |
| WARN | correctness | overflow | ./crates/aether-caelum/src/util/sys.rs:31 | 31 | .abs() on minimum signed integer value (iN::MIN) causes overflow/panic in debug |
| WARN | correctness | overflow | ./src/engine/js/events.rs:29 | 29 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./src/engine/js/selector.rs:17 | 17 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./src/engine/js/selector.rs:23 | 23 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./src/engine/pipeline/layout.rs:18 | 18 | cast to unsigned type usize — possible sign loss if source is signed |
| WARN | correctness | overflow | ./src/engine/pipeline/layout.rs:18 | 18 | integer division with runtime right operand — possible division by zero |
| WARN | correctness | overflow | ./tests/rendering_pipeline_tests.rs:698 | 698 | subtraction with runtime right operand — possible underflow without checked_sub |
| INFO | concurrency | datarace | ./korlang/src/vm/mod.rs:229 | 229 | Arc<Mutex<T>> — consider RwLock if reads dominate writes |
| INFO | concurrency | datarace | ./src/engine/pipeline/fetcher.rs:246 | 246 | Arc<Mutex<T>> — consider RwLock if reads dominate writes |
| INFO | concurrency | datarace | ./tests/korlang_advanced_tests.rs:56 | 56 | Arc<Mutex<T>> — consider RwLock if reads dominate writes |
| INFO | concurrency | datarace | ./tests/korlang_advanced_tests.rs:80 | 80 | Arc<Mutex<T>> — consider RwLock if reads dominate writes |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/block.rs:233 | 233 | unused public function `compute_block_layout` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/block.rs:4 | 4 | unused import `CaelumMaxContent` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/block.rs:9 | 9 | unused import `MaybeMath` |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/compute/common/alignment.rs:5 | 5 | private function `apply_alignment_fallback` is never called |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/compute/common/alignment.rs:37 | 37 | private function `compute_alignment_offset` is never called |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/compute/common/content_size.rs:6 | 6 | private function `compute_content_size_contribution` is never called |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/flexbox.rs:163 | 163 | unused public function `compute_flexbox_layout` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/flexbox.rs:13 | 13 | unused import `MaybeMath` |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/alignment.rs:12 | 12 | private function `align_tracks` is never called |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/alignment.rs:64 | 64 | private function `align_and_position_item` is never called |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:6 | 6 | unused import `CaelumAuto` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:8 | 8 | unused import `MaybeMath` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:9 | 9 | unused import `ResolveOrZero` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/mod.rs:35 | 35 | unused public function `compute_grid_layout` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/mod.rs:8 | 8 | unused import `MaybeMath` |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:238 | 238 | private function `resolve_item_track_indexes` is never called |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:247 | 247 | private function `determine_if_item_crosses_flexible_or_intrinsic_tracks` is never called |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:266 | 266 | private function `track_sizing_algorithm` is never called |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:6 | 6 | unused import `CaelumMinContent` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:8 | 8 | unused import `Debug` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/types/mod.rs:12 | 12 | unused import `GridItem` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/types/mod.rs:13 | 13 | unused import `GridTrack` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/types/mod.rs:14 | 14 | unused import `TrackCounts` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/types/mod.rs:15 | 15 | unused import `NamedLineResolver` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/types/mod.rs:17 | 17 | unused import `GridTrackKind` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/types/named.rs:9 | 9 | unused import `Borrow` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/grid/types/named.rs:11 | 11 | unused import `Debug` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/leaf.rs:13 | 13 | unused public function `compute_leaf_layout` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/leaf.rs:8 | 8 | unused import `MaybeMath` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/mod.rs:54 | 54 | unused public function `compute_root_layout` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/mod.rs:159 | 159 | unused public function `compute_cached_layout` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/mod.rs:196 | 196 | unused public function `round_layout` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/mod.rs:35 | 35 | unused import `compute_leaf_layout` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/mod.rs:39 | 39 | unused import `compute_flexbox_layout` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/mod.rs:41 | 41 | unused import `compute_grid_layout` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/compute/mod.rs:51 | 51 | unused import `ResolveOrZero` |
| WARN | correctness | deadcode | ./crates/aether-caelum/src/geometry.rs:693 | 693 | unused struct `MinMax` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/geometry.rs:281 | 281 | unused associated const `ZERO` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/geometry.rs:313 | 313 | unused associated const `TRUE` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/geometry.rs:315 | 315 | unused associated const `FALSE` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/geometry.rs:508 | 508 | unused associated const `ZERO` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/geometry.rs:612 | 612 | unused associated const `ZERO` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/lib.rs:56 | 56 | unused import `CacheTree` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/prelude.rs:26 | 26 | unused import `CaelumTree` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/alignment.rs:42 | 42 | unused type alias `JustifyItems` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/alignment.rs:48 | 48 | unused type alias `AlignSelf` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/alignment.rs:56 | 56 | unused type alias `JustifySelf` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/alignment.rs:127 | 127 | unused type alias `JustifyContent` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/alignment.rs:24 | 24 | unused enum variant `AlignItems::Center` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/alignment.rs:26 | 26 | unused enum variant `AlignItems::Baseline` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/alignment.rs:86 | 86 | unused enum variant `AlignContent::Center` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/alignment.rs:90 | 90 | unused enum variant `AlignContent::SpaceBetween` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/alignment.rs:93 | 93 | unused enum variant `AlignContent::SpaceEvenly` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/alignment.rs:96 | 96 | unused enum variant `AlignContent::SpaceAround` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/available_space.rs:20 | 20 | unused associated const `ZERO` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/available_space.rs:23 | 23 | unused associated const `MAX_CONTENT` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/available_space.rs:26 | 26 | unused associated const `MIN_CONTENT` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/block.rs:37 | 37 | unused enum variant `TextAlign::Auto` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/block.rs:40 | 40 | unused enum variant `TextAlign::LegacyLeft` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/block.rs:42 | 42 | unused enum variant `TextAlign::LegacyRight` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/block.rs:44 | 44 | unused enum variant `TextAlign::LegacyCenter` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/compact_length.rs:427 | 427 | unused associated const `AUTO` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/compact_length.rs:430 | 430 | unused associated const `MIN_CONTENT` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/compact_length.rs:433 | 433 | unused associated const `MAX_CONTENT` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/flex.rs:78 | 78 | unused enum variant `FlexWrap::NoWrap` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/flex.rs:81 | 81 | unused enum variant `FlexWrap::Wrap` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/flex.rs:83 | 83 | unused enum variant `FlexWrap::WrapReverse` |
| WARN | correctness | deadcode | ./crates/aether-caelum/src/style/float.rs:53 | 53 | unused enum `Clear` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/float.rs:63 | 63 | unused enum variant `Clear::Both` |
| WARN | correctness | deadcode | ./crates/aether-caelum/src/style/grid.rs:29 | 29 | unused struct `NamedGridLine` |
| WARN | correctness | deadcode | ./crates/aether-caelum/src/style/grid.rs:1247 | 1247 | unused struct `GridAutoTracks` |
| WARN | correctness | deadcode | ./crates/aether-caelum/src/style/grid.rs:38 | 38 | unused enum `GridAreaAxis` |
| WARN | correctness | deadcode | ./crates/aether-caelum/src/style/grid.rs:47 | 47 | unused enum `GridAreaEnd` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/grid.rs:11 | 11 | unused import `Debug` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/grid.rs:50 | 50 | unused enum variant `GridAreaEnd::Start` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/grid.rs:52 | 52 | unused enum variant `GridAreaEnd::End` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/mod.rs:13 | 13 | unused import `AvailableSpace` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/mod.rs:14 | 14 | unused import `CompactLength` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/mod.rs:32 | 32 | unused import `Debug` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/mod.rs:263 | 263 | unused enum variant `BoxSizing::ContentBox` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style/mod.rs:339 | 339 | unused enum variant `Direction::Rtl` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style_helpers.rs:32 | 32 | unused public function `evenly_sized_tracks` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style_helpers.rs:42 | 42 | unused public function `line` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style_helpers.rs:65 | 65 | unused public function `minmax` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/style_helpers.rs:16 | 16 | unused import `Debug` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/caelum_tree.rs:18 | 18 | unused import `CacheTree` |
| WARN | correctness | deadcode | ./crates/aether-caelum/src/tree/layout.rs:334 | 334 | unused enum `DetailedLayoutInfo` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/layout.rs:4 | 4 | unused import `CaelumMaxContent` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/layout.rs:184 | 184 | unused associated const `DEFAULT` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/layout.rs:10 | 10 | unused enum variant `RunMode::PerformLayout` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/layout.rs:12 | 12 | unused enum variant `RunMode::ComputeSize` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/layout.rs:22 | 22 | unused enum variant `SizingMode::ContentSize` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/layout.rs:337 | 337 | unused enum variant `DetailedLayoutInfo::Grid` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/layout.rs:339 | 339 | unused enum variant `DetailedLayoutInfo::None` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/mod.rs:14 | 14 | unused import `NodeId` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/mod.rs:15 | 15 | unused import `LayoutPartialTreeExt` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/mod.rs:18 | 18 | unused import `LayoutFlexboxContainer` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/mod.rs:20 | 20 | unused import `LayoutGridContainer` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/mod.rs:22 | 22 | unused import `LayoutBlockContainer` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/mod.rs:27 | 27 | unused import `DetailedLayoutInfo` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/tree/traits.rs:134 | 134 | unused import `CheapCloneStr` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/util/mod.rs:6 | 6 | unused import `MaybeMath` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/util/mod.rs:14 | 14 | unused import `print_tree` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/util/mod.rs:15 | 15 | unused import `write_tree` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/util/print.rs:5 | 5 | unused public function `print_tree` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/util/resolve.rs:5 | 5 | unused import `CaelumZero` |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/util/sys.rs:10 | 10 | private function `new_vec_with_capacity` is never called |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/util/sys.rs:14 | 14 | private function `single_value_vec` is never called |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/util/sys.rs:18 | 18 | private function `round` is never called |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/util/sys.rs:22 | 22 | private function `ceil` is never called |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/util/sys.rs:26 | 26 | private function `floor` is never called |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/util/sys.rs:30 | 30 | private function `abs` is never called |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/util/sys.rs:34 | 34 | private function `f32_max` is never called |
| ERROR | correctness | deadcode | ./crates/aether-caelum/src/util/sys.rs:38 | 38 | private function `f32_min` is never called |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/util/sys.rs:4 | 4 | unused type alias `DefaultCheapStr` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/util/sys.rs:5 | 5 | unused type alias `Map` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/util/sys.rs:7 | 7 | unused type alias `ChildrenVec` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/util/sys.rs:8 | 8 | unused type alias `GridTrackVec` |
| INFO | correctness | deadcode | ./crates/aether-caelum/src/util/sys.rs:1 | 1 | unused import `format` |
| INFO | correctness | deadcode | ./crates/aether-css/src/lib.rs:10 | 10 | unused import `CssPropertyName` |
| INFO | correctness | deadcode | ./crates/aether-css/src/matcher.rs:123 | 123 | unused public function `match_rules` |
| INFO | correctness | deadcode | ./crates/aether-css/src/parser.rs:35 | 35 | unused enum variant `Selector::Composite` |
| INFO | correctness | deadcode | ./crates/aether-css/src/parser.rs:77 | 77 | unused enum variant `PropertyValue::Shorthand` |
| ERROR | correctness | deadcode | ./crates/aether-css/src/resolver.rs:27 | 27 | private function `apply_declarations` is never called |
| ERROR | correctness | deadcode | ./crates/aether-css/src/resolver.rs:184 | 184 | private function `resolve_length_for_unit` is never called |
| INFO | correctness | deadcode | ./crates/aether-css/src/resolver.rs:430 | 430 | unused public function `resolve_styles_for_tree` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:10 | 10 | unused enum variant `Display::Block` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:12 | 12 | unused enum variant `Display::InlineBlock` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:14 | 14 | unused enum variant `Display::Flex` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:16 | 16 | unused enum variant `Display::InlineFlex` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:18 | 18 | unused enum variant `Display::Grid` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:295 | 295 | unused enum variant `Position::Relative` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:297 | 297 | unused enum variant `Position::Absolute` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:299 | 299 | unused enum variant `Position::Fixed` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:301 | 301 | unused enum variant `Position::Sticky` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:308 | 308 | unused enum variant `FlexDirection::Row` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:311 | 311 | unused enum variant `FlexDirection::RowReverse` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:313 | 313 | unused enum variant `FlexDirection::Column` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:315 | 315 | unused enum variant `FlexDirection::ColumnReverse` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:328 | 328 | unused enum variant `FlexWrap::NoWrap` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:331 | 331 | unused enum variant `FlexWrap::Wrap` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:333 | 333 | unused enum variant `FlexWrap::WrapReverse` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:340 | 340 | unused enum variant `JustifyContent::FlexStart` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:343 | 343 | unused enum variant `JustifyContent::FlexEnd` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:345 | 345 | unused enum variant `JustifyContent::Center` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:347 | 347 | unused enum variant `JustifyContent::SpaceBetween` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:349 | 349 | unused enum variant `JustifyContent::SpaceAround` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:351 | 351 | unused enum variant `JustifyContent::SpaceEvenly` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:358 | 358 | unused enum variant `AlignItems::Stretch` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:361 | 361 | unused enum variant `AlignItems::FlexStart` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:363 | 363 | unused enum variant `AlignItems::FlexEnd` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:365 | 365 | unused enum variant `AlignItems::Center` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:367 | 367 | unused enum variant `AlignItems::Baseline` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:374 | 374 | unused enum variant `AlignSelf::Auto` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:377 | 377 | unused enum variant `AlignSelf::FlexStart` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:379 | 379 | unused enum variant `AlignSelf::FlexEnd` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:381 | 381 | unused enum variant `AlignSelf::Center` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:383 | 383 | unused enum variant `AlignSelf::Baseline` |
| INFO | correctness | deadcode | ./crates/aether-css/src/style_value.rs:385 | 385 | unused enum variant `AlignSelf::Stretch` |
| INFO | correctness | deadcode | ./korlang/src/compiler/formatter.rs:4 | 4 | unused public function `format_component` |
| ERROR | correctness | deadcode | ./korlang/src/compiler/formatter.rs:107 | 107 | private function `format_token` is never called |
| INFO | correctness | deadcode | ./korlang/src/compiler/mod.rs:10 | 10 | unused public function `compile` |
| ERROR | correctness | deadcode | ./src/engine/js/js_bridge.rs:157 | 157 | private function `matches_complex` is never called |
| ERROR | correctness | deadcode | ./src/engine/js/js_bridge.rs:233 | 233 | private function `parse_cookie_expiry` is never called |
| ERROR | correctness | deadcode | ./src/engine/js/js_bridge.rs:277 | 277 | private function `sweep_expired_cookies` is never called |
| ERROR | correctness | deadcode | ./src/engine/js/js_bridge.rs:283 | 283 | private function `cookie_store` is never called |
| ERROR | correctness | deadcode | ./src/engine/js/js_bridge.rs:288 | 288 | private function `local_storage_store` is never called |
| INFO | correctness | deadcode | ./src/engine/js/js_bridge.rs:1560 | 1560 | unused public function `register_browser_api` |
| INFO | correctness | deadcode | ./src/engine/korlang.rs:6 | 6 | unused public function `take_window_title` |
| INFO | correctness | deadcode | ./src/engine/korlang.rs:10 | 10 | unused public function `take_navigation_url` |
| INFO | correctness | deadcode | ./src/engine/korlang.rs:15 | 15 | unused public function `eval_korlang` |
| INFO | correctness | deadcode | ./src/engine/net/mock.rs:30 | 30 | unused public function `set_mock` |
| INFO | correctness | deadcode | ./src/engine/net/mock.rs:34 | 34 | unused public function `clear_mock` |
| INFO | correctness | deadcode | ./src/engine/net/mock.rs:38 | 38 | unused public function `resolve_html` |
| INFO | correctness | deadcode | ./src/engine/net/mock.rs:45 | 45 | unused public function `resolve_css` |
| INFO | correctness | deadcode | ./src/engine/net/mock.rs:52 | 52 | unused public function `resolve_binary` |
| INFO | correctness | deadcode | ./src/engine/net/mod.rs:428 | 428 | unused public function `get_csp_for` |
| INFO | correctness | deadcode | ./src/engine/net/mod.rs:449 | 449 | unused public function `csp_blocks_scripts` |
| INFO | correctness | deadcode | ./src/engine/net/mod.rs:456 | 456 | unused public function `csp_blocks_styles` |
| INFO | correctness | deadcode | ./src/engine/net/mod.rs:463 | 463 | unused public function `csp_allows_script_url` |
| INFO | correctness | deadcode | ./src/engine/net/mod.rs:472 | 472 | unused public function `csp_allows_style_url` |
| INFO | correctness | deadcode | ./src/engine/net/mod.rs:481 | 481 | unused public function `csp_allows_image_url` |
| INFO | correctness | deadcode | ./src/engine/net/mod.rs:490 | 490 | unused public function `csp_allows_connect_url` |
| INFO | correctness | deadcode | ./src/engine/net/mod.rs:499 | 499 | unused public function `csp_allows_inline_script` |
| INFO | correctness | deadcode | ./src/engine/net/mod.rs:504 | 504 | unused public function `csp_allows_inline_style` |
| INFO | correctness | deadcode | ./src/engine/net/mod.rs:557 | 557 | unused public function `fetch` |
| INFO | correctness | deadcode | ./src/engine/net/mod.rs:584 | 584 | unused public function `fetch_with_cors` |
| INFO | correctness | deadcode | ./src/engine/net/mod.rs:709 | 709 | unused public function `fetch_bytes` |
| INFO | correctness | deadcode | ./src/engine/parser.rs:1 | 1 | unused import `Parser` |
| ERROR | correctness | deadcode | ./src/engine/pipeline/extractor.rs:649 | 649 | private function `extract_elements_flat` is never called |
| INFO | correctness | deadcode | ./src/engine/pipeline/fetcher.rs:135 | 135 | unused public function `fetch_page_content` |
| INFO | correctness | deadcode | ./src/engine/pipeline/layout.rs:102 | 102 | unused public function `apply_caelum_layout` |
| INFO | correctness | deadcode | ./src/engine/pipeline/mod.rs:15 | 15 | unused public function `set_js_enabled` |
| INFO | correctness | deadcode | ./src/engine/pipeline/mod.rs:19 | 19 | unused public function `is_js_enabled` |
| INFO | correctness | deadcode | ./src/engine/pipeline/mod.rs:6 | 6 | unused import `StyledElement` |
| INFO | correctness | deadcode | ./src/engine/pipeline/mod.rs:7 | 7 | unused import `fetch_page_content` |
| INFO | correctness | deadcode | ./src/engine/pipeline/mod.rs:8 | 8 | unused import `apply_caelum_layout` |
| INFO | correctness | deadcode | ./src/engine/pipeline/navigator.rs:10 | 10 | unused public function `normalize_nav_url` |
| INFO | correctness | deadcode | ./src/engine/pipeline/navigator.rs:19 | 19 | unused public function `save_tabs` |
| INFO | correctness | deadcode | ./src/engine/pipeline/navigator.rs:32 | 32 | unused public function `load_tabs` |
| INFO | correctness | deadcode | ./src/engine/style.rs:4 | 4 | unused public function `compute_style` |
| INFO | correctness | deadcode | ./src/engine/style.rs:14 | 14 | unused public function `compute_style_vp` |
| INFO | correctness | deadcode | ./src/logging.rs:61 | 61 | unused public function `set_enabled` |
| INFO | correctness | deadcode | ./src/logging.rs:74 | 74 | unused public function `is_enabled` |
| INFO | correctness | deadcode | ./src/logging.rs:90 | 90 | unused public function `init` |
| INFO | correctness | deadcode | ./src/logging.rs:2 | 2 | unused import `Write` |
| INFO | correctness | deadcode | ./src/ui/kor_renderer.rs:7 | 7 | unused public function `render_kor_vm` |
| INFO | correctness | deadcode | ./src/ui/screens/browser.rs:37 | 37 | unused enum variant `BrowserMessage::OpenSettings` |
| INFO | correctness | deadcode | ./src/ui/style.rs:39 | 39 | unused public function `sidebar_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:51 | 51 | unused public function `main_area_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:58 | 58 | unused public function `card_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:70 | 70 | unused public function `status_bar_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:77 | 77 | unused public function `overlay_bg_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:84 | 84 | unused public function `palette_panel_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:96 | 96 | unused public function `settings_nav_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:110 | 110 | unused public function `ghost_button_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:130 | 130 | unused public function `nav_icon_button_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:150 | 150 | unused public function `sidebar_item_button_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:175 | 175 | unused public function `pill_button_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:202 | 202 | unused public function `card_button_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:219 | 219 | unused public function `url_input_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:234 | 234 | unused public function `autocomplete_dropdown_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:242 | 242 | unused public function `palette_input_style` |
| INFO | correctness | deadcode | ./src/ui/style.rs:25 | 25 | unused associated const `PAGE_BG` |
| INFO | correctness | deadcode | ./src/ui/style.rs:26 | 26 | unused associated const `PAGE_TEXT` |
| INFO | correctness | deadcode | ./src/ui/style.rs:27 | 27 | unused associated const `PAGE_MUTED` |
| ERROR | correctness | deadcode | ./temp_canvas2.rs:1 | 1 | private function `update` is never called |
| WARN | correctness | deadcode | ./temp_header.rs:45 | 45 | unused struct `BrowserScreen` |
| WARN | correctness | deadcode | ./temp_header.rs:22 | 22 | unused enum `BrowserMessage` |
| INFO | correctness | deadcode | ./temp_header.rs:5 | 5 | unused import `Handle` |
| INFO | correctness | deadcode | ./temp_header.rs:6 | 6 | unused import `mouse` |
| INFO | correctness | deadcode | ./temp_header.rs:10 | 10 | unused import `plog` |
| INFO | correctness | deadcode | ./temp_header.rs:15 | 15 | unused import `compile` |
| INFO | correctness | deadcode | ./temp_header.rs:16 | 16 | unused import `render_kor_vm` |
| INFO | correctness | deadcode | ./temp_header.rs:24 | 24 | unused enum variant `BrowserMessage::UrlChanged` |
| INFO | correctness | deadcode | ./temp_header.rs:25 | 25 | unused enum variant `BrowserMessage::UrlSubmit` |
| INFO | correctness | deadcode | ./temp_header.rs:26 | 26 | unused enum variant `BrowserMessage::NavBack` |
| INFO | correctness | deadcode | ./temp_header.rs:27 | 27 | unused enum variant `BrowserMessage::NavForward` |
| INFO | correctness | deadcode | ./temp_header.rs:28 | 28 | unused enum variant `BrowserMessage::Refresh` |
| INFO | correctness | deadcode | ./temp_header.rs:29 | 29 | unused enum variant `BrowserMessage::WorkspaceSelected` |
| INFO | correctness | deadcode | ./temp_header.rs:30 | 30 | unused enum variant `BrowserMessage::OpenSettings` |
| INFO | correctness | deadcode | ./temp_header.rs:31 | 31 | unused enum variant `BrowserMessage::OpenPalette` |
| INFO | correctness | deadcode | ./temp_header.rs:32 | 32 | unused enum variant `BrowserMessage::Bookmark` |
| INFO | correctness | deadcode | ./temp_header.rs:33 | 33 | unused enum variant `BrowserMessage::LinkClicked` |
| INFO | correctness | deadcode | ./temp_header.rs:34 | 34 | unused enum variant `BrowserMessage::PageLoaded` |
| INFO | correctness | deadcode | ./temp_header.rs:35 | 35 | unused enum variant `BrowserMessage::TimerTick` |
| INFO | correctness | deadcode | ./temp_header.rs:36 | 36 | unused enum variant `BrowserMessage::ElementClicked` |
| INFO | correctness | deadcode | ./temp_header.rs:37 | 37 | unused enum variant `BrowserMessage::TabSelected` |
| INFO | correctness | deadcode | ./temp_header.rs:38 | 38 | unused enum variant `BrowserMessage::NewTab` |
| INFO | correctness | deadcode | ./temp_header.rs:39 | 39 | unused enum variant `BrowserMessage::CloseTab` |
| INFO | correctness | deadcode | ./temp_header.rs:40 | 40 | unused enum variant `BrowserMessage::None` |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:25 | 25 | private function `test_color_hex` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:31 | 31 | private function `test_color_named` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:37 | 37 | private function `test_color_rgb` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:43 | 43 | private function `test_color_rgba` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:49 | 49 | private function `test_color_hsl` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:57 | 57 | private function `test_background_color` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:65 | 65 | private function `test_font_size` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:73 | 73 | private function `test_font_weight` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:81 | 81 | private function `test_display_none` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:87 | 87 | private function `test_display_flex` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:93 | 93 | private function `test_display_inline_block` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:101 | 101 | private function `test_margin_all` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:112 | 112 | private function `test_padding_two_values` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:123 | 123 | private function `test_border_width_and_color` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:138 | 138 | private function `test_width_height` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:148 | 148 | private function `test_flex_properties` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:159 | 159 | private function `test_flex_grow` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:165 | 165 | private function `test_flex_shrink` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:173 | 173 | private function `test_line_height` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:179 | 179 | private function `test_z_index` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:185 | 185 | private function `test_opacity` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:191 | 191 | private function `test_opacity_clamp_high` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:197 | 197 | private function `test_opacity_clamp_low` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:205 | 205 | private function `test_text_decoration` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:213 | 213 | private function `test_class_selector` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:225 | 225 | private function `test_id_selector` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:237 | 237 | private function `test_multiple_declarations` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:253 | 253 | private function `test_invalid_property_no_crash` is never called |
| ERROR | correctness | deadcode | ./tests/css_regression.rs:263 | 263 | private function `test_cascade_order` is never called |
| ERROR | correctness | deadcode | ./tests/integration_test.rs:4 | 4 | private function `test_parsing_div_with_paragraph` is never called |
| ERROR | correctness | deadcode | ./tests/integration_test.rs:24 | 24 | private function `test_parsing_multiple_elements` is never called |
| ERROR | correctness | deadcode | ./tests/integration_test.rs:40 | 40 | private function `test_should_skip_tag_filters` is never called |
| ERROR | correctness | deadcode | ./tests/integration_test.rs:50 | 50 | private function `test_extract_and_layout_pipeline` is never called |
| ERROR | correctness | deadcode | ./tests/integration_test.rs:104 | 104 | private function `test_inner_html_strips_script_tags` is never called |
| ERROR | correctness | deadcode | ./tests/integration_test.rs:132 | 132 | private function `test_set_attribute_rejects_event_handlers` is never called |
| ERROR | correctness | deadcode | ./tests/integration_test.rs:155 | 155 | private function `test_set_attribute_rejects_srcdoc` is never called |
| INFO | correctness | deadcode | ./tests/integration_test.rs:2 | 2 | unused import `NodeType` |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:35 | 35 | private function `test_create_element` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:42 | 42 | private function `test_append_child` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:53 | 53 | private function `test_text_node` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:61 | 61 | private function `test_set_attribute` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:69 | 69 | private function `test_get_element_by_id` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:80 | 80 | private function `test_query_selector` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:90 | 90 | private function `test_query_selector_all` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:106 | 106 | private function `test_set_text_content` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:114 | 114 | private function `test_inner_html` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:125 | 125 | private function `test_style_property` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:134 | 134 | private function `test_timeout` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:144 | 144 | private function `test_clear_timer` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:153 | 153 | private function `test_interval` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:166 | 166 | private function `test_event_listener` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:177 | 177 | private function `test_event_listener_bubbling` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:189 | 189 | private function `test_fetch_url_cross_origin` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:198 | 198 | private function `test_element_traversal` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:228 | 228 | private function `test_sibling_traversal` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:255 | 255 | private function `test_child_nodes_includes_text` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:267 | 267 | private function `test_dom_roundtrip` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:281 | 281 | private function `test_load_dom_with_document_root` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:289 | 289 | private function `test_dom_roundtrip_preserves_structure` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:305 | 305 | private function `test_load_dom_with_body` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:320 | 320 | private function `test_get_tag_name_variants` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:331 | 331 | private function `test_set_and_get_cookie` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:341 | 341 | private function `test_local_storage` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:363 | 363 | private function `test_location_parts` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:376 | 376 | private function `test_location_parts_defaults` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:388 | 388 | private function `test_location_reload` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:395 | 395 | private function `test_location_assign` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:402 | 402 | private function `test_location_replace` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:409 | 409 | private function `test_set_location_href` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:416 | 416 | private function `test_document_write_and_take_output` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:425 | 425 | private function `test_doc_title` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:433 | 433 | private function `test_pending_timers` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:443 | 443 | private function `test_query_selector_by_tag` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:456 | 456 | private function `test_query_selector_by_id` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:466 | 466 | private function `test_query_selector_by_class` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:478 | 478 | private function `test_query_selector_universal` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:487 | 487 | private function `test_query_selector_child_combinator` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:499 | 499 | private function `test_query_selector_descendant_combinator` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:515 | 515 | private function `test_query_selector_compound_descendant` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:527 | 527 | private function `test_event_listener_remove` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:537 | 537 | private function `test_get_event_listeners` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:550 | 550 | private function `test_self_append_child_noop` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:558 | 558 | private function `test_set_text_content_on_text_node_noop` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:566 | 566 | private function `test_remove_event_listener_partial_match` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:575 | 575 | private function `test_inner_html_self_closing` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:587 | 587 | private function `test_inner_html_nested` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:601 | 601 | private function `test_set_attribute_on_text_node_noop` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:609 | 609 | private function `test_element_at_point_no_elements` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:616 | 616 | private function `test_get_children_excludes_text` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:629 | 629 | private function `test_get_child_nodes_includes_all` is never called |
| ERROR | correctness | deadcode | ./tests/js_bridge_test.rs:641 | 641 | private function `test_load_dom_idempotent` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:21 | 21 | private function `test_query_selector_by_tag` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:29 | 29 | private function `test_query_selector_by_id` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:36 | 36 | private function `test_query_selector_by_class` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:43 | 43 | private function `test_query_selector_descendant` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:58 | 58 | private function `test_query_selector_child` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:80 | 80 | private function `test_query_selector_all` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:98 | 98 | private function `test_query_selector_no_match` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:106 | 106 | private function `test_query_selector_nested` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:130 | 130 | private function `test_query_selector_complex` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:148 | 148 | private function `test_query_selector_wildcard` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:167 | 167 | private function `test_set_timeout_adds_entry` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:174 | 174 | private function `test_set_interval_adds_entry` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:181 | 181 | private function `test_clear_timeout_removes_entry` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:190 | 190 | private function `test_clear_interval_removes_entry` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:199 | 199 | private function `test_timer_id_increments` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:213 | 213 | private function `test_timeout_callback_is_source` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:225 | 225 | private function `test_interval_callback_is_source` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:234 | 234 | private function `test_pending_timers_count` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:247 | 247 | private function `test_clear_all_timers` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:259 | 259 | private function `test_timer_after_page_load` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:275 | 275 | private function `test_add_event_listener` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:285 | 285 | private function `test_remove_event_listener` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:295 | 295 | private function `test_event_listener_id_increments` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:308 | 308 | private function `test_click_event_dispatch` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:319 | 319 | private function `test_event_bubbling` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:338 | 338 | private function `test_event_listener_source_string` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:348 | 348 | private function `test_multiple_listeners_same_event` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:362 | 362 | private function `test_get_event_listeners` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:373 | 373 | private function `test_remove_partial_match` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:383 | 383 | private function `test_event_after_remove` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:404 | 404 | private function `test_fetch_same_origin` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:414 | 414 | private function `test_fetch_cross_origin_cors` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:423 | 423 | private function `test_fetch_returns_status_prefix` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:432 | 432 | private function `test_local_storage_set_get` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:442 | 442 | private function `test_local_storage_remove` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:451 | 451 | private function `test_local_storage_clear` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:462 | 462 | private function `test_cookie_set_get` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:472 | 472 | private function `test_cookie_expires` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:481 | 481 | private function `test_fetch_error_handling` is never called |
| ERROR | correctness | deadcode | ./tests/js_engine_tests.rs:491 | 491 | private function `test_fetch_redirect` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:9 | 9 | private function `test_nested_function_calls` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:38 | 38 | private function `test_closure_capture` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:53 | 53 | private function `test_native_print` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:77 | 77 | private function `test_native_chrome_render` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:101 | 101 | private function `test_interpolate_multiple_vars` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:118 | 118 | private function `test_for_each_empty_array` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:134 | 134 | private function `test_for_each_single_item` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:150 | 150 | private function `test_jump_if_false_none` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:168 | 168 | private function `test_jump_if_false_zero` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:185 | 185 | private function `test_jump_if_false_empty_string` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:202 | 202 | private function `test_dup_preserves_value` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:218 | 218 | private function `test_pop_removes_value` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:234 | 234 | private function `test_store_load_roundtrip` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:250 | 250 | private function `test_create_element_sets_tag` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:268 | 268 | private function `test_set_property_adds_attr` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:291 | 291 | private function `test_add_child_nests_elements` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:316 | 316 | private function `test_deep_element_tree` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:357 | 357 | private function `test_multi_arg_native_call` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:377 | 377 | private function `test_for_each_with_jump_back` is never called |
| ERROR | correctness | deadcode | ./tests/korlang_advanced_tests.rs:396 | 396 | private function `test_empty_bytecode` is never called |
| INFO | correctness | deadcode | ./tests/korlang_advanced_tests.rs:3 | 3 | unused import `OpCode` |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:70 | 70 | private function `single_block_element` is never called |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:79 | 79 | private function `two_blocks_parent_child` is never called |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:91 | 91 | private function `thousand_flat_elements` is never called |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:101 | 101 | private function `thousand_inline_siblings` is never called |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:114 | 114 | private function `deep_nesting_50` is never called |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:124 | 124 | private function `deep_nesting_100` is never called |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:134 | 134 | private function `mixed_inline_block` is never called |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:150 | 150 | private function `large_text_elements` is never called |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:164 | 164 | private function `all_display_types` is never called |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:183 | 183 | private function `margins_affect_layout` is never called |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:204 | 204 | private function `padding_contains_children` is never called |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:227 | 227 | private function `borders_no_crash` is never called |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:249 | 249 | private function `empty_slice_no_panic` is never called |
| ERROR | correctness | deadcode | ./tests/layout_stress.rs:255 | 255 | private function `wide_container` is never called |
| ERROR | correctness | deadcode | ./tests/rendering.rs:5 | 5 | private function `test_basic_rendering_pipeline` is never called |
| ERROR | correctness | deadcode | ./tests/rendering.rs:28 | 28 | private function `test_caelum_spatial_init` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:56 | 56 | private function `test_css_parse_color_hex` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:62 | 62 | private function `test_css_parse_color_named` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:68 | 68 | private function `test_css_parse_margin_shorthand` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:77 | 77 | private function `test_css_parse_padding_shorthand` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:86 | 86 | private function `test_css_parse_display_block` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:92 | 92 | private function `test_css_parse_display_flex` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:98 | 98 | private function `test_css_parse_display_grid` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:104 | 104 | private function `test_css_parse_font_size_px` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:110 | 110 | private function `test_css_parse_border_width` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:118 | 118 | private function `test_css_parse_multiple_rules` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:129 | 129 | private function `test_computed_style_inline_to_block_for_block_tags` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:135 | 135 | private function `test_computed_style_class_overrides_tag` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:142 | 142 | private function `test_computed_style_id_overrides_class` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:149 | 149 | private function `test_computed_style_flex_direction` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:155 | 155 | private function `test_computed_style_justify_content` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:165 | 165 | private function `test_skip_tag_script` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:170 | 170 | private function `test_skip_tag_style` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:175 | 175 | private function `test_skip_tag_head` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:180 | 180 | private function `test_skip_tag_meta` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:185 | 185 | private function `test_skip_tag_noscript` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:190 | 190 | private function `test_skip_tag_svg` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:195 | 195 | private function `test_skip_tag_template` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:200 | 200 | private function `test_skip_content_script` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:205 | 205 | private function `test_skip_content_style` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:210 | 210 | private function `test_no_skip_div` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:215 | 215 | private function `test_no_skip_p` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:220 | 220 | private function `test_no_skip_img` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:225 | 225 | private function `test_no_skip_a` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:230 | 230 | private function `test_extract_elements_from_simple_html` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:241 | 241 | private function `test_extract_elements_no_script_content` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:252 | 252 | private function `test_extract_elements_script_content_skipped` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:264 | 264 | private function `test_extract_elements_head_content_hidden` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:280 | 280 | private function `test_block_elements_stack_vertically` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:291 | 291 | private function `test_block_elements_have_width` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:301 | 301 | private function `test_block_elements_have_height` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:311 | 311 | private function `test_nested_block_elements` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:322 | 322 | private function `test_multiple_siblings_stacked` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:335 | 335 | private function `test_block_with_margin_top` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:351 | 351 | private function `test_inline_siblings_flow_horizontally` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:362 | 362 | private function `test_inline_in_block` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:373 | 373 | private function `test_inline_wraps_when_long` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:383 | 383 | private function `test_inline_mixed_with_block` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:397 | 397 | private function `test_inline_block_element` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:410 | 410 | private function `test_multiple_inline_spans` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:427 | 427 | private function `test_flex_row_direction` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:443 | 443 | private function `test_flex_column_direction` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:459 | 459 | private function `test_flex_justify_center` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:474 | 474 | private function `test_flex_align_items_center` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:489 | 489 | private function `test_flex_wrap_nowrap` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:508 | 508 | private function `test_flex_grow` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:531 | 531 | private function `test_grid_display` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:537 | 537 | private function `test_grid_children_in_grid_container` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:554 | 554 | private function `test_grid_single_column` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:571 | 571 | private function `test_grid_item_sizes` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:586 | 586 | private function `test_grid_empty_container` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:603 | 603 | private function `test_float_left_element` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:618 | 618 | private function `test_float_right_element` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:634 | 634 | private function `test_float_does_not_affect_siblings_positioning` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:645 | 645 | private function `test_clear_both` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:655 | 655 | private function `test_multiple_floats` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:678 | 678 | private function `test_margin_top_on_first_element` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:690 | 690 | private function `test_margin_bottom_spacing` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:702 | 702 | private function `test_zero_margins` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:712 | 712 | private function `test_large_margin_top` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:724 | 724 | private function `test_margins_on_nested_elements` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:741 | 741 | private function `test_border_widths_applied` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:753 | 753 | private function `test_padding_affects_size` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:764 | 764 | private function `test_border_color_set` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:774 | 774 | private function `test_no_border_no_padding` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:784 | 784 | private function `test_element_positioning_with_padding` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:799 | 799 | private function `test_empty_elements_vec` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:806 | 806 | private function `test_single_element` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:813 | 813 | private function `test_display_none_elements_ignored` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:821 | 821 | private function `test_very_narrow_container` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:831 | 831 | private function `test_very_wide_container` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:841 | 841 | private function `test_long_text_wrapping` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:851 | 851 | private function `test_elements_with_image_dimensions` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:862 | 862 | private function `test_link_element_preserves_href` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:873 | 873 | private function `test_multiple_parent_levels` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:885 | 885 | private function `test_font_size_affects_height` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:900 | 900 | private function `test_decode_amp` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:905 | 905 | private function `test_decode_lt` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:910 | 910 | private function `test_decode_gt` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:915 | 915 | private function `test_decode_quot` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:920 | 920 | private function `test_decode_apos` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:925 | 925 | private function `test_decode_nbsp` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:930 | 930 | private function `test_decode_decimal` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:935 | 935 | private function `test_decode_hex_emoji` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:940 | 940 | private function `test_decode_no_nested_recursion` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:945 | 945 | private function `test_decode_no_entities_preserved` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:950 | 950 | private function `test_decode_preserves_unknown_entity` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:955 | 955 | private function `test_decode_mixed_text` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:960 | 960 | private function `test_decode_in_extracted_text` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:972 | 972 | private function `test_decode_href_attribute` is never called |
| ERROR | correctness | deadcode | ./tests/rendering_pipeline_tests.rs:984 | 984 | private function `test_decode_alt_attribute` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:71 | 71 | private function `test_tab_struct_construction` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:78 | 78 | private function `test_tab_empty_title` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:85 | 85 | private function `test_tab_clone` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:93 | 93 | private function `test_tab_serialization_roundtrip` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:110 | 110 | private function `test_normalize_nav_url_https` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:115 | 115 | private function `test_normalize_nav_url_http` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:120 | 120 | private function `test_normalize_nav_url_bare_domain` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:125 | 125 | private function `test_normalize_nav_url_double_slash` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:130 | 130 | private function `test_normalize_nav_url_aether_protocol` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:135 | 135 | private function `test_normalize_nav_url_about_blank` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:140 | 140 | private function `test_normalize_nav_url_empty` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:145 | 145 | private function `test_normalize_nav_url_whitespace_only` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:150 | 150 | private function `test_normalize_nav_url_strips_whitespace` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:155 | 155 | private function `test_normalize_url_plain` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:160 | 160 | private function `test_normalize_url_with_path` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:165 | 165 | private function `test_normalize_url_already_has_scheme` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:170 | 170 | private function `test_normalize_url_double_slash_strips_extra_slash` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:181 | 181 | private function `test_is_url_with_scheme` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:187 | 187 | private function `test_is_url_with_dot` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:193 | 193 | private function `test_is_url_aether_protocol` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:198 | 198 | private function `test_is_url_about_protocol` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:203 | 203 | private function `test_is_url_plain_search_query` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:209 | 209 | private function `test_search_url_duckduckgo` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:217 | 217 | private function `test_search_url_google` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:225 | 225 | private function `test_search_url_special_chars` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:237 | 237 | private function `test_settings_defaults` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:246 | 246 | private function `test_settings_serialization_roundtrip` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:262 | 262 | private function `test_settings_load_nonexistent_file_returns_defaults` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:270 | 270 | private function `test_settings_save_and_load` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:289 | 289 | private function `test_settings_toggle_js` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:299 | 299 | private function `test_settings_toggle_cookies` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:311 | 311 | private function `test_history_initial_state` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:319 | 319 | private function `test_history_push` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:329 | 329 | private function `test_history_back` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:341 | 341 | private function `test_history_forward` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:354 | 354 | private function `test_history_cannot_go_back_from_start` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:362 | 362 | private function `test_history_cannot_go_forward_from_end` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:372 | 372 | private function `test_history_truncate_on_new_navigate` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:388 | 388 | private function `test_history_limit_many_entries` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:404 | 404 | private function `test_autocomplete_filter_exact_prefix` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:414 | 414 | private function `test_autocomplete_filter_no_matches` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:423 | 423 | private function `test_autocomplete_filter_case_insensitive_contains` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:432 | 432 | private function `test_autocomplete_filter_limit_results` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:441 | 441 | private function `test_autocomplete_empty_input_shows_nothing` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:452 | 452 | private function `test_styled_element_construction` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:460 | 460 | private function `test_styled_element_default_values` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:471 | 471 | private function `test_styled_element_link` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:480 | 480 | private function `test_styled_element_with_background` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:489 | 489 | private function `test_styled_element_with_border` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:498 | 498 | private function `test_styled_element_with_margin` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:507 | 507 | private function `test_styled_element_with_image` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:515 | 515 | private function `test_styled_element_clone` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:527 | 527 | private function `test_layout_single_block_element` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:536 | 536 | private function `test_layout_two_block_elements_stacked` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:547 | 547 | private function `test_layout_inline_elements_side_by_side` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:560 | 560 | private function `test_layout_hidden_element` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:570 | 570 | private function `test_layout_with_margin` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:582 | 582 | private function `test_layout_with_border` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:594 | 594 | private function `test_skip_tag_script` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:599 | 599 | private function `test_skip_tag_style` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:604 | 604 | private function `test_skip_tag_noscript` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:609 | 609 | private function `test_skip_tag_meta` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:614 | 614 | private function `test_skip_tag_link` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:619 | 619 | private function `test_skip_tag_head` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:624 | 624 | private function `test_skip_tag_svg` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:629 | 629 | private function `test_skip_tag_template` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:634 | 634 | private function `test_no_skip_tag_div` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:639 | 639 | private function `test_no_skip_tag_p` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:644 | 644 | private function `test_no_skip_tag_img` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:649 | 649 | private function `test_no_skip_tag_a` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:654 | 654 | private function `test_no_skip_tag_span` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:659 | 659 | private function `test_skip_content_script` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:664 | 664 | private function `test_skip_content_style` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:669 | 669 | private function `test_no_skip_content_div` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:674 | 674 | private function `test_no_skip_content_p` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:683 | 683 | private function `test_save_tabs_empty` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:694 | 694 | private function `test_save_tabs_multiple` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:714 | 714 | private function `test_sidebar_workspace_labels` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:723 | 723 | private function `test_sidebar_collection_labels` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:730 | 730 | private function `test_sidebar_section_headers` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:741 | 741 | private function `test_devtools_tab_variants` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:752 | 752 | private function `test_devtools_tab_default_is_console` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:763 | 763 | private function `test_normalize_nav_url_with_port` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:768 | 768 | private function `test_normalize_nav_url_with_path_and_query` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:776 | 776 | private function `test_normalize_nav_url_with_fragment` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:784 | 784 | private function `test_settings_search_url_empty_query` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:791 | 791 | private function `test_settings_search_url_unicode_query` is never called |
| ERROR | correctness | deadcode | ./tests/sidebar_tests.rs:798 | 798 | private function `test_tab_title_update` is never called |
| WARN | correctness | apimisuse | ./build.rs:6 | 6 | `println!` in library code — use structured logging |
| INFO | correctness | apimisuse | ./build.rs:11 | 11 | .expect("Failed to read css-caelum-bridge.json") — message describes error, not expected invariant |
| INFO | correctness | apimisuse | ./build.rs:75 | 75 | .expect("Failed to write bridge_gen.rs") — message describes error, not expected invariant |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/block.rs:790 | 790 | `unreachable!()` in production code — abort risk on unexpected path |
| INFO | correctness | apimisuse | ./crates/aether-caelum/src/compute/flexbox.rs:959 | 959 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-caelum/src/compute/float.rs:397 | 397 | .len() compared with small integer — possibly meant .chars().count() |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/float.rs:410 | 410 | .unwrap() in production code — may panic on error |
| INFO | correctness | apimisuse | ./crates/aether-caelum/src/compute/float.rs:429 | 429 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:212 | 212 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/mod.rs:702 | 702 | .len() compared with small integer — possibly meant .chars().count() |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/placement.rs:122 | 122 | `println!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/placement.rs:151 | 151 | `println!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/placement.rs:218 | 218 | `println!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/placement.rs:474 | 474 | `println!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/placement.rs:476 | 476 | `println!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/placement.rs:497 | 497 | `println!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/placement.rs:499 | 499 | `println!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/placement.rs:501 | 501 | `println!` in library code — use structured logging |
| INFO | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:187 | 187 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:230 | 230 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:310 | 310 | .len() compared with small integer — possibly meant .chars().count() |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:512 | 512 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:623 | 623 | `unreachable!()` in production code — abort risk on unexpected path |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1389 | 1389 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:158 | 158 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:206 | 206 | .unwrap() in production code — may panic on error |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:37 | 37 | `panic!()` in production code — use proper error handling |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:91 | 91 | `panic!()` in production code — use proper error handling |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:93 | 93 | `panic!()` in production code — use proper error handling |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:206 | 206 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:232 | 232 | .unwrap() in production code — may panic on error |
| INFO | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:503 | 503 | .len() compared with small integer — possibly meant .chars().count() |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/types/named.rs:280 | 280 | `unreachable!()` in production code — abort risk on unexpected path |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/grid/types/named.rs:287 | 287 | `unreachable!()` in production code — abort risk on unexpected path |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/leaf.rs:137 | 137 | `unreachable!()` in production code — abort risk on unexpected path |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/mod.rs:297 | 297 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/mod.rs:298 | 298 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/mod.rs:299 | 299 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/mod.rs:301 | 301 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/mod.rs:302 | 302 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/mod.rs:304 | 304 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/compute/mod.rs:317 | 317 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:76 | 76 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:77 | 77 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:78 | 78 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:81 | 81 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:82 | 82 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:90 | 90 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:91 | 91 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:92 | 92 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:95 | 95 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:96 | 96 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:104 | 104 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:105 | 105 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:108 | 108 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:111 | 111 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:112 | 112 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:126 | 126 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:127 | 127 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:128 | 128 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:129 | 129 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:133 | 133 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:134 | 134 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:144 | 144 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:146 | 146 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:151 | 151 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:153 | 153 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:162 | 162 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:167 | 167 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:168 | 168 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:173 | 173 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:174 | 174 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:183 | 183 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:185 | 185 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:189 | 189 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:190 | 190 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:197 | 197 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:199 | 199 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:201 | 201 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:205 | 205 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:206 | 206 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:215 | 215 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:216 | 216 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:219 | 219 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:220 | 220 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:227 | 227 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:229 | 229 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:230 | 230 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:236 | 236 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:237 | 237 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:245 | 245 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:249 | 249 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:250 | 250 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:258 | 258 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:259 | 259 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:263 | 263 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:264 | 264 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:273 | 273 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:275 | 275 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:277 | 277 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:283 | 283 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:285 | 285 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:292 | 292 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:298 | 298 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:300 | 300 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:303 | 303 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:321 | 321 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:323 | 323 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:327 | 327 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:328 | 328 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:329 | 329 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:330 | 330 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:340 | 340 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:342 | 342 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:345 | 345 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:347 | 347 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:350 | 350 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:358 | 358 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:363 | 363 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:367 | 367 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:369 | 369 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:378 | 378 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:380 | 380 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:384 | 384 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:386 | 386 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:395 | 395 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:397 | 397 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:401 | 401 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:402 | 402 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:403 | 403 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:413 | 413 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:416 | 416 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:419 | 419 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:422 | 422 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:423 | 423 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:424 | 424 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:437 | 437 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:439 | 439 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:443 | 443 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:445 | 445 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:456 | 456 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:458 | 458 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:466 | 466 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:467 | 467 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:468 | 468 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:469 | 469 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:481 | 481 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:483 | 483 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:492 | 492 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:493 | 493 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:494 | 494 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:495 | 495 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:505 | 505 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:507 | 507 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:517 | 517 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:519 | 519 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:520 | 520 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:529 | 529 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:531 | 531 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:539 | 539 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:541 | 541 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:551 | 551 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:553 | 553 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:566 | 566 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:567 | 567 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:568 | 568 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:577 | 577 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:579 | 579 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:591 | 591 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:592 | 592 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:593 | 593 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:603 | 603 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:607 | 607 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:611 | 611 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:612 | 612 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:619 | 619 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:620 | 620 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:626 | 626 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:627 | 627 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:628 | 628 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:637 | 637 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:642 | 642 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:646 | 646 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:647 | 647 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:655 | 655 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:656 | 656 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:661 | 661 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:665 | 665 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:666 | 666 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:667 | 667 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:676 | 676 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:681 | 681 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:685 | 685 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:686 | 686 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:696 | 696 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:698 | 698 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:700 | 700 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:704 | 704 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:705 | 705 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:706 | 706 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:716 | 716 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:718 | 718 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:722 | 722 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:723 | 723 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:730 | 730 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:732 | 732 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:734 | 734 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:738 | 738 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:739 | 739 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:748 | 748 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:753 | 753 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:757 | 757 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:758 | 758 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:765 | 765 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:766 | 766 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:767 | 767 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:771 | 771 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:772 | 772 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:773 | 773 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:782 | 782 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:787 | 787 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:791 | 791 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:792 | 792 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:799 | 799 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:803 | 803 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:807 | 807 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/lib.rs:808 | 808 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/style/available_space.rs:57 | 57 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/style/compact_length.rs:456 | 456 | `unreachable!()` in production code — abort risk on unexpected path |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/style/dimension.rs:151 | 151 | `unreachable!()` in production code — abort risk on unexpected path |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/style/grid.rs:438 | 438 | `panic!()` in production code — use proper error handling |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/style/grid.rs:505 | 505 | `panic!()` in production code — use proper error handling |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/style/grid.rs:545 | 545 | `panic!()` in production code — use proper error handling |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/style_helpers.rs:26 | 26 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:959 | 959 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:971 | 971 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:981 | 981 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:982 | 982 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:983 | 983 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:995 | 995 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:997 | 997 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1005 | 1005 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1006 | 1006 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1007 | 1007 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1014 | 1014 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1025 | 1025 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1026 | 1026 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1027 | 1027 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1029 | 1029 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1030 | 1030 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1036 | 1036 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1037 | 1037 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1040 | 1040 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1041 | 1041 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1048 | 1048 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1049 | 1049 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1052 | 1052 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1053 | 1053 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1061 | 1061 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1064 | 1064 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1065 | 1065 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1068 | 1068 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1069 | 1069 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1077 | 1077 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1078 | 1078 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1079 | 1079 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1081 | 1081 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1084 | 1084 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1088 | 1088 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1093 | 1093 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1104 | 1104 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1105 | 1105 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1106 | 1106 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1112 | 1112 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1113 | 1113 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1114 | 1114 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1125 | 1125 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1126 | 1126 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1127 | 1127 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1131 | 1131 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1135 | 1135 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1142 | 1142 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1143 | 1143 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1144 | 1144 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1148 | 1148 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1152 | 1152 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1159 | 1159 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1160 | 1160 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1161 | 1161 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1162 | 1162 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1163 | 1163 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1167 | 1167 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1183 | 1183 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1184 | 1184 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1186 | 1186 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1188 | 1188 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1198 | 1198 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1199 | 1199 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1201 | 1201 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1205 | 1205 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1212 | 1212 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1213 | 1213 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1214 | 1214 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1215 | 1215 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1224 | 1224 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1225 | 1225 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1226 | 1226 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1237 | 1237 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1238 | 1238 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1239 | 1239 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1245 | 1245 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1254 | 1254 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1257 | 1257 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1266 | 1266 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1275 | 1275 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1285 | 1285 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1286 | 1286 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1287 | 1287 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1289 | 1289 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1295 | 1295 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1300 | 1300 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1301 | 1301 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1315 | 1315 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1329 | 1329 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1336 | 1336 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1352 | 1352 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1362 | 1362 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1370 | 1370 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1371 | 1371 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1373 | 1373 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/tree/caelum_tree.rs:1374 | 1374 | .unwrap() in production code — may panic on error |
| WARN | correctness | apimisuse | ./crates/aether-caelum/src/util/print.rs:11 | 11 | `println!` in library code — use structured logging |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/util/resolve.rs:36 | 36 | `unreachable!()` in production code — abort risk on unexpected path |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/util/resolve.rs:50 | 50 | `unreachable!()` in production code — abort risk on unexpected path |
| ERROR | correctness | apimisuse | ./crates/aether-caelum/src/util/resolve.rs:65 | 65 | `unreachable!()` in production code — abort risk on unexpected path |
| WARN | correctness | apimisuse | ./crates/aether-css/build.rs:6 | 6 | `println!` in library code — use structured logging |
| INFO | correctness | apimisuse | ./crates/aether-css/build.rs:11 | 11 | .expect("Failed to read css-properties.json") — message describes error, not expected invariant |
| INFO | correctness | apimisuse | ./crates/aether-css/build.rs:68 | 68 | .expect("Failed to write css_properties.rs") — message describes error, not expected invariant |
| WARN | correctness | apimisuse | ./crates/aether-css/src/parser.rs:89 | 89 | `eprintln!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./crates/aether-css/src/parser.rs:104 | 104 | `eprintln!` in library code — use structured logging |
| INFO | correctness | apimisuse | ./crates/aether-css/src/parser.rs:358 | 358 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-css/src/parser.rs:360 | 360 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-css/src/parser.rs:369 | 369 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-css/src/parser.rs:371 | 371 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-css/src/parser.rs:379 | 379 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-css/src/parser.rs:381 | 381 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-css/src/parser.rs:391 | 391 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-css/src/parser.rs:393 | 393 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-css/src/parser.rs:402 | 402 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-css/src/parser.rs:404 | 404 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-css/src/resolver.rs:226 | 226 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./crates/aether-css/src/resolver.rs:232 | 232 | .len() compared with small integer — possibly meant .chars().count() |
| ERROR | correctness | apimisuse | ./crates/aether-css/src/style_value.rs:551 | 551 | .unwrap() in production code — may panic on error |
| WARN | correctness | apimisuse | ./crates/aether-html/build.rs:6 | 6 | `println!` in library code — use structured logging |
| INFO | correctness | apimisuse | ./crates/aether-html/build.rs:11 | 11 | .expect("Failed to read tokenizer-states.json") — message describes error, not expected invariant |
| INFO | correctness | apimisuse | ./crates/aether-html/build.rs:30 | 30 | .expect("Failed to write state_dispatch.rs") — message describes error, not expected invariant |
| WARN | correctness | apimisuse | ./crates/aether-html/src/lib.rs:250 | 250 | `panic!()` in production code — use proper error handling |
| WARN | correctness | apimisuse | ./crates/aether-html/src/lib.rs:253 | 253 | `panic!()` in production code — use proper error handling |
| WARN | correctness | apimisuse | ./examples/korlang_demo.rs:16 | 16 | `println!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./examples/korlang_repl.rs:4 | 4 | `println!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./examples/korlang_repl.rs:5 | 5 | `println!` in library code — use structured logging |
| ERROR | correctness | apimisuse | ./examples/korlang_repl.rs:11 | 11 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./examples/korlang_repl.rs:14 | 14 | .unwrap() in production code — may panic on error |
| WARN | correctness | apimisuse | ./examples/korlang_repl.rs:25 | 25 | `println!` in library code — use structured logging |
| ERROR | correctness | apimisuse | ./examples/korlang_repl.rs:32 | 32 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./examples/korlang_repl.rs:34 | 34 | .unwrap() in production code — may panic on error |
| WARN | correctness | apimisuse | ./examples/korlang_repl.rs:36 | 36 | `println!` in library code — use structured logging |
| INFO | correctness | apimisuse | ./korlang/src/compiler/mod.rs:43 | 43 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./korlang/src/compiler/mod.rs:46 | 46 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./korlang/src/compiler/mod.rs:59 | 59 | .len() compared with small integer — possibly meant .chars().count() |
| WARN | correctness | apimisuse | ./korlang/src/compiler/parser.rs:311 | 311 | `eprintln!` in library code — use structured logging |
| ERROR | correctness | apimisuse | ./korlang/src/lib.rs:34 | 34 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./korlang/src/lib.rs:36 | 36 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./korlang/src/lib.rs:49 | 49 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./korlang/src/lib.rs:51 | 51 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./korlang/src/lib.rs:64 | 64 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./korlang/src/vm/mod.rs:234 | 234 | .unwrap() in production code — may panic on error |
| ERROR | correctness | apimisuse | ./korlang/src/vm/mod.rs:239 | 239 | .unwrap() in production code — may panic on error |
| INFO | correctness | apimisuse | ./src/engine/js/js_bridge.rs:256 | 256 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./src/engine/js/js_bridge.rs:262 | 262 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./src/engine/js/js_bridge.rs:420 | 420 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./src/engine/js/storage.rs:39 | 39 | .len() compared with small integer — possibly meant .chars().count() |
| ERROR | correctness | apimisuse | ./src/engine/korlang.rs:21 | 21 | expression statement discards Result — use `?` instead |
| ERROR | correctness | apimisuse | ./src/engine/korlang.rs:22 | 22 | expression statement discards Result — use `?` instead |
| WARN | correctness | apimisuse | ./src/engine/korlang.rs:59 | 59 | `println!` in library code — use structured logging |
| ERROR | correctness | apimisuse | ./src/engine/net/mod.rs:132 | 132 | .unwrap() in production code — may panic on error |
| INFO | correctness | apimisuse | ./src/engine/net/mod.rs:182 | 182 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./src/engine/net/mod.rs:327 | 327 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./src/engine/net/mod.rs:333 | 333 | .len() compared with small integer — possibly meant .chars().count() |
| WARN | correctness | apimisuse | ./src/engine/net/mod.rs:399 | 399 | `eprintln!` in library code — use structured logging |
| ERROR | correctness | apimisuse | ./src/engine/net/mod.rs:577 | 577 | expression statement discards Result — use `?` instead |
| ERROR | correctness | apimisuse | ./src/engine/net/mod.rs:643 | 643 | expression statement discards Result — use `?` instead |
| ERROR | correctness | apimisuse | ./src/engine/net/mod.rs:645 | 645 | expression statement discards Result — use `?` instead |
| ERROR | correctness | apimisuse | ./src/engine/net/mod.rs:749 | 749 | expression statement discards Result — use `?` instead |
| ERROR | correctness | apimisuse | ./src/engine/net/mod.rs:752 | 752 | expression statement discards Result — use `?` instead |
| ERROR | correctness | apimisuse | ./src/engine/pipeline/fetcher.rs:21 | 21 | .unwrap() in production code — may panic on error |
| INFO | correctness | apimisuse | ./src/engine/pipeline/fetcher.rs:343 | 343 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./src/engine/pipeline/layout.rs:158 | 158 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./src/engine/pipeline/layout.rs:232 | 232 | .len() compared with small integer — possibly meant .chars().count() |
| WARN | correctness | apimisuse | ./src/logging.rs:16 | 16 | `eprintln!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./src/logging.rs:34 | 34 | `eprintln!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./src/logging.rs:37 | 37 | `eprintln!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./src/logging.rs:40 | 40 | `eprintln!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./src/logging.rs:52 | 52 | `eprintln!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./src/logging.rs:55 | 55 | `eprintln!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./src/logging.rs:67 | 67 | `eprintln!` in library code — use structured logging |
| WARN | correctness | apimisuse | ./src/ui/kor_renderer.rs:73 | 73 | `eprintln!` in library code — use structured logging |
| INFO | correctness | apimisuse | ./src/ui/kor_renderer.rs:143 | 143 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./src/ui/kor_renderer.rs:143 | 143 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./src/ui/kor_renderer.rs:144 | 144 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./src/ui/screens/browser.rs:462 | 462 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./src/ui/screens/browser.rs:597 | 597 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./src/ui/screens/browser.rs:608 | 608 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./src/ui/screens/browser.rs:611 | 611 | .len() compared with small integer — possibly meant .chars().count() |
| ERROR | correctness | apimisuse | ./src/ui/screens/browser.rs:791 | 791 | .unwrap() in production code — may panic on error |
| INFO | correctness | apimisuse | ./src/ui/screens/browser.rs:854 | 854 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./tests/sidebar_tests.rs:324 | 324 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | correctness | apimisuse | ./tests/sidebar_tests.rs:383 | 383 | .len() compared with small integer — possibly meant .chars().count() |
| INFO | memory | constcheck | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:935 | 935 | #[inline(always)] on large function `distribute_item_space_to_base_size` (122 lines) — may cause code bloat |
| INFO | memory | constcheck | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1165 | 1165 | #[inline(always)] on large function `expand_flexible_tracks` (87 lines) — may cause code bloat |
| INFO | memory | constcheck | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1267 | 1267 | #[inline(always)] on large function `find_size_of_fr` (55 lines) — may cause code bloat |
| ERROR | correctness | constcheck | ./crates/aether-caelum/src/style_helpers.rs:532 | 532 | const `TEST_VEC` has type `Vec` which implements Drop — drop may not run when inlined |
| WARN | security | security_ext | ./build.rs:11 | 11 | unbounded read — `fs::read_to_string` without size limit on untrusted input |
| INFO | security | security_ext | ./crates/aether-caelum/src/style/compact_length.rs:19 | 19 | public fn `tag_ptr` returns raw pointer — callers can dereference unsafely, consider returning a reference |
| INFO | security | security_ext | ./crates/aether-caelum/src/style/compact_length.rs:308 | 308 | public method `calc_value` returns raw pointer — callers can dereference unsafely |
| WARN | security | security_ext | ./crates/aether-css/build.rs:11 | 11 | unbounded read — `fs::read_to_string` without size limit on untrusted input |
| INFO | security | security_ext | ./crates/aether-css/src/lib.rs:1 | 1 | crate-level `#![allow(dead_code)]` — suppresses dead_code. Only allow with justification |
| WARN | security | security_ext | ./crates/aether-css/src/parser.rs:304 | 304 | unvalidated deserialization — `LengthValue::from_str` called on non-literal (external) data |
| WARN | security | security_ext | ./crates/aether-css/src/resolver.rs:231 | 231 | unvalidated deserialization — `LengthValue::from_str` called on non-literal (external) data |
| WARN | security | security_ext | ./crates/aether-css/src/style_value.rs:281 | 281 | unvalidated deserialization — `Unit::from_str` called on non-literal (external) data |
| WARN | security | security_ext | ./crates/aether-html/build.rs:11 | 11 | unbounded read — `fs::read_to_string` without size limit on untrusted input |
| WARN | security | security_ext | ./src/engine/net/mod.rs:113 | 113 | unbounded read — `std::fs::read_to_string` without size limit on untrusted input |
| WARN | security | security_ext | ./src/engine/net/mod.rs:383 | 383 | unvalidated deserialization — `CspDirective::from_str` called on non-literal (external) data |
| WARN | security | security_ext | ./src/engine/pipeline/navigator.rs:33 | 33 | unbounded read — `std::fs::read_to_string` without size limit on untrusted input |
| WARN | security | security_ext | ./src/ui/screens/settings.rs:29 | 29 | unbounded read — `std::fs::read_to_string` without size limit on untrusted input |
| INFO | security | security_ext | ./src/ui/style.rs:1 | 1 | crate-level `#![allow(dead_code)]` — suppresses dead_code. Only allow with justification |
| WARN | security | security_ext | ./tests/sidebar_tests.rs:281 | 281 | unbounded read — `std::fs::read_to_string` without size limit on untrusted input |
| ERROR | security | security_ext | ./tests/sidebar_tests.rs:286 | 286 | unrestricted file deletion — `std::fs::remove_file` with non-literal path (possible TOCTOU / path traversal) |
| WARN | security | security_ext | ./tests/sidebar_tests.rs:689 | 689 | unbounded read — `std::fs::read_to_string` without size limit on untrusted input |
| ERROR | security | security_ext | ./tests/sidebar_tests.rs:691 | 691 | unrestricted file deletion — `std::fs::remove_file` with non-literal path (possible TOCTOU / path traversal) |
| WARN | security | security_ext | ./tests/sidebar_tests.rs:703 | 703 | unbounded read — `std::fs::read_to_string` without size limit on untrusted input |
| ERROR | security | security_ext | ./tests/sidebar_tests.rs:707 | 707 | unrestricted file deletion — `std::fs::remove_file` with non-literal path (possible TOCTOU / path traversal) |
| WARN | style | unused_qual | ./crates/aether-caelum/src/style/mod.rs:1222 | 1222 | unnecessary qualification: `crate::util::sys::DefaultCheapStr` → `DefaultCheapStr` |
| INFO | style | unused_qual | ./examples/korlang_repl.rs:1 | 1 | unnecessary `self` import — use `std::io` directly |
| WARN | style | unused_qual | ./src/engine/js/js_bridge.rs:197 | 197 | unnecessary qualification: `std::time::Instant` → `Instant` |
| WARN | style | unused_qual | ./src/engine/net/mod.rs:616 | 616 | unnecessary qualification: `std::time::Instant::now` → `Instant::now` |
| WARN | style | unused_qual | ./src/engine/pipeline/extractor.rs:222 | 222 | unnecessary qualification: `crate::engine::stratus::ElementData::with_attributes` → `stratus::ElementData::with_attributes` |
| WARN | style | unused_qual | ./src/engine/pipeline/extractor.rs:223 | 223 | unnecessary qualification: `crate::engine::stratus::resolve_style_vp` → `stratus::resolve_style_vp` |
| WARN | style | unused_qual | ./src/engine/pipeline/fetcher.rs:93 | 93 | unnecessary qualification: `crate::engine::js::JsBridge` → `JsBridge` |
| WARN | style | unused_qual | ./src/ui/kor_renderer.rs:136 | 136 | unnecessary qualification: `korlang::vm::Value` → `Value` |
| WARN | style | unused_qual | ./src/ui/kor_renderer.rs:139 | 139 | unnecessary qualification: `korlang::vm::Value` → `Value` |
| INFO | style | unused_qual | ./src/ui/kor_renderer.rs:1 | 1 | import `korlang::vm::Value` is only used via fully-qualified path — consider removing it |
| WARN | style | unused_qual | ./src/ui/mod.rs:16 | 16 | unnecessary qualification: `iced::Event` → `Event` |
| INFO | style | unused_qual | ./src/ui/mod.rs:6 | 6 | import `iced::Event` is only used via fully-qualified path — consider removing it |
| WARN | style | unused_qual | ./src/ui/screens/browser.rs:204 | 204 | unnecessary qualification: `iced::widget::canvas::Event` → `canvas::Event` |
| WARN | style | unused_qual | ./src/ui/screens/browser.rs:207 | 207 | unnecessary qualification: `iced::widget::canvas::event::Status` → `canvas::event::Status` |
| WARN | style | unused_qual | ./src/ui/screens/browser.rs:693 | 693 | unnecessary qualification: `iced::Color::WHITE` → `Color::WHITE` |
| WARN | style | unused_qual | ./src/ui/screens/browser.rs:696 | 696 | unnecessary qualification: `iced::Color::from_rgba` → `Color::from_rgba` |
| WARN | style | unused_qual | ./src/ui/screens/browser.rs:708 | 708 | unnecessary qualification: `iced::Color::from_rgba` → `Color::from_rgba` |
| WARN | style | unused_qual | ./src/ui/screens/browser.rs:722 | 722 | unnecessary qualification: `iced::Color::from_rgba` → `Color::from_rgba` |
| WARN | style | unused_qual | ./src/ui/screens/browser.rs:725 | 725 | unnecessary qualification: `iced::Color::from_rgba` → `Color::from_rgba` |
| WARN | style | unused_qual | ./src/ui/screens/browser.rs:741 | 741 | unnecessary qualification: `iced::Color::from_rgba` → `Color::from_rgba` |
| WARN | style | unused_qual | ./src/ui/screens/browser.rs:749 | 749 | unnecessary qualification: `iced::widget::button::Status::Hovered` → `button::Status::Hovered` |
| WARN | style | unused_qual | ./src/ui/screens/browser.rs:1122 | 1122 | unnecessary qualification: `std::collections::HashMap::new` → `HashMap::new` |
| WARN | style | unused_qual | ./src/ui/screens/palette.rs:160 | 160 | unnecessary qualification: `iced::widget::button::Status::Hovered` → `button::Status::Hovered` |
| WARN | style | unused_qual | ./src/ui/screens/palette.rs:205 | 205 | unnecessary qualification: `iced::widget::button::Status::Hovered` → `button::Status::Hovered` |
| WARN | style | unused_qual | ./src/ui/screens/settings.rs:393 | 393 | unnecessary qualification: `iced::widget::button::Status::Hovered` → `button::Status::Hovered` |
| WARN | style | unused_qual | ./src/ui/screens/settings.rs:453 | 453 | unnecessary qualification: `iced::widget::button::Status::Hovered` → `button::Status::Hovered` |
| WARN | style | unused_qual | ./src/ui/screens/settings.rs:497 | 497 | unnecessary qualification: `iced::widget::button::Style::default` → `button::Style::default` |
| WARN | style | unused_qual | ./src/ui/style.rs:205 | 205 | unnecessary qualification: `iced::Color::from_rgba` → `Color::from_rgba` |
| WARN | style | unused_qual | ./src/ui/style.rs:206 | 206 | unnecessary qualification: `iced::Color::from_rgba` → `Color::from_rgba` |
| INFO | style | unused_qual | ./tests/css_regression.rs:3 | 3 | unnecessary `self` import — use `aether_browser::engine::stratus` directly |
| INFO | style | unused_qual | ./tests/rendering_pipeline_tests.rs:2 | 2 | unnecessary `self` import — use `aether_browser::engine::stratus` directly |
| WARN | style | pattern_match | ./crates/aether-caelum/src/compute/flexbox.rs:2435 | 2435 | match on bool — use `if/else` instead |
| INFO | style | pattern_match | ./crates/aether-caelum/src/compute/grid/mod.rs:636 | 636 | wildcard `_` in destructuring — consider naming with `_` prefix |
| WARN | style | pattern_match | ./crates/aether-caelum/src/compute/grid/mod.rs:773 | 773 | match on bool — use `if/else` instead |
| WARN | style | pattern_match | ./crates/aether-caelum/src/compute/grid/placement.rs:301 | 301 | match on bool — use `if/else` instead |
| WARN | style | pattern_match | ./crates/aether-caelum/src/compute/grid/placement.rs:389 | 389 | match on bool — use `if/else` instead |
| WARN | style | pattern_match | ./crates/aether-caelum/src/style/mod.rs:318 | 318 | match on bool — use `if/else` instead |
| WARN | style | pattern_match | ./crates/aether-css/src/resolver.rs:382 | 382 | catch-all `_ => {}` arm — unmatched cases are silently ignored |
| WARN | style | pattern_match | ./crates/aether-css/src/resolver.rs:426 | 426 | catch-all `_ => {}` arm — unmatched cases are silently ignored |
| WARN | style | pattern_match | ./korlang/src/compiler/mod.rs:115 | 115 | catch-all `_ => {}` arm — unmatched cases are silently ignored |
| WARN | style | pattern_match | ./korlang/src/compiler/mod.rs:126 | 126 | catch-all `_ => {}` arm — unmatched cases are silently ignored |
| WARN | style | pattern_match | ./src/engine/net/mod.rs:308 | 308 | catch-all `_ => {}` arm — unmatched cases are silently ignored |
| WARN | style | pattern_match | ./src/engine/pipeline/extractor.rs:190 | 190 | catch-all `_ => {}` arm — unmatched cases are silently ignored |
| WARN | style | pattern_match | ./src/ui/screens/settings.rs:121 | 121 | catch-all `_ => {}` arm — unmatched cases are silently ignored |
| INFO | style | pattern_match | ./tests/js_engine_tests.rs:23 | 23 | wildcard `_` in destructuring — consider naming with `_` prefix |
| INFO | style | pattern_match | ./tests/js_engine_tests.rs:31 | 31 | wildcard `_` in destructuring — consider naming with `_` prefix |
| INFO | style | pattern_match | ./tests/js_engine_tests.rs:38 | 38 | wildcard `_` in destructuring — consider naming with `_` prefix |
| INFO | style | pattern_match | ./tests/js_engine_tests.rs:100 | 100 | wildcard `_` in destructuring — consider naming with `_` prefix |
| INFO | perf | iter_issues | ./korlang/src/compiler/formatter.rs:92 | 92 | `vars[i]` inside `for i in 0..vars.len()` — use `for item in &vars` instead |
| INFO | perf | iter_issues | ./korlang/src/compiler/mod.rs:91 | 91 | `vars[i]` inside `for i in 0..vars.len()` — use `for item in &vars` instead |
| INFO | perf | iter_issues | ./src/engine/pipeline/layout.rs:186 | 186 | `elements[i]` inside `for i in 0..elements.len()` — use `for item in &elements` instead |
| WARN | style | lifetime | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:19 | 19 | unused lifetime parameter `a` |
| WARN | style | lifetime | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:65 | 65 | unused lifetime parameter `a` |
| WARN | style | lifetime | ./crates/aether-caelum/src/compute/grid/placement.rs:82 | 82 | unused lifetime parameter `a` |
| INFO | style | lifetime | ./crates/aether-caelum/src/compute/grid/placement.rs:154 | 154 | explicit reborrow `&*x` — use `x` directly when `x` is already a reference |
| INFO | style | lifetime | ./crates/aether-caelum/src/compute/grid/placement.rs:222 | 222 | explicit reborrow `&*x` — use `x` directly when `x` is already a reference |
| INFO | style | lifetime | ./crates/aether-caelum/src/compute/grid/types/named.rs:345 | 345 | explicit reborrow `&*x` — use `x` directly when `x` is already a reference |
| INFO | style | lifetime | ./crates/aether-caelum/src/compute/grid/types/named.rs:352 | 352 | explicit reborrow `&*x` — use `x` directly when `x` is already a reference |
| INFO | style | lifetime | ./src/engine/net/mod.rs:120 | 120 | explicit reborrow `&*x` — use `x` directly when `x` is already a reference |
| INFO | perf | string_issues | ./crates/aether-css/src/lib.rs:33 | 33 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/lib.rs:33 | 33 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/lib.rs:34 | 34 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/lib.rs:46 | 46 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/lib.rs:34 | 34 | "div".to_string() used 2 times — extract as const |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:337 | 337 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:339 | 339 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:448 | 448 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:458 | 458 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:468 | 468 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:480 | 480 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:491 | 491 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:491 | 491 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:492 | 492 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:502 | 502 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:511 | 511 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:520 | 520 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:529 | 529 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:538 | 538 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:547 | 547 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:556 | 556 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:448 | 448 | "div".to_string() used 12 times — extract as const |
| INFO | perf | string_issues | ./crates/aether-css/src/resolver.rs:491 | 491 | "id".to_string() used 2 times — extract as const |
| INFO | perf | string_issues | ./crates/aether-css/src/style_value.rs:485 | 485 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/style_value.rs:486 | 486 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/style_value.rs:491 | 491 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-css/src/style_value.rs:523 | 523 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-dom/src/lib.rs:111 | 111 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-dom/src/lib.rs:117 | 117 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-html/src/lib.rs:133 | 133 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./crates/aether-html/src/lib.rs:172 | 172 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./crates/aether-html/src/lib.rs:242 | 242 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./examples/korlang_repl.rs:21 | 21 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./korlang/src/compiler/formatter.rs:45 | 45 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./korlang/src/compiler/formatter.rs:54 | 54 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./korlang/src/compiler/formatter.rs:58 | 58 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./korlang/src/compiler/formatter.rs:67 | 67 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./korlang/src/compiler/formatter.rs:75 | 75 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./korlang/src/compiler/formatter.rs:78 | 78 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./korlang/src/compiler/formatter.rs:88 | 88 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./korlang/src/compiler/mod.rs:69 | 69 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./korlang/src/compiler/parser.rs:93 | 93 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./korlang/src/compiler/parser.rs:94 | 94 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./korlang/src/compiler/parser.rs:95 | 95 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./korlang/src/compiler/parser.rs:96 | 96 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./korlang/src/compiler/parser.rs:97 | 97 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./korlang/src/vm/mod.rs:46 | 46 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./korlang/src/vm/mod.rs:47 | 47 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./korlang/src/vm/mod.rs:48 | 48 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./korlang/src/vm/mod.rs:277 | 277 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/js/fetch.rs:13 | 13 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/js/fetch.rs:18 | 18 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/js/js_bridge.rs:333 | 333 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/js/js_bridge.rs:360 | 360 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/js/js_bridge.rs:673 | 673 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/js/js_bridge.rs:674 | 674 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/js/js_bridge.rs:745 | 745 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/js/js_bridge.rs:775 | 775 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/js/js_bridge.rs:2116 | 2116 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/js/mod.rs:44 | 44 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/js/mod.rs:48 | 48 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/js/mod.rs:60 | 60 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/js/mod.rs:64 | 64 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/js/mod.rs:74 | 74 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/js/mod.rs:77 | 77 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/js/mod.rs:48 | 48 | "No JS context".to_string() used 3 times — extract as const |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:60 | 60 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:144 | 144 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:148 | 148 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:618 | 618 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:636 | 636 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:673 | 673 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:688 | 688 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:728 | 728 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:736 | 736 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:745 | 745 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:756 | 756 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:764 | 764 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:775 | 775 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/net/mod.rs:618 | 618 | "HTTP client not available".to_string() used 2 times — extract as const |
| INFO | perf | string_issues | ./src/engine/pipeline/extractor.rs:228 | 228 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/pipeline/extractor.rs:250 | 250 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/pipeline/extractor.rs:437 | 437 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/pipeline/extractor.rs:454 | 454 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/pipeline/extractor.rs:472 | 472 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/pipeline/extractor.rs:492 | 492 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/pipeline/extractor.rs:554 | 554 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/pipeline/extractor.rs:748 | 748 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/pipeline/extractor.rs:763 | 763 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/pipeline/extractor.rs:454 | 454 | "text".to_string() used 2 times — extract as const |
| INFO | perf | string_issues | ./src/engine/pipeline/fetcher.rs:153 | 153 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/engine/pipeline/navigator.rs:12 | 12 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/style.rs:9 | 9 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/style.rs:19 | 19 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/engine/style.rs:9 | 9 | "unknown".to_string() used 2 times — extract as const |
| INFO | perf | string_issues | ./src/logging.rs:30 | 30 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/ui/kor_renderer.rs:58 | 58 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/kor_renderer.rs:81 | 81 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/kor_renderer.rs:91 | 91 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:252 | 252 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:255 | 255 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:256 | 256 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:257 | 257 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:308 | 308 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:313 | 313 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:493 | 493 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:494 | 494 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:495 | 495 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:509 | 509 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:546 | 546 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:578 | 578 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:596 | 596 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:598 | 598 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:599 | 599 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:693 | 693 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:722 | 722 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:1006 | 1006 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:1007 | 1007 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:1022 | 1022 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:1027 | 1027 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:1027 | 1027 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:1028 | 1028 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:1028 | 1028 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:1037 | 1037 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:1123 | 1123 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:1123 | 1123 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:1124 | 1124 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/browser.rs:596 | 596 | "about:blank".to_string() used 2 times — extract as const |
| INFO | perf | string_issues | ./src/ui/screens/settings.rs:19 | 19 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/settings.rs:20 | 20 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/settings.rs:46 | 46 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/ui/screens/settings.rs:47 | 47 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./src/ui/screens/settings.rs:59 | 59 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/settings.rs:300 | 300 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/settings.rs:302 | 302 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./src/ui/screens/settings.rs:20 | 20 | "duckduckgo".to_string() used 2 times — extract as const |
| INFO | style | string_issues | ./tests/integration_test.rs:6 | 6 | String::from("...") — use "...".to_string() instead |
| INFO | style | string_issues | ./tests/integration_test.rs:26 | 26 | String::from("...") — use "...".to_string() instead |
| INFO | perf | string_issues | ./tests/integration_test.rs:57 | 57 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/integration_test.rs:64 | 64 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/integration_test.rs:109 | 109 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/integration_test.rs:137 | 137 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/integration_test.rs:160 | 160 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/integration_test.rs:109 | 109 | "div".to_string() used 3 times — extract as const |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:7 | 7 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:7 | 7 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:9 | 9 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:9 | 9 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:10 | 10 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:18 | 18 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:137 | 137 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:147 | 147 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:158 | 158 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:170 | 170 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:170 | 170 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:183 | 183 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:183 | 183 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:184 | 184 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:184 | 184 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:308 | 308 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:308 | 308 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:309 | 309 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:345 | 345 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:345 | 345 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:346 | 346 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:346 | 346 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:398 | 398 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:405 | 405 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:412 | 412 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:429 | 429 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:437 | 437 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:531 | 531 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:531 | 531 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:533 | 533 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:533 | 533 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:541 | 541 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:541 | 541 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:542 | 542 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:542 | 542 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:543 | 543 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:543 | 543 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:570 | 570 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:570 | 570 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:571 | 571 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:571 | 571 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:137 | 137 | "console.log('hi')".to_string() used 2 times — extract as const |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:531 | 531 | "fn()".to_string() used 2 times — extract as const |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:10 | 10 | "div".to_string() used 2 times — extract as const |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:170 | 170 | "click".to_string() used 9 times — extract as const |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:170 | 170 | "handler()".to_string() used 2 times — extract as const |
| INFO | perf | string_issues | ./tests/js_bridge_test.rs:7 | 7 | "id".to_string() used 2 times — extract as const |
| INFO | perf | string_issues | ./tests/js_engine_tests.rs:216 | 216 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_engine_tests.rs:228 | 228 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/js_engine_tests.rs:342 | 342 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/layout_stress.rs:15 | 15 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/layout_stress.rs:26 | 26 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/layout_stress.rs:27 | 27 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/layout_stress.rs:28 | 28 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/layout_stress.rs:29 | 29 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/layout_stress.rs:30 | 30 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/layout_stress.rs:53 | 53 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/layout_stress.rs:106 | 106 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/layout_stress.rs:107 | 107 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./tests/layout_stress.rs:142 | 142 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./tests/layout_stress.rs:171 | 171 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./tests/rendering.rs:7 | 7 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/rendering.rs:23 | 23 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/rendering_pipeline_tests.rs:32 | 32 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/rendering_pipeline_tests.rs:37 | 37 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/rendering_pipeline_tests.rs:37 | 37 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/rendering_pipeline_tests.rs:38 | 38 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/rendering_pipeline_tests.rs:38 | 38 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/rendering_pipeline_tests.rs:47 | 47 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/rendering_pipeline_tests.rs:866 | 866 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:21 | 21 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:33 | 33 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:34 | 34 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:35 | 35 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:36 | 36 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:59 | 59 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:73 | 73 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:73 | 73 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:80 | 80 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:87 | 87 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:87 | 87 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:211 | 211 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:219 | 219 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:249 | 249 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:250 | 250 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:274 | 274 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:275 | 275 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:323 | 323 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:382 | 382 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:390 | 390 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:406 | 406 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:407 | 407 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:408 | 408 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:416 | 416 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:417 | 417 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:425 | 425 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:434 | 434 | format!("{}", x) with single placeholder — use x.to_string() or x.into() instead |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:443 | 443 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:475 | 475 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:510 | 510 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:563 | 563 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:800 | 800 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:800 | 800 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:801 | 801 | "literal".to_string() — use `"literal"` directly or `"literal".into()` |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:73 | 73 | "https://example.com".to_string() used 6 times — extract as const |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:219 | 219 | "google".to_string() used 3 times — extract as const |
| INFO | perf | string_issues | ./tests/sidebar_tests.rs:408 | 408 | "https://other.net".to_string() used 2 times — extract as const |
| WARN | style | attr_checks | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:184 | 184 | #[allow(...)] on module `tests` — too broad, apply to individual items |
| WARN | perf | attr_checks | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:935 | 935 | #[inline(always)] on large fn `distribute_item_space_to_base_size` (122 lines) — may cause code bloat |
| WARN | perf | attr_checks | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1165 | 1165 | #[inline(always)] on large fn `expand_flexible_tracks` (87 lines) — may cause code bloat |
| WARN | perf | attr_checks | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1267 | 1267 | #[inline(always)] on large fn `find_size_of_fr` (55 lines) — may cause code bloat |
| WARN | style | attr_checks | ./crates/aether-caelum/src/compute/grid/types/grid_track_counts.rs:72 | 72 | #[allow(...)] on impl block — too broad, apply to individual items |
| WARN | style | attr_checks | ./crates/aether-caelum/src/geometry.rs:355 | 355 | #[allow(...)] on impl block — too broad, apply to individual items |
| WARN | style | attr_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:511 | 511 | #[allow(...)] on impl block — too broad, apply to individual items |
| INFO | style | attr_checks | ./src/logging.rs:90 | 90 | #[allow(dead_code)] on pub fn `init` — unnecessary |
| WARN | style | shadowing | ./build.rs:16 | 16 | `output` is declared `mut` but never mutated |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/compute/block.rs:74 | 74 | local `insets` shadows function parameter of same name |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:128 | 128 | `pos` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:145 | 145 | `slot` is declared `mut` but never mutated |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:278 | 278 | match binding `min` shadows outer variable |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:278 | 278 | match binding `max` shadows outer variable |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:303 | 303 | `root_bfc` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:304 | 304 | `root_ctx` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:406 | 406 | `items` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:601 | 601 | `float_contribution` is declared `mut` but never mutated |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:942 | 942 | `free_x_space` is shadowed in a nested block — reduces readability |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:713 | 713 | `location` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:830 | 830 | `child_block_ctx` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:911 | 911 | `location` is declared `mut` but never mutated |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:1202 | 1202 | match binding `left` shadows outer variable |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:1202 | 1202 | match binding `right` shadows outer variable |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:1209 | 1209 | match binding `left` shadows outer variable |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/block.rs:1210 | 1210 | match binding `right` shadows outer variable |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:203 | 203 | match binding `min` shadows outer variable |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:203 | 203 | match binding `max` shadows outer variable |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:230 | 230 | `constants` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:238 | 238 | `flex_items` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:265 | 265 | `flex_lines` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:448 | 448 | `content_box_inset` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:682 | 682 | `ckd` is declared `mut` but never mutated |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:895 | 895 | match binding `main_axis_available_space` shadows outer variable |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:897 | 897 | local `flex_items` shadows function parameter of same name |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:860 | 860 | `lines` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:879 | 879 | `lines` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:886 | 886 | `lines` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:896 | 896 | `lines` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:903 | 903 | `line_length` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:1047 | 1047 | `ckd` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:1663 | 1663 | `num_auto_margins` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:2021 | 2021 | `total_offset_main` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:2077 | 2077 | `total_offset_cross` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/flexbox.rs:2083 | 2083 | `content_size` is declared `mut` but never mutated |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/float.rs:390 | 390 | `start_y` is shadowed in a nested block — reduces readability |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/float.rs:397 | 397 | `start_idx` is shadowed in a nested block — reduces readability |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/float.rs:398 | 398 | `end_idx` is shadowed in a nested block — reduces readability |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/float.rs:431 | 431 | match binding `end_idx` shadows outer variable |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/float.rs:298 | 298 | `start_idx` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/float.rs:325 | 325 | `fitter` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/float.rs:392 | 392 | `insets` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/float.rs:410 | 410 | `start_idx` is declared `mut` but never mutated |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/float.rs:501 | 501 | match binding `segment` shadows outer variable |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/alignment.rs:345 | 345 | `start` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:243 | 243 | `current_track_index` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:277 | 277 | `track` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:279 | 279 | `gutter` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:694 | 694 | `tracks` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/mod.rs:88 | 88 | `content_box_inset` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/mod.rs:126 | 126 | `inner_node_size` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/mod.rs:184 | 184 | `name_resolver` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/mod.rs:200 | 200 | `items` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/mod.rs:201 | 201 | `cell_occupancy_matrix` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/mod.rs:228 | 228 | `columns` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/mod.rs:229 | 229 | `rows` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/mod.rs:230 | 230 | `column_track_counts_for_init` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/mod.rs:557 | 557 | `order` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/mod.rs:701 | 701 | `left` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/mod.rs:716 | 716 | `left` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/placement.rs:115 | 115 | `idx` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/placement.rs:142 | 142 | `idx` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/placement.rs:211 | 211 | `idx` is declared `mut` but never mutated |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/grid/placement.rs:409 | 409 | `secondary_span` is shadowed in a nested block — reduces readability |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/grid/placement.rs:428 | 428 | `primary_span` is shadowed in a nested block — reduces readability |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/grid/placement.rs:429 | 429 | `secondary_span` is shadowed in a nested block — reduces readability |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/placement.rs:534 | 534 | `items` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/placement.rs:535 | 535 | `cell_occupancy_matrix` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/placement.rs:537 | 537 | `name_resolver` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/placement.rs:555 | 555 | `sorted_children` is declared `mut` but never mutated |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:477 | 477 | `row_items` is shadowed in a nested block — reduces readability |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:557 | 557 | `item_sizer` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:560 | 560 | `batched_item_iterator` is declared `mut` but never mutated |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1200 | 1200 | `flex_fraction` is shadowed in a nested block — reduces readability |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1312 | 1312 | closure binding `flex_factor` shadows outer variable of same name |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1288 | 1288 | `used_space` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1289 | 1289 | `naive_flex_factor_sum` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1375 | 1375 | `space_to_distribute` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:143 | 143 | `data` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:334 | 334 | `size` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:518 | 518 | closure binding `size` shadows outer variable of same name |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/types/named.rs:107 | 107 | `current_line` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/grid/types/named.rs:147 | 147 | `current_line` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/leaf.rs:71 | 71 | `content_box_inset` is declared `mut` but never mutated |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/mod.rs:93 | 93 | match binding `min` shadows outer variable |
| INFO | style | shadowing | ./crates/aether-caelum/src/compute/mod.rs:93 | 93 | match binding `max` shadows outer variable |
| WARN | style | shadowing | ./crates/aether-caelum/src/compute/mod.rs:214 | 214 | `layout` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/geometry.rs:418 | 418 | `new` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/geometry.rs:431 | 431 | `new` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/geometry.rs:444 | 444 | `new` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/geometry.rs:457 | 457 | `new` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/geometry.rs:542 | 542 | `new` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:75 | 75 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:89 | 89 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:103 | 103 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:119 | 119 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:142 | 142 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:161 | 161 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:182 | 182 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:196 | 196 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:214 | 214 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:226 | 226 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:244 | 244 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:257 | 257 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:319 | 319 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:338 | 338 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:356 | 356 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:376 | 376 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:393 | 393 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:411 | 411 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:435 | 435 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:454 | 454 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:479 | 479 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:503 | 503 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:527 | 527 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:549 | 549 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:575 | 575 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:602 | 602 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:618 | 618 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:636 | 636 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:654 | 654 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:675 | 675 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:695 | 695 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:715 | 715 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:729 | 729 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:747 | 747 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:764 | 764 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:781 | 781 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/lib.rs:798 | 798 | `t` is declared `mut` but never mutated |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/style/available_space.rs:84 | 84 | match binding `value` shadows function parameter of same name |
| WARN | style | shadowing | ./crates/aether-caelum/src/style/mod.rs:563 | 563 | `s` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/style_helpers.rs:35 | 35 | `repeated_tracks` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/style_helpers.rs:37 | 37 | `tracks` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/tree/caelum_tree.rs:557 | 557 | `data` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/tree/caelum_tree.rs:890 | 890 | `caelum_view` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-caelum/src/tree/caelum_tree.rs:1241 | 1241 | `children` is declared `mut` but never mutated |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:47 | 47 | match binding `min` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:47 | 47 | match binding `max` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:48 | 48 | match binding `max` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:49 | 49 | match binding `min` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:113 | 113 | match binding `min` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:113 | 113 | match binding `max` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:114 | 114 | match binding `max` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:115 | 115 | match binding `min` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:178 | 178 | match binding `rhs` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:180 | 180 | match binding `rhs` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:182 | 182 | match binding `rhs` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:188 | 188 | match binding `rhs` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:197 | 197 | match binding `min` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:197 | 197 | match binding `max` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:198 | 198 | match binding `max` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:199 | 199 | match binding `min` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:208 | 208 | match binding `rhs` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/math.rs:216 | 216 | match binding `rhs` shadows function parameter of same name |
| WARN | style | shadowing | ./crates/aether-caelum/src/util/print.rs:8 | 8 | `buffer` is declared `mut` but never mutated |
| ERROR | correctness | shadowing | ./crates/aether-caelum/src/util/print.rs:60 | 60 | local `has_sibling` shadows function parameter of same name |
| WARN | style | shadowing | ./crates/aether-css/build.rs:21 | 21 | `from_str_arms` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/build.rs:22 | 22 | `display_arms` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/build.rs:23 | 23 | `variant_names` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/src/lib.rs:32 | 32 | `attrs` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/src/matcher.rs:104 | 104 | `matched` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/src/matcher.rs:137 | 137 | `attributes` is declared `mut` but never mutated |
| ERROR | correctness | shadowing | ./crates/aether-css/src/parser.rs:88 | 88 | local `input` shadows function parameter of same name |
| WARN | style | shadowing | ./crates/aether-css/src/parser.rs:98 | 98 | `stylesheet` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/src/parser.rs:99 | 99 | `rules` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/src/parser.rs:139 | 139 | `selectors` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/src/parser.rs:175 | 175 | `selector` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/src/parser.rs:227 | 227 | `declarations` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/src/parser.rs:268 | 268 | `value` is declared `mut` but never mutated |
| ERROR | correctness | shadowing | ./crates/aether-css/src/parser.rs:383 | 383 | local `s` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./crates/aether-css/src/parser.rs:395 | 395 | local `s` shadows function parameter of same name |
| WARN | style | shadowing | ./crates/aether-css/src/parser.rs:417 | 417 | `result` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/src/parser.rs:468 | 468 | `brace_depth` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/src/parser.rs:516 | 516 | `depth` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/src/resolver.rs:16 | 16 | `style` is declared `mut` but never mutated |
| INFO | style | shadowing | ./crates/aether-css/src/resolver.rs:222 | 222 | match binding `s` shadows outer variable |
| WARN | style | shadowing | ./crates/aether-css/src/resolver.rs:307 | 307 | `t` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-css/src/resolver.rs:490 | 490 | `attrs` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-dom/src/lib.rs:88 | 88 | `results` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-html/build.rs:16 | 16 | `output` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-html/src/lib.rs:28 | 28 | `doc_node` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-html/src/lib.rs:66 | 66 | `content` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-html/src/lib.rs:79 | 79 | `content` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-html/src/lib.rs:107 | 107 | `attributes` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-html/src/lib.rs:174 | 174 | `content` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-html/src/lib.rs:185 | 185 | `nodes` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-html/src/lib.rs:224 | 224 | `result` is declared `mut` but never mutated |
| WARN | style | shadowing | ./crates/aether-html/src/lib.rs:242 | 242 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./examples/korlang_demo.rs:17 | 17 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./examples/korlang_repl.rs:7 | 7 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./examples/korlang_repl.rs:13 | 13 | `input` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/formatter.rs:5 | 5 | `out` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/formatter.rs:18 | 18 | `out` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/formatter.rs:30 | 30 | `out` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/formatter.rs:52 | 52 | `out` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/formatter.rs:65 | 65 | `out` is declared `mut` but never mutated |
| ERROR | correctness | shadowing | ./korlang/src/compiler/formatter.rs:101 | 101 | match binding `expr` shadows function parameter of same name |
| WARN | style | shadowing | ./korlang/src/compiler/formatter.rs:88 | 88 | `out` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/lexer.rs:28 | 28 | `tokens` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/lexer.rs:104 | 104 | `id` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/lexer.rs:127 | 127 | `parts` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/lexer.rs:128 | 128 | `vars` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/lexer.rs:129 | 129 | `current` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/lexer.rs:147 | 147 | `var` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/lexer.rs:166 | 166 | `s` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/mod.rs:11 | 11 | `lexer` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/mod.rs:13 | 13 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/mod.rs:14 | 14 | `bytecode` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/mod.rs:25 | 25 | `body_ops` is declared `mut` but never mutated |
| ERROR | correctness | shadowing | ./korlang/src/compiler/mod.rs:118 | 118 | match binding `expr` shadows function parameter of same name |
| WARN | style | shadowing | ./korlang/src/compiler/parser.rs:70 | 70 | `states` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/parser.rs:71 | 71 | `functions` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/parser.rs:108 | 108 | `params` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/parser.rs:115 | 115 | `body` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/parser.rs:135 | 135 | `then_branch` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/parser.rs:140 | 140 | `else_branch` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/parser.rs:157 | 157 | `body` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/parser.rs:167 | 167 | `properties` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/parser.rs:178 | 178 | `children` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/parser.rs:272 | 272 | `args` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/compiler/parser.rs:289 | 289 | `items` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/lib.rs:31 | 31 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/lib.rs:46 | 46 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/lib.rs:61 | 61 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/lib.rs:72 | 72 | `lexer` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/lib.rs:73 | 73 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/vm/mod.rs:200 | 200 | `items` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/vm/mod.rs:251 | 251 | `args` is declared `mut` but never mutated |
| WARN | style | shadowing | ./korlang/src/vm/mod.rs:268 | 268 | `parts` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/events.rs:21 | 21 | `results` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:106 | 106 | `simples` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:141 | 141 | `pos` is declared `mut` but never mutated |
| ERROR | correctness | shadowing | ./src/engine/js/js_bridge.rs:160 | 160 | match binding `combinator` shadows function parameter of same name |
| ERROR | correctness | shadowing | ./src/engine/js/js_bridge.rs:253 | 253 | local `s` shadows function parameter of same name |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:300 | 300 | `parts` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:451 | 451 | `fn_` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:466 | 466 | `n` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:470 | 470 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:481 | 481 | `stack` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:583 | 583 | `out` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:604 | 604 | `html` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:627 | 627 | `html` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:657 | 657 | `stack` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:712 | 712 | `result` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:754 | 754 | `attr_end` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:764 | 764 | `attrs` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:838 | 838 | `attrs` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:841 | 841 | `key` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/js_bridge.rs:842 | 842 | `value` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/selector.rs:16 | 16 | `results` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/timers.rs:34 | 34 | `ready` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/js/timers.rs:35 | 35 | `i` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/korlang.rs:20 | 20 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/korlang.rs:66 | 66 | `render_vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/net/mod.rs:132 | 132 | `last` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/net/mod.rs:192 | 192 | `parts` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/net/mod.rs:377 | 377 | `policy` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/net/mod.rs:381 | 381 | `tokens` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/net/mod.rs:639 | 639 | `headers` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:133 | 133 | `result` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:134 | 134 | `chars` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:137 | 137 | `entity` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:193 | 193 | `out` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:353 | 353 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:358 | 358 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:368 | 368 | `visible_idx` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:373 | 373 | `child_path` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:524 | 524 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:594 | 594 | `visible_idx` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:599 | 599 | `child_path` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:625 | 625 | `out` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:632 | 632 | `out` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:685 | 685 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:690 | 690 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:799 | 799 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:849 | 849 | `path` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:891 | 891 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:894 | 894 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:903 | 903 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:906 | 906 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:915 | 915 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/extractor.rs:918 | 918 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/fetcher.rs:168 | 168 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/fetcher.rs:172 | 172 | `styles` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/fetcher.rs:177 | 177 | `stylesheet` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/fetcher.rs:196 | 196 | `link_urls` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/fetcher.rs:243 | 243 | `scripts` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/fetcher.rs:247 | 247 | `js_engine` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/fetcher.rs:284 | 284 | `guard` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/fetcher.rs:294 | 294 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/fetcher.rs:301 | 301 | `img_count` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/engine/pipeline/layout.rs:78 | 78 | `s` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/logging.rs:12 | 12 | `p` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/logging.rs:31 | 31 | `file` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:253 | 253 | `kor_vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:272 | 272 | `sidebar_kor_vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:285 | 285 | `sidebar_ws_kor_vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:501 | 501 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:517 | 517 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:528 | 528 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:567 | 567 | `all` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:821 | 821 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:826 | 826 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:971 | 971 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:1044 | 1044 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:1056 | 1056 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:1067 | 1067 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:1078 | 1078 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:1090 | 1090 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:1101 | 1101 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./src/ui/screens/browser.rs:1122 | 1122 | `attrs` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/css_regression.rs:15 | 15 | `map` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/integration_test.rs:7 | 7 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/integration_test.rs:27 | 27 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/integration_test.rs:70 | 70 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/integration_test.rs:74 | 74 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/integration_test.rs:110 | 110 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/integration_test.rs:138 | 138 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/integration_test.rs:161 | 161 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:6 | 6 | `attrs` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:8 | 8 | `child_attrs` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:37 | 37 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:44 | 44 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:55 | 55 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:63 | 63 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:71 | 71 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:82 | 82 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:92 | 92 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:108 | 108 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:116 | 116 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:127 | 127 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:136 | 136 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:146 | 146 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:155 | 155 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:168 | 168 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:179 | 179 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:200 | 200 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:230 | 230 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:257 | 257 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:307 | 307 | `body_attrs` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:314 | 314 | `doc` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:322 | 322 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:333 | 333 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:343 | 343 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:390 | 390 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:397 | 397 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:404 | 404 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:411 | 411 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:418 | 418 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:427 | 427 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:435 | 435 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:445 | 445 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:458 | 458 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:468 | 468 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:480 | 480 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:489 | 489 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:501 | 501 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:517 | 517 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:529 | 529 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:539 | 539 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:552 | 552 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:560 | 560 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:568 | 568 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:577 | 577 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:589 | 589 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:603 | 603 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:618 | 618 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_bridge_test.rs:631 | 631 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:6 | 6 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:45 | 45 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:60 | 60 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:82 | 82 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:108 | 108 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:132 | 132 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:150 | 150 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:169 | 169 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:176 | 176 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:183 | 183 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:192 | 192 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:201 | 201 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:215 | 215 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:227 | 227 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:236 | 236 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:249 | 249 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:261 | 261 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:277 | 277 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:287 | 287 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:297 | 297 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:310 | 310 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:321 | 321 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:340 | 340 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:350 | 350 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:364 | 364 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:375 | 375 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:385 | 385 | `b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:434 | 434 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:444 | 444 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:453 | 453 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:464 | 464 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/js_engine_tests.rs:474 | 474 | `bridge` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:11 | 11 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:41 | 41 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:55 | 55 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:79 | 79 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:103 | 103 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:120 | 120 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:136 | 136 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:152 | 152 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:170 | 170 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:187 | 187 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:204 | 204 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:220 | 220 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:236 | 236 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:252 | 252 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:270 | 270 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:293 | 293 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:318 | 318 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:359 | 359 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:379 | 379 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/korlang_advanced_tests.rs:398 | 398 | `vm` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:72 | 72 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:81 | 81 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:93 | 93 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:103 | 103 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:105 | 105 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:116 | 116 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:126 | 126 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:136 | 136 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:139 | 139 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:154 | 154 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:156 | 156 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:166 | 166 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:168 | 168 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:185 | 185 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:187 | 187 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:206 | 206 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:229 | 229 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:257 | 257 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/layout_stress.rs:259 | 259 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering.rs:13 | 13 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:19 | 19 | `map` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:233 | 233 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:236 | 236 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:244 | 244 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:247 | 247 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:255 | 255 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:258 | 258 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:267 | 267 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:270 | 270 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:282 | 282 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:293 | 293 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:303 | 303 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:313 | 313 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:324 | 324 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:338 | 338 | `parent` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:340 | 340 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:342 | 342 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:353 | 353 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:364 | 364 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:375 | 375 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:385 | 385 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:399 | 399 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:402 | 402 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:412 | 412 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:429 | 429 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:445 | 445 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:461 | 461 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:476 | 476 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:491 | 491 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:510 | 510 | `el_a` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:512 | 512 | `el_b` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:514 | 514 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:539 | 539 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:556 | 556 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:573 | 573 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:588 | 588 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:605 | 605 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:608 | 608 | `elements` is declared `mut` but never mutated |
| INFO | style | shadowing | ./tests/rendering_pipeline_tests.rs:629 | 629 | loop variable `el` shadows outer variable |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:620 | 620 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:623 | 623 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:636 | 636 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:647 | 647 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:657 | 657 | `el1` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:660 | 660 | `el2` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:663 | 663 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:681 | 681 | `parent` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:683 | 683 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:685 | 685 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:692 | 692 | `el1` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:694 | 694 | `el2` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:696 | 696 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:704 | 704 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:715 | 715 | `parent` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:717 | 717 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:719 | 719 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:726 | 726 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:743 | 743 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:755 | 755 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:759 | 759 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:766 | 766 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:769 | 769 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:776 | 776 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:786 | 786 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:789 | 789 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:808 | 808 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:816 | 816 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:823 | 823 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:833 | 833 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:843 | 843 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:853 | 853 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:856 | 856 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:864 | 864 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:867 | 867 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:875 | 875 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:887 | 887 | `el_small` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:889 | 889 | `el_large` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:891 | 891 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:963 | 963 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:966 | 966 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:975 | 975 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:978 | 978 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:987 | 987 | `parser` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/rendering_pipeline_tests.rs:990 | 990 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:291 | 291 | `s` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:301 | 301 | `s` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:374 | 374 | `hist_entries` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:473 | 473 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:482 | 482 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:491 | 491 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:500 | 500 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:509 | 509 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:529 | 529 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:538 | 538 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:549 | 549 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:562 | 562 | `el` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:564 | 564 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:572 | 572 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:584 | 584 | `elements` is declared `mut` but never mutated |
| WARN | style | shadowing | ./tests/sidebar_tests.rs:800 | 800 | `tab` is declared `mut` but never mutated |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:251 | 251 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:270 | 270 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:289 | 289 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:357 | 357 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:418 | 418 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:455 | 455 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:536 | 536 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:632 | 632 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:694 | 694 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:695 | 695 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:742 | 742 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:765 | 765 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:796 | 796 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:798 | 798 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:814 | 814 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:841 | 841 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:857 | 857 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:869 | 869 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:875 | 875 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:901 | 901 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:915 | 915 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:925 | 925 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:930 | 930 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:974 | 974 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1001 | 1001 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1008 | 1008 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1017 | 1017 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1055 | 1055 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1130 | 1130 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1131 | 1131 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1132 | 1132 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1133 | 1133 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1168 | 1168 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1180 | 1180 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1202 | 1202 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1205 | 1205 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1211 | 1211 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1214 | 1214 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1230 | 1230 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/block.rs:1231 | 1231 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/common/alignment.rs:51 | 51 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/common/alignment.rs:55 | 55 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/common/alignment.rs:58 | 58 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/common/alignment.rs:63 | 63 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/common/alignment.rs:66 | 66 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/common/alignment.rs:73 | 73 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/common/alignment.rs:76 | 76 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/common/alignment.rs:80 | 80 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/common/alignment.rs:83 | 83 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/common/content_size.rs:31 | 31 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:178 | 178 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:197 | 197 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:215 | 215 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:359 | 359 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:395 | 395 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:402 | 402 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:434 | 434 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:521 | 521 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:701 | 701 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:757 | 757 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:800 | 800 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:910 | 910 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:937 | 937 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:961 | 961 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1020 | 1020 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1027 | 1027 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1086 | 1086 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1101 | 1101 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1137 | 1137 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1221 | 1221 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1224 | 1224 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1252 | 1252 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1255 | 1255 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1264 | 1264 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1274 | 1274 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1327 | 1327 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1377 | 1377 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1390 | 1390 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1394 | 1394 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1394 | 1394 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1395 | 1395 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1395 | 1395 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1399 | 1399 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1400 | 1400 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1450 | 1450 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1451 | 1451 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1452 | 1452 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1455 | 1455 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1456 | 1456 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1457 | 1457 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1463 | 1463 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1464 | 1464 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1469 | 1469 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1470 | 1470 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1523 | 1523 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1529 | 1529 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1622 | 1622 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1633 | 1633 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1681 | 1681 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1688 | 1688 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1703 | 1703 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1710 | 1710 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1748 | 1748 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1754 | 1754 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1757 | 1757 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1781 | 1781 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1784 | 1784 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1788 | 1788 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1791 | 1791 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1795 | 1795 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1798 | 1798 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1802 | 1802 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1805 | 1805 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1810 | 1810 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1819 | 1819 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1824 | 1824 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1827 | 1827 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1886 | 1886 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1893 | 1893 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1914 | 1914 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1916 | 1916 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1930 | 1930 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1935 | 1935 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1938 | 1938 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1942 | 1942 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1965 | 1965 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1969 | 1969 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1970 | 1970 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1989 | 1989 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:1996 | 1996 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2023 | 2023 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2048 | 2048 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2079 | 2079 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2100 | 2100 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2118 | 2118 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2136 | 2136 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2164 | 2164 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2256 | 2256 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2264 | 2264 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2279 | 2279 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2280 | 2280 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2288 | 2288 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2290 | 2290 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2291 | 2291 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2293 | 2293 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2310 | 2310 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2318 | 2318 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2325 | 2325 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2328 | 2328 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2331 | 2331 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2337 | 2337 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2343 | 2343 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2348 | 2348 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2354 | 2354 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2381 | 2381 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2389 | 2389 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2394 | 2394 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2397 | 2397 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2403 | 2403 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2409 | 2409 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2413 | 2413 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2417 | 2417 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2423 | 2423 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2440 | 2440 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2441 | 2441 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2471 | 2471 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2481 | 2481 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/flexbox.rs:2503 | 2503 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/float.rs:285 | 285 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/float.rs:311 | 311 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/float.rs:375 | 375 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/float.rs:401 | 401 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/float.rs:481 | 481 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:37 | 37 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:52 | 52 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:100 | 100 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:125 | 125 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:128 | 128 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:132 | 132 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:135 | 135 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:158 | 158 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:171 | 171 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:183 | 183 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:196 | 196 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:220 | 220 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:258 | 258 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:259 | 259 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:302 | 302 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:311 | 311 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:314 | 314 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:318 | 318 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:321 | 321 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:330 | 330 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:333 | 333 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:341 | 341 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/alignment.rs:349 | 349 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:35 | 35 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:36 | 36 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:42 | 42 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:52 | 52 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:70 | 70 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:81 | 81 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:87 | 87 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:124 | 124 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:131 | 131 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:218 | 218 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:250 | 250 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:128 | 128 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:132 | 132 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:152 | 152 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:156 | 156 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:59 | 59 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:77 | 77 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:137 | 137 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:202 | 202 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:206 | 206 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:243 | 243 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:249 | 249 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:288 | 288 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:294 | 294 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:315 | 315 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:343 | 343 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:404 | 404 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:464 | 464 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:502 | 502 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:503 | 503 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:551 | 551 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:586 | 586 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:588 | 588 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:591 | 591 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:599 | 599 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:608 | 608 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:618 | 618 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:621 | 621 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:625 | 625 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:628 | 628 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:639 | 639 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:659 | 659 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:678 | 678 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:728 | 728 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/mod.rs:735 | 735 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:23 | 23 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:37 | 37 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:47 | 47 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:72 | 72 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:145 | 145 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:154 | 154 | `&*x` — unnecessary reborrow if `x` is already a reference |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:222 | 222 | `&*x` — unnecessary reborrow if `x` is already a reference |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:310 | 310 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:338 | 338 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:339 | 339 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:374 | 374 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:395 | 395 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:400 | 400 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:419 | 419 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:452 | 452 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:436 | 436 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:530 | 530 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:576 | 576 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:595 | 595 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:662 | 662 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:678 | 678 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:697 | 697 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:715 | 715 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:733 | 733 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/placement.rs:751 | 751 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:39 | 39 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:48 | 48 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:51 | 51 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:160 | 160 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:188 | 188 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:218 | 218 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:232 | 232 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:340 | 340 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:396 | 396 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:400 | 400 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:572 | 572 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:577 | 577 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:580 | 580 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:584 | 584 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:611 | 611 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:616 | 616 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:619 | 619 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:662 | 662 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:696 | 696 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:705 | 705 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:723 | 723 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:759 | 759 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:805 | 805 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:904 | 904 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:958 | 958 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:968 | 968 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1029 | 1029 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1032 | 1032 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1033 | 1033 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1089 | 1089 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1100 | 1100 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1106 | 1106 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1111 | 1111 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1120 | 1120 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1191 | 1191 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1210 | 1210 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1238 | 1238 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1242 | 1242 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1253 | 1253 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1275 | 1275 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1296 | 1296 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1310 | 1310 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1315 | 1315 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1344 | 1344 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| WARN | style | redundancy | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:1375 | 1375 | Self-assignment `let space_to_distribute = space_to_distribute;` — redundant |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:25 | 25 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:33 | 33 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:40 | 40 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:121 | 121 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:125 | 125 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:253 | 253 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:261 | 261 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:287 | 287 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:291 | 291 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:295 | 295 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:299 | 299 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:321 | 321 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:325 | 325 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:329 | 329 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:333 | 333 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:89 | 89 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:110 | 110 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:114 | 114 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:124 | 124 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/coordinates.rs:133 | 133 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:199 | 199 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:205 | 205 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:210 | 210 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:225 | 225 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:231 | 231 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:236 | 236 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:257 | 257 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:284 | 284 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:299 | 299 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:324 | 324 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:327 | 327 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:471 | 471 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:478 | 478 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:525 | 525 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_item.rs:533 | 533 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_track.rs:97 | 97 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_track.rs:98 | 98 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/grid_track.rs:152 | 152 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:94 | 94 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:96 | 96 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:98 | 98 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:100 | 100 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:222 | 222 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:230 | 230 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:249 | 249 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:265 | 265 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| WARN | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:303 | 303 | Self-assignment `let idx = idx;` — redundant |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:320 | 320 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:327 | 327 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:339 | 339 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:339 | 339 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:340 | 340 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:345 | 345 | `&*x` — unnecessary reborrow if `x` is already a reference |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:347 | 347 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:347 | 347 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:352 | 352 | `&*x` — unnecessary reborrow if `x` is already a reference |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:354 | 354 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:354 | 354 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:367 | 367 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/grid/types/named.rs:369 | 369 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/leaf.rs:32 | 32 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/leaf.rs:97 | 97 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/leaf.rs:117 | 117 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/leaf.rs:127 | 127 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/mod.rs:72 | 72 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/mod.rs:130 | 130 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/mod.rs:131 | 131 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/mod.rs:136 | 136 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/compute/mod.rs:180 | 180 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:221 | 221 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:233 | 233 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:247 | 247 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:256 | 256 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:265 | 265 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:274 | 274 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:398 | 398 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:409 | 409 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:421 | 421 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:434 | 434 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:447 | 447 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:460 | 460 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:472 | 472 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:483 | 483 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:545 | 545 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:670 | 670 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/geometry.rs:681 | 681 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/available_space.rs:30 | 30 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/compact_length.rs:437 | 437 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/compact_length.rs:442 | 442 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/compact_length.rs:447 | 447 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/dimension.rs:16 | 16 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/dimension.rs:21 | 21 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/dimension.rs:80 | 80 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/dimension.rs:85 | 85 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/dimension.rs:177 | 177 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/dimension.rs:182 | 182 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:355 | 355 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:494 | 494 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:497 | 497 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:521 | 521 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:524 | 524 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:578 | 578 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:583 | 583 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:593 | 593 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:850 | 850 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:855 | 855 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:877 | 877 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:1070 | 1070 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:1070 | 1070 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:1075 | 1075 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:1075 | 1075 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:1080 | 1080 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/grid.rs:1080 | 1080 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:566 | 566 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:567 | 567 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:568 | 568 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:569 | 569 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:570 | 570 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:580 | 580 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:596 | 596 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:600 | 600 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:604 | 604 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:613 | 613 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:617 | 617 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:625 | 625 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:629 | 629 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/style/mod.rs:633 | 633 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:70 | 70 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:79 | 79 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:79 | 79 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:380 | 380 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:385 | 385 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:390 | 390 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:390 | 390 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:401 | 401 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:401 | 401 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:412 | 412 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:412 | 412 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:424 | 424 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:425 | 425 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:426 | 426 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:427 | 427 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:450 | 450 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:455 | 455 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:460 | 460 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:460 | 460 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:472 | 472 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:472 | 472 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:484 | 484 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:484 | 484 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:497 | 497 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:498 | 498 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:499 | 499 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/style_helpers.rs:500 | 500 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/cache.rs:79 | 79 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/cache.rs:85 | 85 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/cache.rs:91 | 91 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/cache.rs:141 | 141 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/cache.rs:174 | 174 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:53 | 53 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:56 | 56 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:181 | 181 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:186 | 186 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:191 | 191 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:201 | 201 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:205 | 205 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:209 | 209 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:217 | 217 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:239 | 239 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:240 | 240 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:241 | 241 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:278 | 278 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:287 | 287 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:300 | 300 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:304 | 304 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:363 | 363 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:368 | 368 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:392 | 392 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:396 | 396 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:400 | 400 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:455 | 455 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:460 | 460 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:480 | 480 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:485 | 485 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:490 | 490 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:502 | 502 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:507 | 507 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:550 | 550 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:566 | 566 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:571 | 571 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:574 | 574 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:594 | 594 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:596 | 596 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:604 | 604 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:618 | 618 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:635 | 635 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:641 | 641 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:649 | 649 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:654 | 654 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:655 | 655 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:665 | 665 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:669 | 669 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:672 | 672 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:681 | 681 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:685 | 685 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:691 | 691 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:694 | 694 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:710 | 710 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:718 | 718 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:721 | 721 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:725 | 725 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:741 | 741 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:743 | 743 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:759 | 759 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:763 | 763 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:766 | 766 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:768 | 768 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:778 | 778 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:781 | 781 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:799 | 799 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:804 | 804 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:810 | 810 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:818 | 818 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:825 | 825 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:826 | 826 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:827 | 827 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:834 | 834 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:843 | 843 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:859 | 859 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:861 | 861 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:867 | 867 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/caelum_tree.rs:875 | 875 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/layout.rs:45 | 45 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/layout.rs:54 | 54 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/node.rs:55 | 55 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/tree/traits.rs:331 | 331 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./crates/aether-caelum/src/util/print.rs:32 | 32 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-caelum/src/util/print.rs:55 | 55 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/matcher.rs:44 | 44 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/matcher.rs:57 | 57 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/matcher.rs:73 | 73 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/matcher.rs:75 | 75 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/matcher.rs:84 | 84 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/matcher.rs:93 | 93 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:91 | 91 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:143 | 143 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:155 | 155 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:161 | 161 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:166 | 166 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:201 | 201 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:213 | 213 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:216 | 216 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:290 | 290 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:296 | 296 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:301 | 301 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:305 | 305 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:309 | 309 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:321 | 321 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:337 | 337 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:345 | 345 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:347 | 347 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:365 | 365 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:375 | 375 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:387 | 387 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:398 | 398 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:407 | 407 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:408 | 408 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:409 | 409 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:423 | 423 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:437 | 437 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/parser.rs:499 | 499 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:41 | 41 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:56 | 56 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:63 | 63 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:71 | 71 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:74 | 74 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:77 | 77 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:80 | 80 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:100 | 100 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:107 | 107 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:143 | 143 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:144 | 144 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:200 | 200 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:226 | 226 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:232 | 232 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:236 | 236 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:236 | 236 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:324 | 324 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:364 | 364 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/resolver.rs:413 | 413 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-css/src/style_value.rs:62 | 62 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-css/src/style_value.rs:265 | 265 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-dom/src/lib.rs:76 | 76 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-html/src/lib.rs:37 | 37 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-html/src/lib.rs:43 | 43 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-html/src/lib.rs:46 | 46 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-html/src/lib.rs:59 | 59 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-html/src/lib.rs:94 | 94 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-html/src/lib.rs:104 | 104 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./crates/aether-html/src/lib.rs:128 | 128 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-html/src/lib.rs:132 | 132 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./crates/aether-html/src/lib.rs:156 | 156 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/formatter.rs:46 | 46 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/formatter.rs:59 | 59 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/formatter.rs:98 | 98 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/formatter.rs:101 | 101 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/lexer.rs:77 | 77 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/lexer.rs:78 | 78 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/lexer.rs:79 | 79 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/lexer.rs:80 | 80 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/lexer.rs:152 | 152 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/lexer.rs:160 | 160 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:68 | 68 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:68 | 68 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:74 | 74 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:75 | 75 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:82 | 82 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:89 | 89 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:89 | 89 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:106 | 106 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:106 | 106 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:117 | 117 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:137 | 137 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:144 | 144 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:153 | 153 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:153 | 153 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:159 | 159 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:166 | 166 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:166 | 166 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:171 | 171 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:188 | 188 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:236 | 236 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:248 | 248 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:258 | 258 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:270 | 270 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:279 | 279 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:298 | 298 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:298 | 298 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./korlang/src/compiler/parser.rs:308 | 308 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:118 | 118 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:125 | 125 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:132 | 132 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:138 | 138 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:139 | 139 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:175 | 175 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:182 | 182 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:189 | 189 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:196 | 196 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:208 | 208 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:217 | 217 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:218 | 218 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:244 | 244 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:247 | 247 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:249 | 249 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:278 | 278 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./korlang/src/vm/mod.rs:286 | 286 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/fetch.rs:15 | 15 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:72 | 72 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:80 | 80 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:87 | 87 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:93 | 93 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:113 | 113 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:128 | 128 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:133 | 133 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:140 | 140 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:149 | 149 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:161 | 161 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:164 | 164 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:165 | 165 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:173 | 173 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:181 | 181 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:239 | 239 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:244 | 244 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:256 | 256 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:262 | 262 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:268 | 268 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:272 | 272 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:295 | 295 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:295 | 295 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:302 | 302 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:309 | 309 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:312 | 312 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:320 | 320 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:326 | 326 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:344 | 344 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:350 | 350 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:363 | 363 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:446 | 446 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:485 | 485 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:512 | 512 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:603 | 603 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:626 | 626 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:661 | 661 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:672 | 672 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:675 | 675 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:699 | 699 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:700 | 700 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:701 | 701 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:711 | 711 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:714 | 714 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:748 | 748 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:820 | 820 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:840 | 840 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:854 | 854 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:874 | 874 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:881 | 881 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:923 | 923 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1002 | 1002 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1027 | 1027 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1591 | 1591 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1602 | 1602 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1605 | 1605 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1611 | 1611 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1614 | 1614 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1620 | 1620 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1629 | 1629 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1638 | 1638 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1641 | 1641 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1647 | 1647 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1650 | 1650 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1656 | 1656 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1665 | 1665 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1668 | 1668 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1674 | 1674 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1683 | 1683 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1686 | 1686 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1692 | 1692 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1695 | 1695 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1701 | 1701 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1704 | 1704 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1711 | 1711 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1714 | 1714 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1720 | 1720 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1723 | 1723 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1730 | 1730 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1733 | 1733 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1739 | 1739 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1742 | 1742 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1748 | 1748 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1751 | 1751 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1757 | 1757 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1760 | 1760 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1766 | 1766 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1769 | 1769 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1775 | 1775 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1778 | 1778 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1784 | 1784 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1787 | 1787 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1793 | 1793 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1796 | 1796 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1803 | 1803 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1806 | 1806 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1812 | 1812 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1815 | 1815 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1821 | 1821 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1831 | 1831 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1840 | 1840 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1850 | 1850 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1853 | 1853 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1859 | 1859 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1869 | 1869 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1870 | 1870 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1876 | 1876 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1883 | 1883 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1884 | 1884 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1890 | 1890 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1891 | 1891 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1897 | 1897 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1898 | 1898 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1904 | 1904 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1905 | 1905 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1911 | 1911 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1912 | 1912 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1918 | 1918 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1919 | 1919 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1925 | 1925 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1932 | 1932 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1939 | 1939 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1947 | 1947 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1950 | 1950 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1957 | 1957 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1960 | 1960 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1967 | 1967 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1970 | 1970 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1978 | 1978 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1988 | 1988 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1991 | 1991 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:1999 | 1999 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2008 | 2008 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2017 | 2017 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2026 | 2026 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2029 | 2029 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2037 | 2037 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2040 | 2040 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2049 | 2049 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2050 | 2050 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2056 | 2056 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2064 | 2064 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2065 | 2065 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2071 | 2071 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2081 | 2081 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2091 | 2091 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2098 | 2098 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2105 | 2105 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2114 | 2114 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2116 | 2116 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/js_bridge.rs:2131 | 2131 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/mod.rs:41 | 41 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/mod.rs:47 | 47 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/mod.rs:63 | 63 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/mod.rs:76 | 76 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/mod.rs:85 | 85 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/selector.rs:12 | 12 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/js/selector.rs:21 | 21 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/storage.rs:77 | 77 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/js/timers.rs:47 | 47 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/korlang.rs:18 | 18 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/korlang.rs:18 | 18 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/korlang.rs:25 | 25 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/korlang.rs:26 | 26 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/korlang.rs:34 | 34 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/korlang.rs:40 | 40 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/korlang.rs:47 | 47 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/korlang.rs:53 | 53 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/korlang.rs:70 | 70 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mock.rs:40 | 40 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mock.rs:47 | 47 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mock.rs:54 | 54 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:33 | 33 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:46 | 46 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:54 | 54 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:86 | 86 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:120 | 120 | `&*x` — unnecessary reborrow if `x` is already a reference |
| INFO | style | redundancy | ./src/engine/net/mod.rs:143 | 143 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:148 | 148 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:155 | 155 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:158 | 158 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:163 | 163 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:168 | 168 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:272 | 272 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:279 | 279 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:280 | 280 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:281 | 281 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:287 | 287 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:288 | 288 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:290 | 290 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:291 | 291 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:298 | 298 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:303 | 303 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:305 | 305 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:352 | 352 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:354 | 354 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:362 | 362 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:363 | 363 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:367 | 367 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:368 | 368 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:371 | 371 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:415 | 415 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:439 | 439 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:452 | 452 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:459 | 459 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:465 | 465 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:474 | 474 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:483 | 483 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:492 | 492 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:525 | 525 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:526 | 526 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:534 | 534 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:538 | 538 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:562 | 562 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:566 | 566 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:570 | 570 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:575 | 575 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:589 | 589 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:607 | 607 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:630 | 630 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./src/engine/net/mod.rs:636 | 636 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:664 | 664 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:673 | 673 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:682 | 682 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:683 | 683 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:714 | 714 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:720 | 720 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:754 | 754 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:731 | 731 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./src/engine/net/mod.rs:736 | 736 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:771 | 771 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:775 | 775 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:781 | 781 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:786 | 786 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:792 | 792 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/net/mod.rs:804 | 804 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/net/mod.rs:813 | 813 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:131 | 131 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:151 | 151 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:156 | 156 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:169 | 169 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:187 | 187 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:416 | 416 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:419 | 419 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:420 | 420 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:428 | 428 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:431 | 431 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:459 | 459 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:460 | 460 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:484 | 484 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:488 | 488 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:489 | 489 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:490 | 490 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:507 | 507 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:510 | 510 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:513 | 513 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:559 | 559 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:565 | 565 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:568 | 568 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:572 | 572 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:575 | 575 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:585 | 585 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:588 | 588 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:718 | 718 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:723 | 723 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:728 | 728 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:733 | 733 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:738 | 738 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:741 | 741 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:748 | 748 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:758 | 758 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:766 | 766 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:776 | 776 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:784 | 784 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:787 | 787 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:790 | 790 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:825 | 825 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:833 | 833 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/extractor.rs:835 | 835 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:73 | 73 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:76 | 76 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:77 | 77 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:116 | 116 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:116 | 116 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:117 | 117 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:119 | 119 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:120 | 120 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:120 | 120 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:120 | 120 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:121 | 121 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:123 | 123 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:123 | 123 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:125 | 125 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:154 | 154 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:164 | 164 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:189 | 189 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:215 | 215 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:223 | 223 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:330 | 330 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/fetcher.rs:337 | 337 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:15 | 15 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:19 | 19 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:62 | 62 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:63 | 63 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:70 | 70 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:74 | 74 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:108 | 108 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:113 | 113 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:145 | 145 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:148 | 148 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:179 | 179 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:180 | 180 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:181 | 181 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:182 | 182 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:205 | 205 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/layout.rs:232 | 232 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/pipeline/navigator.rs:12 | 12 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/pipeline/navigator.rs:14 | 14 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/engine/pipeline/navigator.rs:21 | 21 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/style.rs:6 | 6 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/engine/style.rs:16 | 16 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:10 | 10 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:11 | 11 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:25 | 25 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:35 | 35 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:43 | 43 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:48 | 48 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:52 | 52 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:58 | 58 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:78 | 78 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:81 | 81 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:83 | 83 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:87 | 87 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:88 | 88 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:91 | 91 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:93 | 93 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:96 | 96 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:104 | 104 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:107 | 107 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:114 | 114 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:118 | 118 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:129 | 129 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:131 | 131 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:132 | 132 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:137 | 137 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:140 | 140 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/kor_renderer.rs:143 | 143 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/ui/mod.rs:47 | 47 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/mod.rs:58 | 58 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/mod.rs:63 | 63 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/mod.rs:72 | 72 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/mod.rs:78 | 78 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/mod.rs:110 | 110 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:219 | 219 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:225 | 225 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:226 | 226 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:235 | 235 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:232 | 232 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:358 | 358 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:384 | 384 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:396 | 396 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:412 | 412 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:422 | 422 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:432 | 432 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:442 | 442 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:471 | 471 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:476 | 476 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:490 | 490 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:525 | 525 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:538 | 538 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:550 | 550 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:561 | 561 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:658 | 658 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:661 | 661 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:665 | 665 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:680 | 680 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:681 | 681 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:682 | 682 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:692 | 692 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:693 | 693 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:696 | 696 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:697 | 697 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:698 | 698 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:705 | 705 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:708 | 708 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:711 | 711 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:713 | 713 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:716 | 716 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:718 | 718 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:721 | 721 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:722 | 722 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:725 | 725 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:726 | 726 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:727 | 727 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:732 | 732 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:741 | 741 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:747 | 747 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:753 | 753 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:766 | 766 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:779 | 779 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:793 | 793 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:795 | 795 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:796 | 796 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:803 | 803 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:804 | 804 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:815 | 815 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:846 | 846 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:852 | 852 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:853 | 853 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:857 | 857 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:858 | 858 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:860 | 860 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:861 | 861 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:862 | 862 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:868 | 868 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:880 | 880 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:882 | 882 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:883 | 883 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:887 | 887 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:889 | 889 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:890 | 890 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:907 | 907 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:915 | 915 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:916 | 916 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:922 | 922 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:926 | 926 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:930 | 930 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:935 | 935 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:938 | 938 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:939 | 939 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:943 | 943 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:962 | 962 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:975 | 975 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:983 | 983 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:988 | 988 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/browser.rs:1008 | 1008 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/palette.rs:78 | 78 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/palette.rs:113 | 113 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/palette.rs:131 | 131 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/palette.rs:132 | 132 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/palette.rs:160 | 160 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/palette.rs:167 | 167 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/palette.rs:192 | 192 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/palette.rs:193 | 193 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/palette.rs:205 | 205 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/palette.rs:212 | 212 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/palette.rs:297 | 297 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/palette.rs:301 | 301 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:128 | 128 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:152 | 152 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:176 | 176 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:245 | 245 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:268 | 268 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:286 | 286 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:299 | 299 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:301 | 301 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:303 | 303 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:309 | 309 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:310 | 310 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:321 | 321 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:324 | 324 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:330 | 330 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:331 | 331 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:342 | 342 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:345 | 345 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:350 | 350 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:351 | 351 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:381 | 381 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:385 | 385 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:393 | 393 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:400 | 400 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:410 | 410 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:411 | 411 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:441 | 441 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:445 | 445 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:453 | 453 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:460 | 460 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:479 | 479 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:487 | 487 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:490 | 490 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:491 | 491 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:528 | 528 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/screens/settings.rs:539 | 539 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/style.rs:45 | 45 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/style.rs:64 | 64 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/style.rs:90 | 90 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/style.rs:102 | 102 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/style.rs:113 | 113 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/style.rs:122 | 122 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/style.rs:133 | 133 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/style.rs:142 | 142 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/style.rs:154 | 154 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/style.rs:156 | 156 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/style.rs:162 | 162 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/style.rs:167 | 167 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/style.rs:179 | 179 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/style.rs:181 | 181 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/style.rs:187 | 187 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/style.rs:188 | 188 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/style.rs:194 | 194 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./src/ui/style.rs:195 | 195 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/style.rs:210 | 210 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/style.rs:225 | 225 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/style.rs:237 | 237 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./src/ui/style.rs:248 | 248 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./temp_canvas2.rs:19 | 19 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./temp_canvas2.rs:25 | 25 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./temp_canvas2.rs:26 | 26 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./temp_canvas2.rs:30 | 30 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./tests/integration_test.rs:6 | 6 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./tests/integration_test.rs:26 | 26 | `::from()` call — verify the conversion is to a different type |
| INFO | style | redundancy | ./tests/integration_test.rs:117 | 117 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./tests/integration_test.rs:125 | 125 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./tests/js_bridge_test.rs:28 | 28 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./tests/js_bridge_test.rs:30 | 30 | Unnecessary `return` at end of block — last expression is already the return value |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:170 | 170 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:177 | 177 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:184 | 184 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:193 | 193 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:202 | 202 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:203 | 203 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:204 | 204 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:238 | 238 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:239 | 239 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:240 | 240 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:250 | 250 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:251 | 251 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:252 | 252 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:262 | 262 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:263 | 263 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:279 | 279 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:279 | 279 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:289 | 289 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:289 | 289 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:291 | 291 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:291 | 291 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:299 | 299 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:299 | 299 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:300 | 300 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:300 | 300 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:301 | 301 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:301 | 301 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:314 | 314 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:314 | 314 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:327 | 327 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:327 | 327 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:328 | 328 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:328 | 328 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:329 | 329 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:329 | 329 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:343 | 343 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:352 | 352 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:352 | 352 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:353 | 353 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:353 | 353 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:354 | 354 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:354 | 354 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:366 | 366 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:366 | 366 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:367 | 367 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:367 | 367 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:377 | 377 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:377 | 377 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:379 | 379 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:379 | 379 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:387 | 387 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:387 | 387 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:389 | 389 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:389 | 389 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:392 | 392 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:392 | 392 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:435 | 435 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:435 | 435 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:436 | 436 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:436 | 436 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:445 | 445 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:445 | 445 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:454 | 454 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:454 | 454 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:455 | 455 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/js_engine_tests.rs:455 | 455 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/korlang_advanced_tests.rs:12 | 12 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./tests/korlang_advanced_tests.rs:15 | 15 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./tests/korlang_advanced_tests.rs:17 | 17 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./tests/korlang_advanced_tests.rs:20 | 20 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| INFO | style | redundancy | ./tests/korlang_advanced_tests.rs:42 | 42 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/korlang_advanced_tests.rs:42 | 42 | .into() call — verify the conversion is necessary (types might be the same) |
| INFO | style | redundancy | ./tests/layout_stress.rs:138 | 138 | Unnecessary block `{ expr }` — use `(expr)` if grouping is needed |
| WARN | memory | edge_cases | ./crates/aether-caelum/src/compute/block.rs:74 | 74 | indexed access at [0] panics on empty collection |
| WARN | memory | edge_cases | ./crates/aether-caelum/src/compute/block.rs:74 | 74 | indexed access at [0] panics on empty collection |
| WARN | memory | edge_cases | ./crates/aether-caelum/src/compute/block.rs:104 | 104 | indexed access at [0] panics on empty collection |
| WARN | memory | edge_cases | ./crates/aether-caelum/src/compute/block.rs:104 | 104 | indexed access at [0] panics on empty collection |
| WARN | memory | edge_cases | ./crates/aether-caelum/src/compute/block.rs:104 | 104 | indexed access at [0] panics on empty collection |
| WARN | memory | edge_cases | ./crates/aether-caelum/src/compute/block.rs:136 | 136 | indexed access at [0] panics on empty collection |
| WARN | memory | edge_cases | ./crates/aether-caelum/src/compute/block.rs:148 | 148 | indexed access at [0] panics on empty collection |
| WARN | correctness | edge_cases | ./crates/aether-caelum/src/compute/common/alignment.rs:75 | 75 | division by runtime value — possible division by zero |
| WARN | correctness | edge_cases | ./crates/aether-caelum/src/compute/common/alignment.rs:82 | 82 | division by runtime value — possible division by zero |
| WARN | correctness | edge_cases | ./crates/aether-caelum/src/compute/common/alignment.rs:97 | 97 | division by runtime value — possible division by zero |
| WARN | correctness | edge_cases | ./crates/aether-caelum/src/compute/common/alignment.rs:98 | 98 | division by runtime value — possible division by zero |
| WARN | correctness | edge_cases | ./crates/aether-caelum/src/compute/common/alignment.rs:99 | 99 | division by runtime value — possible division by zero |
| INFO | security | edge_cases | ./crates/aether-caelum/src/compute/float.rs:78 | 78 | slice indexed by variable `slot` — ensure bounds are validated |
| WARN | memory | edge_cases | ./crates/aether-caelum/src/compute/float.rs:84 | 84 | indexed access at [0] panics on empty collection |
| WARN | memory | edge_cases | ./crates/aether-caelum/src/compute/float.rs:112 | 112 | indexed access at [0] panics on empty collection |
| WARN | memory | edge_cases | ./crates/aether-caelum/src/compute/float.rs:112 | 112 | indexed access at [0] panics on empty collection |
| WARN | memory | edge_cases | ./crates/aether-caelum/src/compute/float.rs:112 | 112 | indexed access at [0] panics on empty collection |
| WARN | memory | edge_cases | ./crates/aether-caelum/src/compute/float.rs:119 | 119 | indexed access at [0] panics on empty collection |
| INFO | security | edge_cases | ./crates/aether-caelum/src/compute/grid/track_sizing.rs:60 | 60 | slice indexed by variable `batch_range` — ensure bounds are validated |
| WARN | correctness | edge_cases | ./crates/aether-caelum/src/compute/grid/types/cell_occupancy.rs:25 | 25 | division by runtime value — possible division by zero |
| WARN | memory | edge_cases | ./crates/aether-caelum/src/tree/cache.rs:40 | 40 | array repeat with runtime-determined count — may overflow the stack |
| INFO | security | edge_cases | ./crates/aether-caelum/src/tree/caelum_tree.rs:191 | 191 | slice indexed by variable `id` — ensure bounds are validated |
| INFO | security | edge_cases | ./korlang/src/compiler/mod.rs:48 | 48 | slice indexed by variable `else_jump_idx` — ensure bounds are validated |
| INFO | security | edge_cases | ./korlang/src/compiler/mod.rs:51 | 51 | slice indexed by variable `end_jump_idx` — ensure bounds are validated |
| INFO | security | edge_cases | ./korlang/src/compiler/mod.rs:63 | 63 | slice indexed by variable `exit_jump_idx` — ensure bounds are validated |
| INFO | security | edge_cases | ./korlang/src/vm/mod.rs:112 | 112 | slice indexed by variable `ip` — ensure bounds are validated |
| WARN | correctness | edge_cases | ./src/engine/pipeline/layout.rs:18 | 18 | division by runtime value — possible division by zero |
| WARN | memory | edge_cases | ./tests/integration_test.rs:14 | 14 | indexed access at [0] panics on empty collection |
| WARN | memory | edge_cases | ./tests/rendering_pipeline_tests.rs:698 | 698 | indexed access at [0] panics on empty collection |
| WARN | memory | edge_cases | ./tests/rendering_pipeline_tests.rs:698 | 698 | indexed access at [0] panics on empty collection |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:363 | 363 | `#[test]` function `explicit_grid_sizing_no_repeats` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:387 | 387 | `#[test]` function `explicit_grid_sizing_auto_fill_exact_fit` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:418 | 418 | `#[test]` function `explicit_grid_sizing_auto_fill_non_exact_fit` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:449 | 449 | `#[test]` function `explicit_grid_sizing_auto_fill_min_size_exact_fit` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:480 | 480 | `#[test]` function `explicit_grid_sizing_auto_fill_min_size_non_exact_fit` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:511 | 511 | `#[test]` function `explicit_grid_sizing_auto_fill_multiple_repeated_tracks` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:542 | 542 | `#[test]` function `explicit_grid_sizing_auto_fill_gap` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:574 | 574 | `#[test]` function `explicit_grid_sizing_no_defined_size` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:605 | 605 | `#[test]` function `explicit_grid_sizing_mix_repeated_and_non_repeated` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:637 | 637 | `#[test]` function `explicit_grid_sizing_mix_with_padding` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:669 | 669 | `#[test]` function `test_initialize_grid_tracks` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:194 | 194 | `#[test]` function `child_min_max_line_auto` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:202 | 202 | `#[test]` function `child_min_max_line_negative_track` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:217 | 217 | `#[test]` function `explicit_grid_sizing_with_children` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:235 | 235 | `#[test]` function `negative_implicit_grid_sizing` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/placement.rs:571 | 571 | `#[test]` function `test_only_fixed_placement` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/placement.rs:590 | 590 | `#[test]` function `test_placement_spanning_origin` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/placement.rs:609 | 609 | `#[test]` function `test_only_auto_placement_row_flow` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/placement.rs:633 | 633 | `#[test]` function `test_only_auto_placement_column_flow` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/placement.rs:657 | 657 | `#[test]` function `test_oversized_item` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/placement.rs:673 | 673 | `#[test]` function `test_fixed_in_secondary_axis` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/placement.rs:692 | 692 | `#[test]` function `test_definite_in_secondary_axis_with_fully_definite_negative` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/placement.rs:710 | 710 | `#[test]` function `test_dense_packing_algorithm` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/placement.rs:728 | 728 | `#[test]` function `test_sparse_packing_algorithm` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/grid/placement.rs:746 | 746 | `#[test]` function `test_auto_placement_in_negative_tracks` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/compute/mod.rs:291 | 291 | `#[test]` function `hidden_layout_should_hide_recursively` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:73 | 73 | `#[test]` function `flexbox_items_larger_than_container` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:87 | 87 | `#[test]` function `flexbox_zero_size_items` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:101 | 101 | `#[test]` function `flexbox_nested_containers` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:117 | 117 | `#[test]` function `flexbox_wrap_with_min_max` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:140 | 140 | `#[test]` function `grid_auto_placement` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:159 | 159 | `#[test]` function `grid_item_spanning_multiple_tracks` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:180 | 180 | `#[test]` function `block_float_left` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:194 | 194 | `#[test]` function `block_clear_both` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:212 | 212 | `#[test]` function `zero_height_container` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:224 | 224 | `#[test]` function `deeply_nested_layout` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:242 | 242 | `#[test]` function `aspect_ratio_sizing` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:255 | 255 | `#[test]` function `overflow_hidden` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:271 | 271 | `#[test]` function `from_css_width_height` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:281 | 281 | `#[test]` function `from_css_display` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:290 | 290 | `#[test]` function `from_css_position` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:296 | 296 | `#[test]` function `from_css_margin_padding` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:310 | 310 | `#[test]` function `from_css_unknown_property` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:317 | 317 | `#[test]` function `flexbox_wrap_items` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:336 | 336 | `#[test]` function `flexbox_nowrap_shrink` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:354 | 354 | `#[test]` function `flexbox_align_items_stretch` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:374 | 374 | `#[test]` function `flexbox_align_items_center` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:391 | 391 | `#[test]` function `flexbox_justify_space_between` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:409 | 409 | `#[test]` function `flexbox_grow_ratio` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:433 | 433 | `#[test]` function `flexbox_column_direction` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:452 | 452 | `#[test]` function `flexbox_gap_between_items` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:477 | 477 | `#[test]` function `grid_explicit_fr_columns` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:501 | 501 | `#[test]` function `grid_column_gap` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:525 | 525 | `#[test]` function `grid_auto_rows` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:547 | 547 | `#[test]` function `grid_fixed_px_columns` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:573 | 573 | `#[test]` function `grid_explicit_row_height` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:600 | 600 | `#[test]` function `block_percentage_width` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:616 | 616 | `#[test]` function `block_auto_height_expands` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:634 | 634 | `#[test]` function `block_margin_top_pushes_down` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:652 | 652 | `#[test]` function `block_negative_margin_overlap` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:673 | 673 | `#[test]` function `block_absolute_positioning` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:693 | 693 | `#[test]` function `float_left_two_items` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:713 | 713 | `#[test]` function `float_right_position` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:727 | 727 | `#[test]` function `float_clear_left_block` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:745 | 745 | `#[test]` function `min_height_overrides` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:762 | 762 | `#[test]` function `display_none_child_takes_no_space` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:779 | 779 | `#[test]` function `max_width_constrains` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/lib.rs:796 | 796 | `#[test]` function `percent_sized_flex_item` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/style/flex.rs:175 | 175 | `#[test]` function `flex_direction_is_row` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/style/flex.rs:183 | 183 | `#[test]` function `flex_direction_is_column` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/style/flex.rs:191 | 191 | `#[test]` function `flex_direction_is_reverse` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/style/mod.rs:1162 | 1162 | `#[test]` function `defaults_match` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/style/mod.rs:1219 | 1219 | `#[test]` function `style_sizes` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/style_helpers.rs:534 | 534 | `#[test]` function `test_repeat_u16` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/style_helpers.rs:546 | 546 | `#[test]` function `test_repeat_auto_fit_str` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/style_helpers.rs:558 | 558 | `#[test]` function `test_repeat_auto_fill_str` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:933 | 933 | `#[test]` function `new_should_allocate_default_capacity` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:943 | 943 | `#[test]` function `test_with_capacity` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:953 | 953 | `#[test]` function `test_new_leaf` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:965 | 965 | `#[test]` function `new_leaf_with_context` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:977 | 977 | `#[test]` function `test_new_with_children` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:991 | 991 | `#[test]` function `remove_node_should_remove` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1000 | 1000 | `#[test]` function `remove_node_should_detach_hierarchy` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1021 | 1021 | `#[test]` function `remove_last_node` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1033 | 1033 | `#[test]` function `set_measure` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1045 | 1045 | `#[test]` function `set_measure_of_previously_unmeasured_node` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1057 | 1057 | `#[test]` function `add_child` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1073 | 1073 | `#[test]` function `insert_child_at_index` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1100 | 1100 | `#[test]` function `set_children` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1121 | 1121 | `#[test]` function `remove_child` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1139 | 1139 | `#[test]` function `remove_child_at_index` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1156 | 1156 | `#[test]` function `remove_children_range` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1179 | 1179 | `#[test]` function `remove_child_updates_parents` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1194 | 1194 | `#[test]` function `replace_child_at_index` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1209 | 1209 | `#[test]` function `test_child_at_index` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1221 | 1221 | `#[test]` function `test_child_count` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1233 | 1233 | `#[test]` function `test_children` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1250 | 1250 | `#[test]` function `test_set_style` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1260 | 1260 | `#[test]` function `test_style` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1272 | 1272 | `#[test]` function `test_layout` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1282 | 1282 | `#[test]` function `test_mark_dirty` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1307 | 1307 | `#[test]` function `compute_layout_should_produce_valid_result` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1323 | 1323 | `#[test]` function `make_sure_layout_location_is_top_left` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/tree/caelum_tree.rs:1367 | 1367 | `#[test]` function `set_children_reparents` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/math.rs:254 | 254 | `#[test]` function `test_maybe_min` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/math.rs:263 | 263 | `#[test]` function `test_maybe_max` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/math.rs:272 | 272 | `#[test]` function `test_maybe_add` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/math.rs:281 | 281 | `#[test]` function `test_maybe_sub` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/math.rs:294 | 294 | `#[test]` function `test_maybe_min` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/math.rs:301 | 301 | `#[test]` function `test_maybe_max` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/math.rs:308 | 308 | `#[test]` function `test_maybe_add` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/math.rs:315 | 315 | `#[test]` function `test_maybe_sub` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/math.rs:326 | 326 | `#[test]` function `test_maybe_min` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/math.rs:333 | 333 | `#[test]` function `test_maybe_max` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/math.rs:340 | 340 | `#[test]` function `test_maybe_add` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/math.rs:347 | 347 | `#[test]` function `test_maybe_sub` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:178 | 178 | `#[test]` function `resolve_auto` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:189 | 189 | `#[test]` function `resolve_length` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:201 | 201 | `#[test]` function `resolve_percent` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:220 | 220 | `#[test]` function `maybe_resolve_auto` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:231 | 231 | `#[test]` function `maybe_resolve_length` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:243 | 243 | `#[test]` function `maybe_resolve_percent` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:262 | 262 | `#[test]` function `resolve_or_zero_auto` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:269 | 269 | `#[test]` function `resolve_or_zero_length` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:276 | 276 | `#[test]` function `resolve_or_zero_percent` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:290 | 290 | `#[test]` function `resolve_or_zero_auto` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:298 | 298 | `#[test]` function `resolve_or_zero_length` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:306 | 306 | `#[test]` function `resolve_or_zero_percent` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:324 | 324 | `#[test]` function `resolve_or_zero_auto` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:332 | 332 | `#[test]` function `resolve_or_zero_length` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-caelum/src/util/resolve.rs:340 | 340 | `#[test]` function `resolve_or_zero_percent` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/lib.rs:21 | 21 | `#[test]` function `test_full_pipeline` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/lib.rs:41 | 41 | `#[test]` function `test_empty_css` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/matcher.rs:144 | 144 | `#[test]` function `test_tag_matching` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/matcher.rs:154 | 154 | `#[test]` function `test_class_matching` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/matcher.rs:164 | 164 | `#[test]` function `test_id_matching` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/matcher.rs:174 | 174 | `#[test]` function `test_no_match` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/matcher.rs:184 | 184 | `#[test]` function `test_specificity` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:556 | 556 | `#[test]` function `test_parse_simple_rule` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:564 | 564 | `#[test]` function `test_parse_multiple_rules` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:571 | 571 | `#[test]` function `test_parse_class_selector` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:581 | 581 | `#[test]` function `test_parse_id_selector` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:591 | 591 | `#[test]` function `test_parse_multiple_with_id` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:598 | 598 | `#[test]` function `test_parse_empty_input` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:604 | 604 | `#[test]` function `test_parse_hsl_color` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:612 | 612 | `#[test]` function `test_parse_hsla_color` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:620 | 620 | `#[test]` function `test_parse_color_mix_stub` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:628 | 628 | `#[test]` function `test_parse_color_mix_with_hex` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:636 | 636 | `#[test]` function `test_parse_rgb_color` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:644 | 644 | `#[test]` function `test_parse_rgba_color` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:652 | 652 | `#[test]` function `test_parse_rgba_alpha_one` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:660 | 660 | `#[test]` function `test_parse_rgb_spaces_around_commas` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:668 | 668 | `#[test]` function `test_parse_rgb_percentage` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/parser.rs:676 | 676 | `#[test]` function `test_parse_hsla_color_with_alpha` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/resolver.rs:444 | 444 | `#[test]` function `test_resolve_simple` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/resolver.rs:454 | 454 | `#[test]` function `test_resolve_display` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/resolver.rs:464 | 464 | `#[test]` function `test_resolve_flex` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/resolver.rs:476 | 476 | `#[test]` function `test_cascade_override` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/resolver.rs:486 | 486 | `#[test]` function `test_specificity_override` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/resolver.rs:498 | 498 | `#[test]` function `test_resolve_current_color` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/resolver.rs:507 | 507 | `#[test]` function `test_resolve_hsl_color` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/resolver.rs:516 | 516 | `#[test]` function `test_resolve_color_mix_stub` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/resolver.rs:525 | 525 | `#[test]` function `test_resolve_border_current_color` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/resolver.rs:534 | 534 | `#[test]` function `test_resolve_rgb_color` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/resolver.rs:543 | 543 | `#[test]` function `test_resolve_rgba_color` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/resolver.rs:552 | 552 | `#[test]` function `test_resolve_hsla_color` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/style_value.rs:536 | 536 | `#[test]` function `test_color_from_hex` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/style_value.rs:543 | 543 | `#[test]` function `test_color_from_named` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/style_value.rs:549 | 549 | `#[test]` function `test_color_current_color` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/style_value.rs:556 | 556 | `#[test]` function `test_unit_parsing` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/style_value.rs:563 | 563 | `#[test]` function `test_display_from_str` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-css/src/style_value.rs:570 | 570 | `#[test]` function `test_computed_style_default` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-dom/src/lib.rs:109 | 109 | `#[test]` function `test_node_creation` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-dom/src/lib.rs:115 | 115 | `#[test]` function `test_element_tag_name` contains no assertions |
| WARN | correctness | assertion_checks | ./crates/aether-html/src/lib.rs:240 | 240 | `#[test]` function `test_parse_simple_element` contains no assertions |
| WARN | correctness | assertion_checks | ./korlang/src/lib.rs:27 | 27 | `#[test]` function `test_arithmetic` contains no assertions |
| WARN | correctness | assertion_checks | ./korlang/src/lib.rs:42 | 42 | `#[test]` function `test_functions` contains no assertions |
| WARN | correctness | assertion_checks | ./korlang/src/lib.rs:57 | 57 | `#[test]` function `test_list_iteration` contains no assertions |
| WARN | correctness | assertion_checks | ./korlang/src/lib.rs:69 | 69 | `#[test]` function `test_formatter_roundtrip` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:859 | 859 | `#[test]` function `test_decode_amp` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:861 | 861 | `#[test]` function `test_decode_lt` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:863 | 863 | `#[test]` function `test_decode_gt` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:865 | 865 | `#[test]` function `test_decode_quot` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:867 | 867 | `#[test]` function `test_decode_apos` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:869 | 869 | `#[test]` function `test_decode_decimal` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:871 | 871 | `#[test]` function `test_decode_hex_emoji` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:873 | 873 | `#[test]` function `test_decode_no_nested` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:875 | 875 | `#[test]` function `test_decode_no_entities` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:877 | 877 | `#[test]` function `test_decode_unknown` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:879 | 879 | `#[test]` function `test_decode_mixed` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:881 | 881 | `#[test]` function `test_decode_nbsp` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:883 | 883 | `#[test]` function `test_decode_copy` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:885 | 885 | `#[test]` function `test_decode_reg` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:888 | 888 | `#[test]` function `test_extract_decodes_text` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:900 | 900 | `#[test]` function `test_extract_decodes_anchor_href` contains no assertions |
| WARN | correctness | assertion_checks | ./src/engine/pipeline/extractor.rs:912 | 912 | `#[test]` function `test_extract_decodes_img_alt` contains no assertions |
| WARN | correctness | assertion_checks | ./src/ui/screens/browser.rs:1042 | 1042 | `#[test]` function `test_ifc_simple_inline_siblings` contains no assertions |
| WARN | correctness | assertion_checks | ./src/ui/screens/browser.rs:1054 | 1054 | `#[test]` function `test_ifc_single_inline_in_block` contains no assertions |
| WARN | correctness | assertion_checks | ./src/ui/screens/browser.rs:1065 | 1065 | `#[test]` function `test_ifc_inline_wraps_when_exceeds_container` contains no assertions |
| WARN | correctness | assertion_checks | ./src/ui/screens/browser.rs:1076 | 1076 | `#[test]` function `test_ifc_mixed_inline_and_block` contains no assertions |
| WARN | correctness | assertion_checks | ./src/ui/screens/browser.rs:1088 | 1088 | `#[test]` function `test_ifc_nested_inline` contains no assertions |
| WARN | correctness | assertion_checks | ./src/ui/screens/browser.rs:1099 | 1099 | `#[test]` function `test_ifc_inline_block_margin_contrib` contains no assertions |
| WARN | correctness | assertion_checks | ./src/ui/screens/browser.rs:1114 | 1114 | `#[test]` function `test_stratus_roundtrip` contains no assertions |
| WARN | correctness | assertion_checks | ./src/ui/screens/browser.rs:1132 | 1132 | `#[test]` function `test_js_bridge_init` contains no assertions |
| WARN | correctness | assertion_checks | ./src/ui/screens/browser.rs:1140 | 1140 | `#[test]` function `test_nav_url_normalization` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:25 | 25 | `#[test]` function `test_color_hex` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:31 | 31 | `#[test]` function `test_color_named` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:37 | 37 | `#[test]` function `test_color_rgb` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:43 | 43 | `#[test]` function `test_color_rgba` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:49 | 49 | `#[test]` function `test_color_hsl` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:57 | 57 | `#[test]` function `test_background_color` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:65 | 65 | `#[test]` function `test_font_size` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:73 | 73 | `#[test]` function `test_font_weight` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:81 | 81 | `#[test]` function `test_display_none` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:87 | 87 | `#[test]` function `test_display_flex` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:93 | 93 | `#[test]` function `test_display_inline_block` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:101 | 101 | `#[test]` function `test_margin_all` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:112 | 112 | `#[test]` function `test_padding_two_values` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:123 | 123 | `#[test]` function `test_border_width_and_color` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:138 | 138 | `#[test]` function `test_width_height` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:148 | 148 | `#[test]` function `test_flex_properties` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:159 | 159 | `#[test]` function `test_flex_grow` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:165 | 165 | `#[test]` function `test_flex_shrink` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:173 | 173 | `#[test]` function `test_line_height` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:179 | 179 | `#[test]` function `test_z_index` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:185 | 185 | `#[test]` function `test_opacity` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:191 | 191 | `#[test]` function `test_opacity_clamp_high` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:197 | 197 | `#[test]` function `test_opacity_clamp_low` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:205 | 205 | `#[test]` function `test_text_decoration` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:213 | 213 | `#[test]` function `test_class_selector` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:225 | 225 | `#[test]` function `test_id_selector` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:237 | 237 | `#[test]` function `test_multiple_declarations` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:253 | 253 | `#[test]` function `test_invalid_property_no_crash` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/css_regression.rs:263 | 263 | `#[test]` function `test_cascade_order` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/html5_compliance.rs:5 | 5 | `#[test]` function `test_should_skip_tag` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/integration_test.rs:4 | 4 | `#[test]` function `test_parsing_div_with_paragraph` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/integration_test.rs:24 | 24 | `#[test]` function `test_parsing_multiple_elements` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/integration_test.rs:40 | 40 | `#[test]` function `test_should_skip_tag_filters` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/integration_test.rs:50 | 50 | `#[test]` function `test_extract_and_layout_pipeline` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/integration_test.rs:104 | 104 | `#[test]` function `test_inner_html_strips_script_tags` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/integration_test.rs:132 | 132 | `#[test]` function `test_set_attribute_rejects_event_handlers` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/integration_test.rs:155 | 155 | `#[test]` function `test_set_attribute_rejects_srcdoc` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:35 | 35 | `#[test]` function `test_create_element` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:42 | 42 | `#[test]` function `test_append_child` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:53 | 53 | `#[test]` function `test_text_node` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:61 | 61 | `#[test]` function `test_set_attribute` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:69 | 69 | `#[test]` function `test_get_element_by_id` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:80 | 80 | `#[test]` function `test_query_selector` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:90 | 90 | `#[test]` function `test_query_selector_all` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:106 | 106 | `#[test]` function `test_set_text_content` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:114 | 114 | `#[test]` function `test_inner_html` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:125 | 125 | `#[test]` function `test_style_property` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:134 | 134 | `#[test]` function `test_timeout` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:144 | 144 | `#[test]` function `test_clear_timer` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:153 | 153 | `#[test]` function `test_interval` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:166 | 166 | `#[test]` function `test_event_listener` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:177 | 177 | `#[test]` function `test_event_listener_bubbling` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:189 | 189 | `#[test]` function `test_fetch_url_cross_origin` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:198 | 198 | `#[test]` function `test_element_traversal` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:228 | 228 | `#[test]` function `test_sibling_traversal` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:255 | 255 | `#[test]` function `test_child_nodes_includes_text` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:267 | 267 | `#[test]` function `test_dom_roundtrip` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:281 | 281 | `#[test]` function `test_load_dom_with_document_root` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:289 | 289 | `#[test]` function `test_dom_roundtrip_preserves_structure` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:305 | 305 | `#[test]` function `test_load_dom_with_body` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:320 | 320 | `#[test]` function `test_get_tag_name_variants` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:331 | 331 | `#[test]` function `test_set_and_get_cookie` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:341 | 341 | `#[test]` function `test_local_storage` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:363 | 363 | `#[test]` function `test_location_parts` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:376 | 376 | `#[test]` function `test_location_parts_defaults` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:388 | 388 | `#[test]` function `test_location_reload` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:395 | 395 | `#[test]` function `test_location_assign` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:402 | 402 | `#[test]` function `test_location_replace` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:409 | 409 | `#[test]` function `test_set_location_href` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:416 | 416 | `#[test]` function `test_document_write_and_take_output` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:425 | 425 | `#[test]` function `test_doc_title` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:433 | 433 | `#[test]` function `test_pending_timers` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:443 | 443 | `#[test]` function `test_query_selector_by_tag` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:456 | 456 | `#[test]` function `test_query_selector_by_id` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:466 | 466 | `#[test]` function `test_query_selector_by_class` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:478 | 478 | `#[test]` function `test_query_selector_universal` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:487 | 487 | `#[test]` function `test_query_selector_child_combinator` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:499 | 499 | `#[test]` function `test_query_selector_descendant_combinator` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:515 | 515 | `#[test]` function `test_query_selector_compound_descendant` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:527 | 527 | `#[test]` function `test_event_listener_remove` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:537 | 537 | `#[test]` function `test_get_event_listeners` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:550 | 550 | `#[test]` function `test_self_append_child_noop` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:558 | 558 | `#[test]` function `test_set_text_content_on_text_node_noop` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:566 | 566 | `#[test]` function `test_remove_event_listener_partial_match` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:575 | 575 | `#[test]` function `test_inner_html_self_closing` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:587 | 587 | `#[test]` function `test_inner_html_nested` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:601 | 601 | `#[test]` function `test_set_attribute_on_text_node_noop` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:609 | 609 | `#[test]` function `test_element_at_point_no_elements` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:616 | 616 | `#[test]` function `test_get_children_excludes_text` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:629 | 629 | `#[test]` function `test_get_child_nodes_includes_all` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_bridge_test.rs:641 | 641 | `#[test]` function `test_load_dom_idempotent` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:21 | 21 | `#[test]` function `test_query_selector_by_tag` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:29 | 29 | `#[test]` function `test_query_selector_by_id` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:36 | 36 | `#[test]` function `test_query_selector_by_class` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:43 | 43 | `#[test]` function `test_query_selector_descendant` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:58 | 58 | `#[test]` function `test_query_selector_child` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:80 | 80 | `#[test]` function `test_query_selector_all` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:98 | 98 | `#[test]` function `test_query_selector_no_match` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:106 | 106 | `#[test]` function `test_query_selector_nested` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:130 | 130 | `#[test]` function `test_query_selector_complex` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:148 | 148 | `#[test]` function `test_query_selector_wildcard` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:167 | 167 | `#[test]` function `test_set_timeout_adds_entry` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:174 | 174 | `#[test]` function `test_set_interval_adds_entry` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:181 | 181 | `#[test]` function `test_clear_timeout_removes_entry` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:190 | 190 | `#[test]` function `test_clear_interval_removes_entry` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:199 | 199 | `#[test]` function `test_timer_id_increments` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:213 | 213 | `#[test]` function `test_timeout_callback_is_source` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:225 | 225 | `#[test]` function `test_interval_callback_is_source` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:234 | 234 | `#[test]` function `test_pending_timers_count` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:247 | 247 | `#[test]` function `test_clear_all_timers` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:259 | 259 | `#[test]` function `test_timer_after_page_load` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:275 | 275 | `#[test]` function `test_add_event_listener` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:285 | 285 | `#[test]` function `test_remove_event_listener` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:295 | 295 | `#[test]` function `test_event_listener_id_increments` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:308 | 308 | `#[test]` function `test_click_event_dispatch` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:319 | 319 | `#[test]` function `test_event_bubbling` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:338 | 338 | `#[test]` function `test_event_listener_source_string` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:348 | 348 | `#[test]` function `test_multiple_listeners_same_event` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:362 | 362 | `#[test]` function `test_get_event_listeners` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:373 | 373 | `#[test]` function `test_remove_partial_match` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:383 | 383 | `#[test]` function `test_event_after_remove` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:404 | 404 | `#[test]` function `test_fetch_same_origin` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:414 | 414 | `#[test]` function `test_fetch_cross_origin_cors` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:423 | 423 | `#[test]` function `test_fetch_returns_status_prefix` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:432 | 432 | `#[test]` function `test_local_storage_set_get` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:442 | 442 | `#[test]` function `test_local_storage_remove` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:451 | 451 | `#[test]` function `test_local_storage_clear` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:462 | 462 | `#[test]` function `test_cookie_set_get` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:472 | 472 | `#[test]` function `test_cookie_expires` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:481 | 481 | `#[test]` function `test_fetch_error_handling` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/js_engine_tests.rs:491 | 491 | `#[test]` function `test_fetch_redirect` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:9 | 9 | `#[test]` function `test_nested_function_calls` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:38 | 38 | `#[test]` function `test_closure_capture` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:53 | 53 | `#[test]` function `test_native_print` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:77 | 77 | `#[test]` function `test_native_chrome_render` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:101 | 101 | `#[test]` function `test_interpolate_multiple_vars` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:118 | 118 | `#[test]` function `test_for_each_empty_array` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:134 | 134 | `#[test]` function `test_for_each_single_item` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:150 | 150 | `#[test]` function `test_jump_if_false_none` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:168 | 168 | `#[test]` function `test_jump_if_false_zero` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:185 | 185 | `#[test]` function `test_jump_if_false_empty_string` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:202 | 202 | `#[test]` function `test_dup_preserves_value` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:218 | 218 | `#[test]` function `test_pop_removes_value` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:234 | 234 | `#[test]` function `test_store_load_roundtrip` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:250 | 250 | `#[test]` function `test_create_element_sets_tag` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:268 | 268 | `#[test]` function `test_set_property_adds_attr` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:291 | 291 | `#[test]` function `test_add_child_nests_elements` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:316 | 316 | `#[test]` function `test_deep_element_tree` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:357 | 357 | `#[test]` function `test_multi_arg_native_call` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:377 | 377 | `#[test]` function `test_for_each_with_jump_back` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/korlang_advanced_tests.rs:396 | 396 | `#[test]` function `test_empty_bytecode` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:70 | 70 | `#[test]` function `single_block_element` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:79 | 79 | `#[test]` function `two_blocks_parent_child` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:91 | 91 | `#[test]` function `thousand_flat_elements` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:101 | 101 | `#[test]` function `thousand_inline_siblings` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:114 | 114 | `#[test]` function `deep_nesting_50` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:124 | 124 | `#[test]` function `deep_nesting_100` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:134 | 134 | `#[test]` function `mixed_inline_block` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:150 | 150 | `#[test]` function `large_text_elements` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:164 | 164 | `#[test]` function `all_display_types` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:183 | 183 | `#[test]` function `margins_affect_layout` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:204 | 204 | `#[test]` function `padding_contains_children` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:227 | 227 | `#[test]` function `borders_no_crash` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:249 | 249 | `#[test]` function `empty_slice_no_panic` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/layout_stress.rs:255 | 255 | `#[test]` function `wide_container` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering.rs:5 | 5 | `#[test]` function `test_basic_rendering_pipeline` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering.rs:28 | 28 | `#[test]` function `test_caelum_spatial_init` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:56 | 56 | `#[test]` function `test_css_parse_color_hex` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:62 | 62 | `#[test]` function `test_css_parse_color_named` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:68 | 68 | `#[test]` function `test_css_parse_margin_shorthand` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:77 | 77 | `#[test]` function `test_css_parse_padding_shorthand` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:86 | 86 | `#[test]` function `test_css_parse_display_block` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:92 | 92 | `#[test]` function `test_css_parse_display_flex` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:98 | 98 | `#[test]` function `test_css_parse_display_grid` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:104 | 104 | `#[test]` function `test_css_parse_font_size_px` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:110 | 110 | `#[test]` function `test_css_parse_border_width` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:118 | 118 | `#[test]` function `test_css_parse_multiple_rules` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:129 | 129 | `#[test]` function `test_computed_style_inline_to_block_for_block_tags` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:135 | 135 | `#[test]` function `test_computed_style_class_overrides_tag` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:142 | 142 | `#[test]` function `test_computed_style_id_overrides_class` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:149 | 149 | `#[test]` function `test_computed_style_flex_direction` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:155 | 155 | `#[test]` function `test_computed_style_justify_content` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:165 | 165 | `#[test]` function `test_skip_tag_script` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:170 | 170 | `#[test]` function `test_skip_tag_style` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:175 | 175 | `#[test]` function `test_skip_tag_head` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:180 | 180 | `#[test]` function `test_skip_tag_meta` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:185 | 185 | `#[test]` function `test_skip_tag_noscript` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:190 | 190 | `#[test]` function `test_skip_tag_svg` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:195 | 195 | `#[test]` function `test_skip_tag_template` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:200 | 200 | `#[test]` function `test_skip_content_script` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:205 | 205 | `#[test]` function `test_skip_content_style` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:210 | 210 | `#[test]` function `test_no_skip_div` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:215 | 215 | `#[test]` function `test_no_skip_p` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:220 | 220 | `#[test]` function `test_no_skip_img` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:225 | 225 | `#[test]` function `test_no_skip_a` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:230 | 230 | `#[test]` function `test_extract_elements_from_simple_html` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:241 | 241 | `#[test]` function `test_extract_elements_no_script_content` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:252 | 252 | `#[test]` function `test_extract_elements_script_content_skipped` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:264 | 264 | `#[test]` function `test_extract_elements_head_content_hidden` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:280 | 280 | `#[test]` function `test_block_elements_stack_vertically` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:291 | 291 | `#[test]` function `test_block_elements_have_width` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:301 | 301 | `#[test]` function `test_block_elements_have_height` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:311 | 311 | `#[test]` function `test_nested_block_elements` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:322 | 322 | `#[test]` function `test_multiple_siblings_stacked` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:335 | 335 | `#[test]` function `test_block_with_margin_top` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:351 | 351 | `#[test]` function `test_inline_siblings_flow_horizontally` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:362 | 362 | `#[test]` function `test_inline_in_block` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:373 | 373 | `#[test]` function `test_inline_wraps_when_long` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:383 | 383 | `#[test]` function `test_inline_mixed_with_block` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:397 | 397 | `#[test]` function `test_inline_block_element` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:410 | 410 | `#[test]` function `test_multiple_inline_spans` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:427 | 427 | `#[test]` function `test_flex_row_direction` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:443 | 443 | `#[test]` function `test_flex_column_direction` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:459 | 459 | `#[test]` function `test_flex_justify_center` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:474 | 474 | `#[test]` function `test_flex_align_items_center` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:489 | 489 | `#[test]` function `test_flex_wrap_nowrap` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:508 | 508 | `#[test]` function `test_flex_grow` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:531 | 531 | `#[test]` function `test_grid_display` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:537 | 537 | `#[test]` function `test_grid_children_in_grid_container` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:554 | 554 | `#[test]` function `test_grid_single_column` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:571 | 571 | `#[test]` function `test_grid_item_sizes` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:586 | 586 | `#[test]` function `test_grid_empty_container` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:603 | 603 | `#[test]` function `test_float_left_element` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:618 | 618 | `#[test]` function `test_float_right_element` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:634 | 634 | `#[test]` function `test_float_does_not_affect_siblings_positioning` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:645 | 645 | `#[test]` function `test_clear_both` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:655 | 655 | `#[test]` function `test_multiple_floats` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:678 | 678 | `#[test]` function `test_margin_top_on_first_element` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:690 | 690 | `#[test]` function `test_margin_bottom_spacing` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:702 | 702 | `#[test]` function `test_zero_margins` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:712 | 712 | `#[test]` function `test_large_margin_top` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:724 | 724 | `#[test]` function `test_margins_on_nested_elements` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:741 | 741 | `#[test]` function `test_border_widths_applied` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:753 | 753 | `#[test]` function `test_padding_affects_size` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:764 | 764 | `#[test]` function `test_border_color_set` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:774 | 774 | `#[test]` function `test_no_border_no_padding` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:784 | 784 | `#[test]` function `test_element_positioning_with_padding` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:799 | 799 | `#[test]` function `test_empty_elements_vec` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:806 | 806 | `#[test]` function `test_single_element` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:813 | 813 | `#[test]` function `test_display_none_elements_ignored` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:821 | 821 | `#[test]` function `test_very_narrow_container` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:831 | 831 | `#[test]` function `test_very_wide_container` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:841 | 841 | `#[test]` function `test_long_text_wrapping` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:851 | 851 | `#[test]` function `test_elements_with_image_dimensions` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:862 | 862 | `#[test]` function `test_link_element_preserves_href` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:873 | 873 | `#[test]` function `test_multiple_parent_levels` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:885 | 885 | `#[test]` function `test_font_size_affects_height` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:900 | 900 | `#[test]` function `test_decode_amp` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:905 | 905 | `#[test]` function `test_decode_lt` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:910 | 910 | `#[test]` function `test_decode_gt` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:915 | 915 | `#[test]` function `test_decode_quot` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:920 | 920 | `#[test]` function `test_decode_apos` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:925 | 925 | `#[test]` function `test_decode_nbsp` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:930 | 930 | `#[test]` function `test_decode_decimal` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:935 | 935 | `#[test]` function `test_decode_hex_emoji` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:940 | 940 | `#[test]` function `test_decode_no_nested_recursion` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:945 | 945 | `#[test]` function `test_decode_no_entities_preserved` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:950 | 950 | `#[test]` function `test_decode_preserves_unknown_entity` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:955 | 955 | `#[test]` function `test_decode_mixed_text` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:960 | 960 | `#[test]` function `test_decode_in_extracted_text` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:972 | 972 | `#[test]` function `test_decode_href_attribute` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/rendering_pipeline_tests.rs:984 | 984 | `#[test]` function `test_decode_alt_attribute` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:71 | 71 | `#[test]` function `test_tab_struct_construction` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:78 | 78 | `#[test]` function `test_tab_empty_title` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:85 | 85 | `#[test]` function `test_tab_clone` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:93 | 93 | `#[test]` function `test_tab_serialization_roundtrip` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:110 | 110 | `#[test]` function `test_normalize_nav_url_https` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:115 | 115 | `#[test]` function `test_normalize_nav_url_http` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:120 | 120 | `#[test]` function `test_normalize_nav_url_bare_domain` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:125 | 125 | `#[test]` function `test_normalize_nav_url_double_slash` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:130 | 130 | `#[test]` function `test_normalize_nav_url_aether_protocol` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:135 | 135 | `#[test]` function `test_normalize_nav_url_about_blank` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:140 | 140 | `#[test]` function `test_normalize_nav_url_empty` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:145 | 145 | `#[test]` function `test_normalize_nav_url_whitespace_only` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:150 | 150 | `#[test]` function `test_normalize_nav_url_strips_whitespace` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:155 | 155 | `#[test]` function `test_normalize_url_plain` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:160 | 160 | `#[test]` function `test_normalize_url_with_path` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:165 | 165 | `#[test]` function `test_normalize_url_already_has_scheme` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:170 | 170 | `#[test]` function `test_normalize_url_double_slash_strips_extra_slash` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:181 | 181 | `#[test]` function `test_is_url_with_scheme` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:187 | 187 | `#[test]` function `test_is_url_with_dot` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:193 | 193 | `#[test]` function `test_is_url_aether_protocol` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:198 | 198 | `#[test]` function `test_is_url_about_protocol` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:203 | 203 | `#[test]` function `test_is_url_plain_search_query` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:209 | 209 | `#[test]` function `test_search_url_duckduckgo` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:217 | 217 | `#[test]` function `test_search_url_google` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:225 | 225 | `#[test]` function `test_search_url_special_chars` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:237 | 237 | `#[test]` function `test_settings_defaults` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:246 | 246 | `#[test]` function `test_settings_serialization_roundtrip` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:262 | 262 | `#[test]` function `test_settings_load_nonexistent_file_returns_defaults` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:270 | 270 | `#[test]` function `test_settings_save_and_load` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:289 | 289 | `#[test]` function `test_settings_toggle_js` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:299 | 299 | `#[test]` function `test_settings_toggle_cookies` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:311 | 311 | `#[test]` function `test_history_initial_state` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:319 | 319 | `#[test]` function `test_history_push` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:329 | 329 | `#[test]` function `test_history_back` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:341 | 341 | `#[test]` function `test_history_forward` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:354 | 354 | `#[test]` function `test_history_cannot_go_back_from_start` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:362 | 362 | `#[test]` function `test_history_cannot_go_forward_from_end` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:372 | 372 | `#[test]` function `test_history_truncate_on_new_navigate` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:388 | 388 | `#[test]` function `test_history_limit_many_entries` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:404 | 404 | `#[test]` function `test_autocomplete_filter_exact_prefix` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:414 | 414 | `#[test]` function `test_autocomplete_filter_no_matches` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:423 | 423 | `#[test]` function `test_autocomplete_filter_case_insensitive_contains` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:432 | 432 | `#[test]` function `test_autocomplete_filter_limit_results` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:441 | 441 | `#[test]` function `test_autocomplete_empty_input_shows_nothing` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:452 | 452 | `#[test]` function `test_styled_element_construction` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:460 | 460 | `#[test]` function `test_styled_element_default_values` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:471 | 471 | `#[test]` function `test_styled_element_link` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:480 | 480 | `#[test]` function `test_styled_element_with_background` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:489 | 489 | `#[test]` function `test_styled_element_with_border` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:498 | 498 | `#[test]` function `test_styled_element_with_margin` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:507 | 507 | `#[test]` function `test_styled_element_with_image` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:515 | 515 | `#[test]` function `test_styled_element_clone` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:527 | 527 | `#[test]` function `test_layout_single_block_element` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:536 | 536 | `#[test]` function `test_layout_two_block_elements_stacked` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:547 | 547 | `#[test]` function `test_layout_inline_elements_side_by_side` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:560 | 560 | `#[test]` function `test_layout_hidden_element` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:570 | 570 | `#[test]` function `test_layout_with_margin` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:582 | 582 | `#[test]` function `test_layout_with_border` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:594 | 594 | `#[test]` function `test_skip_tag_script` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:599 | 599 | `#[test]` function `test_skip_tag_style` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:604 | 604 | `#[test]` function `test_skip_tag_noscript` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:609 | 609 | `#[test]` function `test_skip_tag_meta` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:614 | 614 | `#[test]` function `test_skip_tag_link` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:619 | 619 | `#[test]` function `test_skip_tag_head` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:624 | 624 | `#[test]` function `test_skip_tag_svg` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:629 | 629 | `#[test]` function `test_skip_tag_template` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:634 | 634 | `#[test]` function `test_no_skip_tag_div` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:639 | 639 | `#[test]` function `test_no_skip_tag_p` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:644 | 644 | `#[test]` function `test_no_skip_tag_img` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:649 | 649 | `#[test]` function `test_no_skip_tag_a` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:654 | 654 | `#[test]` function `test_no_skip_tag_span` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:659 | 659 | `#[test]` function `test_skip_content_script` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:664 | 664 | `#[test]` function `test_skip_content_style` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:669 | 669 | `#[test]` function `test_no_skip_content_div` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:674 | 674 | `#[test]` function `test_no_skip_content_p` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:683 | 683 | `#[test]` function `test_save_tabs_empty` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:694 | 694 | `#[test]` function `test_save_tabs_multiple` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:714 | 714 | `#[test]` function `test_sidebar_workspace_labels` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:723 | 723 | `#[test]` function `test_sidebar_collection_labels` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:730 | 730 | `#[test]` function `test_sidebar_section_headers` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:741 | 741 | `#[test]` function `test_devtools_tab_variants` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:752 | 752 | `#[test]` function `test_devtools_tab_default_is_console` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:763 | 763 | `#[test]` function `test_normalize_nav_url_with_port` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:768 | 768 | `#[test]` function `test_normalize_nav_url_with_path_and_query` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:776 | 776 | `#[test]` function `test_normalize_nav_url_with_fragment` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:784 | 784 | `#[test]` function `test_settings_search_url_empty_query` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:791 | 791 | `#[test]` function `test_settings_search_url_unicode_query` contains no assertions |
| WARN | correctness | assertion_checks | ./tests/sidebar_tests.rs:798 | 798 | `#[test]` function `test_tab_title_update` contains no assertions |
| INFO | correctness | domains | Cargo.toml:0 | 0 | workspace has few members — consider if workspace is needed |
| INFO | correctness | path | ./build.rs:5 | 5 | main: 1 branches, ~0 paths, 0 nesting |
| INFO | correctness | path | ./build.rs:5 | 5 |   path 1: for loop iteration |
| WARN | correctness | path | ./build.rs:8 | 8 | main: `.expect()` panics on None/Err |
| WARN | correctness | path | ./build.rs:11 | 11 | main: `.expect()` panics on None/Err |
| WARN | correctness | path | ./build.rs:13 | 13 | main: `.expect()` panics on None/Err |
| WARN | correctness | path | ./build.rs:75 | 75 | main: `.expect()` panics on None/Err |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:234 | 234 | compute_block_layout: 8 branches, ~0 paths, 2 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:234 | 234 |   path 1: style . box_sizing () == BoxSizing :: ContentBox |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:234 | 234 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:234 | 234 |   path 3: inputs . sizing_mode == SizingMode :: InherentSize |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:234 | 234 |   path 4: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:234 | 234 |   path 5: run_mode == RunMode :: ComputeSize |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:234 | 234 |   path 6: if-let Size { width : Some (width) , height : Some (height) } = styled_based_known_dimensions  [width = width from `styled_based_known_dimensions.width : Some (width) . member.0` (pattern), height = height from `styled_based_known_dimensions.height : Some (height) . member.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:234 | 234 |   path 7: match block_ctx => Some (inherited_bfc) if ! is_scroll_container  [inherited_bfc = inherited_bfc from `block_ctx.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:234 | 234 |   path 8: match block_ctx => _ |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:316 | 316 | compute_inner: 8 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:316 | 316 |   path 1: match direction => Direction :: Ltr |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:316 | 316 |   path 2: match direction => Direction :: Rtl |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:316 | 316 |   path 3: style . box_sizing () == BoxSizing :: ContentBox |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:316 | 316 |   path 4: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:316 | 316 |   path 5: if-let (RunMode :: ComputeSize , Some (container_outer_height)) = (run_mode , known_dimensions . height)  [container_outer_height = container_outer_height from `(run_mode , known_dimensions . height).1.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:316 | 316 |   path 6: block_ctx . is_bfc_root () || is_scroll_container |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:316 | 316 |   path 7: run_mode == RunMode :: ComputeSize |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:316 | 316 |   path 8: for loop iteration |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/block.rs:343 | 343 | compute_inner: match on direction may be non-exhaustive |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:593 | 593 | determine_content_based_container_width: 1 branches, ~0 paths, 0 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:593 | 593 |   path 1: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:642 | 642 | perform_final_layout_on_in_flow_children: 4 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:642 | 642 |   path 1: block_ctx . is_bfc_root () |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:642 | 642 |   path 2: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:642 | 642 |   path 3: own_margins_collapse_with_children . end |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:642 | 642 |   path 4: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:1026 | 1026 | perform_absolute_layout_on_absolute_children: 1 branches, ~0 paths, 0 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/block.rs:1026 | 1026 |   path 1: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:9 | 9 | apply_alignment_fallback: 7 branches, ~0 paths, 2 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:9 | 9 |   path 1: num_items <= 1 || free_space <= 0.0 |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:9 | 9 |   path 2: match alignment_mode => AlignContent :: Stretch |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:9 | 9 |   path 3: match alignment_mode => AlignContent :: SpaceBetween |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:9 | 9 |   path 4: match alignment_mode => AlignContent :: SpaceAround |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:9 | 9 |   path 5: match alignment_mode => AlignContent :: SpaceEvenly |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:9 | 9 |   path 6: match alignment_mode => _ |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:9 | 9 |   path 7: free_space <= 0.0 && is_safe |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:43 | 43 | compute_alignment_offset: 29 branches, ~0 paths, 3 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:43 | 43 |   path 1: num_items == 0  [num_items in [0, 0]] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:43 | 43 |   path 2: is_first |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:43 | 43 |   path 3: match alignment_mode => AlignContent :: Start |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:43 | 43 |   path 4: match alignment_mode => AlignContent :: FlexStart |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:43 | 43 |   path 5: layout_is_flex_reversed |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:43 | 43 |   path 6: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:43 | 43 |   path 7: match alignment_mode => AlignContent :: End |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:43 | 43 |   path 8: match alignment_mode => AlignContent :: FlexEnd |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:43 | 43 |   path 9: layout_is_flex_reversed |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:43 | 43 |   path 10: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:43 | 43 |   ... and 19 more paths (truncated) |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:53 | 53 | compute_alignment_offset: match on alignment_mode may be non-exhaustive |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:82 | 82 | compute_alignment_offset: division by variable — risk of division by zero |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:90 | 90 | compute_alignment_offset: match on alignment_mode may be non-exhaustive |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:97 | 97 | compute_alignment_offset: division by variable — risk of division by zero |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:98 | 98 | compute_alignment_offset: division by variable — risk of division by zero |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/common/alignment.rs:99 | 99 | compute_alignment_offset: division by variable — risk of division by zero |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/content_size.rs:8 | 8 | compute_content_size_contribution: 2 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/content_size.rs:8 | 8 |   path 1: size_content_size_contribution . width > 0.0 && size_content_size_contribution . height > 0.0 |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/common/content_size.rs:8 | 8 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:164 | 164 | compute_flexbox_layout: 6 branches, ~0 paths, 2 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:164 | 164 |   path 1: style . box_sizing () == BoxSizing :: ContentBox |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:164 | 164 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:164 | 164 |   path 3: inputs . sizing_mode == SizingMode :: InherentSize |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:164 | 164 |   path 4: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:164 | 164 |   path 5: run_mode == RunMode :: ComputeSize |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:164 | 164 |   path 6: if-let Size { width : Some (width) , height : Some (height) } = styled_based_known_dimensions  [width = width from `styled_based_known_dimensions.width : Some (width) . member.0` (pattern), height = height from `styled_based_known_dimensions.height : Some (height) . member.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:226 | 226 | compute_preliminary: 9 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:226 | 226 |   path 1: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:226 | 226 |   path 2: if-let Some (inner_main_size) = constants . node_inner_size . main (constants . dir)  [inner_main_size = inner_main_size from `constants . node_inner_size . main (constants . dir).0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:226 | 226 |   path 3: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:226 | 226 |   path 4: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:226 | 226 |   path 5: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:226 | 226 |   path 6: run_mode == RunMode :: ComputeSize |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:226 | 226 |   path 7: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:226 | 226 |   path 8: flex_lines . is_empty () |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:226 | 226 |   path 9: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:416 | 416 | compute_constants: 4 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:416 | 416 |   path 1: style . box_sizing () == BoxSizing :: ContentBox |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:416 | 416 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:416 | 416 |   path 3: match layout_direction => Direction :: Ltr |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:416 | 416 |   path 4: match layout_direction => Direction :: Rtl |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:451 | 451 | compute_constants: match on layout_direction may be non-exhaustive |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:592 | 592 | determine_available_space: 4 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:592 | 592 |   path 1: match known_dimensions . width => Some (node_width)  [node_width = node_width from `known_dimensions . width.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:592 | 592 |   path 2: match known_dimensions . width => None  [None = None from `known_dimensions . width` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:592 | 592 |   path 3: match known_dimensions . height => Some (node_height)  [node_height = node_height from `known_dimensions . height.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:592 | 592 |   path 4: match known_dimensions . height => None  [None = None from `known_dimensions . height` (pattern)] |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:598 | 598 | determine_available_space: match on known_dimensions . width may be non-exhaustive |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:606 | 606 | determine_available_space: match on known_dimensions . height may be non-exhaustive |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:645 | 645 | determine_flex_base_size: 1 branches, ~0 paths, 0 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:645 | 645 |   path 1: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:854 | 854 | collect_flex_lines: 9 branches, ~0 paths, 2 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:854 | 854 |   path 1: ! constants . is_wrap |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:854 | 854 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:854 | 854 |   path 3: match constants . max_size . main (constants . dir) => Some (max_size)  [max_size = max_size from `constants . max_size . main (constants . dir).0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:854 | 854 |   path 4: match constants . max_size . main (constants . dir) => None  [None = None from `constants . max_size . main (constants . dir)` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:854 | 854 |   path 5: match main_axis_available_space => AvailableSpace :: MaxContent |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:854 | 854 |   path 6: match main_axis_available_space => AvailableSpace :: MinContent |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:854 | 854 |   path 7: while (! items . is_empty ()) |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:854 | 854 |   path 8: match main_axis_available_space => AvailableSpace :: Definite (main_axis_available_space)  [main_axis_available_space = main_axis_available_space from `main_axis_available_space.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:854 | 854 |   path 9: while (! flex_items . is_empty ())  [main_axis_available_space = main_axis_available_space from `main_axis_available_space.0` (pattern)] |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:864 | 864 | collect_flex_lines: match on constants . max_size . main (constants . dir) may be non-exhaustive |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:875 | 875 | collect_flex_lines: match on main_axis_available_space may be non-exhaustive |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1172 | 1172 | resolve_flexible_lengths: 3 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1172 | 1172 |   path 1: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1172 | 1172 |   path 2: exactly_sized |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1172 | 1172 |   path 3: loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1368 | 1368 | determine_hypothetical_cross_size: 1 branches, ~0 paths, 0 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1368 | 1368 |   path 1: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1418 | 1418 | calculate_children_base_lines: 2 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1418 | 1418 |   path 1: ! constants . is_row |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1418 | 1418 |   path 2: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1491 | 1491 | calculate_cross_size: 4 branches, ~0 paths, 2 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1491 | 1491 |   path 1: ! constants . is_wrap && node_size . cross (constants . dir) . is_some () |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1491 | 1491 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1491 | 1491 |   path 3: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1491 | 1491 |   path 4: ! constants . is_wrap |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1558 | 1558 | handle_align_content_stretch: 2 branches, ~0 paths, 2 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1558 | 1558 |   path 1: constants . align_content == AlignContent :: Stretch |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1558 | 1558 |   path 2: lines_total_cross < container_min_inner_cross |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1576 | 1576 | handle_align_content_stretch: division by variable — risk of division by zero |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1594 | 1594 | determine_used_cross_size: 1 branches, ~0 paths, 0 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1594 | 1594 |   path 1: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1657 | 1657 | distribute_remaining_free_space: 1 branches, ~0 paths, 0 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1657 | 1657 |   path 1: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1729 | 1729 | resolve_cross_axis_auto_margins: 1 branches, ~0 paths, 0 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1729 | 1729 |   path 1: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1772 | 1772 | align_flex_items_along_cross_axis: 21 branches, ~0 paths, 3 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1772 | 1772 |   path 1: match child . align_self => AlignSelf :: Start |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1772 | 1772 |   path 2: cross_axis_should_reverse |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1772 | 1772 |   path 3: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1772 | 1772 |   path 4: match child . align_self => AlignSelf :: FlexStart |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1772 | 1772 |   path 5: constants . is_wrap_reverse ^ cross_axis_should_reverse |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1772 | 1772 |   path 6: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1772 | 1772 |   path 7: match child . align_self => AlignSelf :: End |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1772 | 1772 |   path 8: cross_axis_should_reverse |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1772 | 1772 |   path 9: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1772 | 1772 |   path 10: match child . align_self => AlignSelf :: FlexEnd |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1772 | 1772 |   ... and 11 more paths (truncated) |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1780 | 1780 | align_flex_items_along_cross_axis: match on child . align_self may be non-exhaustive |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1876 | 1876 | align_flex_lines_per_align_content: 2 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1876 | 1876 |   path 1: constants . is_wrap_reverse |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1876 | 1876 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1900 | 1900 | calculate_flex_item: 16 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1900 | 1900 |   path 1: is_rtl_row |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1900 | 1900 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1900 | 1900 |   path 3: is_rtl_column |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1900 | 1900 |   path 4: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1900 | 1900 |   path 5: is_rtl_column |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1900 | 1900 |   path 6: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1900 | 1900 |   path 7: is_rtl_row |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1900 | 1900 |   path 8: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1900 | 1900 |   path 9: direction . is_row () |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1900 | 1900 |   path 10: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:1900 | 1900 |   ... and 6 more paths (truncated) |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2010 | 2010 | calculate_layout_line: 8 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2010 | 2010 |   path 1: layout_direction . is_rtl () && direction . is_row () |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2010 | 2010 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2010 | 2010 |   path 3: is_rtl_column |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2010 | 2010 |   path 4: direction . is_reverse () |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2010 | 2010 |   path 5: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2010 | 2010 |   path 6: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2010 | 2010 |   path 7: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2010 | 2010 |   path 8: ! is_rtl_column |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2072 | 2072 | final_layout_pass: 8 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2072 | 2072 |   path 1: constants . is_column && constants . layout_direction . is_rtl () |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2072 | 2072 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2072 | 2072 |   path 3: constants . is_wrap_reverse |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2072 | 2072 |   path 4: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2072 | 2072 |   path 5: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2072 | 2072 |   path 6: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2072 | 2072 |   path 7: constants . layout_direction . is_rtl () |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2072 | 2072 |   path 8: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2128 | 2128 | perform_absolute_layout_on_absolute_children: 1 branches, ~0 paths, 0 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2128 | 2128 |   path 1: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2498 | 2498 | sum_axis_gaps: 2 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2498 | 2498 |   path 1: num_items <= 1  [num_items in [-9223372036854775808, 1]] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/flexbox.rs:2498 | 2498 |   path 2: else  [num_items in [2, 9223372036854775807]] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:15 | 15 | align_tracks: 2 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:15 | 15 |   path 1: axis_is_reversed |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:15 | 15 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:65 | 65 | align_and_position_item: 4 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:65 | 65 |   path 1: style . box_sizing () == BoxSizing :: ContentBox |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:65 | 65 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:65 | 65 |   path 3: position == Position :: Absolute && (width . is_none () || height . is_none ()) |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:65 | 65 |   path 4: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:285 | 285 | align_item_within_area: 20 branches, ~0 paths, 3 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:285 | 285 |   path 1: auto_margin_count > 0  [auto_margin_count in [1, 9223372036854775807]] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:285 | 285 |   path 2: else  [auto_margin_count in [-9223372036854775808, 0]] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:285 | 285 |   path 3: match alignment_style => AlignSelf :: Start | AlignSelf :: FlexStart | AlignSelf :: Baseline | AlignSelf :: Stretch |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:285 | 285 |   path 4: direction . is_rtl () |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:285 | 285 |   path 5: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:285 | 285 |   path 6: match alignment_style => AlignSelf :: End | AlignSelf :: FlexEnd |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:285 | 285 |   path 7: direction . is_rtl () |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:285 | 285 |   path 8: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:285 | 285 |   path 9: match alignment_style => AlignSelf :: Center |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:285 | 285 |   path 10: position == Position :: Absolute |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:285 | 285 |   ... and 10 more paths (truncated) |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:301 | 301 | align_item_within_area: truncating cast to margin . start . is_none () as u8 . ty — may silently lose data |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:301 | 301 | align_item_within_area: truncating cast to margin . end . is_none () as u8 . ty — may silently lose data |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:302 | 302 | align_item_within_area: division by variable — risk of division by zero |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:309 | 309 | align_item_within_area: match on alignment_style may be non-exhaustive |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/alignment.rs:329 | 329 | align_item_within_area: match on (inset . start , inset . end) may be non-exhaustive |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:76 | 76 | boundary: auto_repetition_count == 0 |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:24 | 24 | compute_explicit_grid_size_in_axis: 12 branches, ~0 paths, 3 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:24 | 24 |   path 1: match axis => AbsoluteAxis :: Horizontal |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:24 | 24 |   path 2: match axis => AbsoluteAxis :: Vertical |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:24 | 24 |   path 3: track_count == 0  [track_count in [0, 0]] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:24 | 24 |   path 4: template_has_repetitions_with_zero_tracks |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:24 | 24 |   path 5: ! template_is_valid |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:24 | 24 |   path 6: auto_repetition_count == 0 |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:24 | 24 |   path 7: match auto_fit_container_size => None  [None = None from `auto_fit_container_size` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:24 | 24 |   path 8: match auto_fit_container_size => Some (inner_container_size)  [inner_container_size = inner_container_size from `auto_fit_container_size.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:24 | 24 |   path 9: first_repetition_and_non_repeating_tracks_used_space > inner_container_size  [inner_container_size = inner_container_size from `auto_fit_container_size.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:24 | 24 |   path 10: else  [inner_container_size = inner_container_size from `auto_fit_container_size.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:24 | 24 |   ... and 2 more paths (truncated) |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:31 | 31 | compute_explicit_grid_size_in_axis: match on axis may be non-exhaustive |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:67 | 67 | compute_explicit_grid_size_in_axis: truncating cast to template . clone () . filter (| track_def | track_def . is_auto_repetition ()) . count () as u16 . ty — may silently lose data |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:90 | 90 | compute_explicit_grid_size_in_axis: `.expect()` panics on None/Err |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:101 | 101 | compute_explicit_grid_size_in_axis: truncating cast to repetition_definition_iter . len () as u16 . ty — may silently lose data |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:104 | 104 | compute_explicit_grid_size_in_axis: match on auto_fit_container_size may be non-exhaustive |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:161 | 161 | compute_explicit_grid_size_in_axis: division by variable — risk of division by zero |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:171 | 171 | compute_explicit_grid_size_in_axis: match on auto_fit_strategy may be non-exhaustive |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:185 | 185 | initialize_grid_tracks: 11 branches, ~0 paths, 2 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:185 | 185 |   path 1: match axis => AbsoluteAxis :: Horizontal |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:185 | 185 |   path 2: match axis => AbsoluteAxis :: Vertical |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:185 | 185 |   path 3: counts . negative_implicit > 0 |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:185 | 185 |   path 4: auto_track_count == 0  [auto_track_count in [0, 0]] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:185 | 185 |   path 5: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:185 | 185 |   path 6: counts . explicit > 0 |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:185 | 185 |   path 7: if-let Some (track_template) = track_template  [track_template = track_template from `track_template.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:185 | 185 |   path 8: auto_track_count == 0  [auto_track_count in [0, 0]] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:185 | 185 |   path 9: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:185 | 185 |   path 10: if-let Some (first) = tracks . first_mut ()  [first = first from `tracks . first_mut ().0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:185 | 185 |   ... and 1 more paths (truncated) |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:196 | 196 | initialize_grid_tracks: match on axis may be non-exhaustive |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:321 | 321 | initialize_grid_tracks: truncating cast to current_track_index as u16 . ty — may silently lose data |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:338 | 338 | create_implicit_tracks: 1 branches, ~0 paths, 0 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/explicit_grid.rs:338 | 338 |   path 1: for loop iteration |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:19 | 19 | compute_grid_size_estimate: 2 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:19 | 19 |   path 1: tot_inline_tracks < col_max_span |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:19 | 19 |   path 2: tot_block_tracks < row_max_span |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:110 | 110 | child_min_line_max_line_span: 18 branches, ~0 paths, 2 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:110 | 110 |   path 1: match (oz_line . start , oz_line . end) => (Line (track1) , Line (track2))  [track1 = track1 from `(oz_line . start , oz_line . end).0.0` (pattern), track2 = track2 from `(oz_line . start , oz_line . end).1.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:110 | 110 |   path 2: track1 == track2  [track1 = track1 from `(oz_line . start , oz_line . end).0.0` (pattern), track2 = track2 from `(oz_line . start , oz_line . end).1.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:110 | 110 |   path 3: else  [track1 = track1 from `(oz_line . start , oz_line . end).0.0` (pattern), track2 = track2 from `(oz_line . start , oz_line . end).1.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:110 | 110 |   path 4: match (oz_line . start , oz_line . end) => (Line (track) , Auto)  [track = track from `(oz_line . start , oz_line . end).0.0` (pattern), Auto = Auto from `(oz_line . start , oz_line . end).1` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:110 | 110 |   path 5: match (oz_line . start , oz_line . end) => (Line (track) , Span (_))  [track = track from `(oz_line . start , oz_line . end).0.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:110 | 110 |   path 6: match (oz_line . start , oz_line . end) => (Auto , Line (track))  [Auto = Auto from `(oz_line . start , oz_line . end).0` (pattern), track = track from `(oz_line . start , oz_line . end).1.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:110 | 110 |   path 7: match (oz_line . start , oz_line . end) => (Span (span) , Line (track))  [span = span from `(oz_line . start , oz_line . end).0.0` (pattern), track = track from `(oz_line . start , oz_line . end).1.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:110 | 110 |   path 8: match (oz_line . start , oz_line . end) => (Auto | Span (_) , Auto | Span (_))  [Auto = Auto from `(oz_line . start , oz_line . end).1` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:110 | 110 |   path 9: match (oz_line . start , oz_line . end) => (Line (track1) , Line (track2))  [track1 = track1 from `(oz_line . start , oz_line . end).0.0` (pattern), track2 = track2 from `(oz_line . start , oz_line . end).1.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:110 | 110 |   path 10: track1 == track2  [track1 = track1 from `(oz_line . start , oz_line . end).0.0` (pattern), track2 = track2 from `(oz_line . start , oz_line . end).1.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:110 | 110 |   ... and 8 more paths (truncated) |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:126 | 126 | child_min_line_max_line_span: match on (oz_line . start , oz_line . end) may be non-exhaustive |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/implicit_grid.rs:150 | 150 | child_min_line_max_line_span: match on (oz_line . start , oz_line . end) may be non-exhaustive |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:41 | 41 | compute_grid_layout: 24 branches, ~0 paths, 2 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:41 | 41 |   path 1: style . box_sizing () == BoxSizing :: ContentBox |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:41 | 41 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:41 | 41 |   path 3: inputs . sizing_mode == SizingMode :: InherentSize |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:41 | 41 |   path 4: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:41 | 41 |   path 5: match direction => Direction :: Ltr |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:41 | 41 |   path 6: match direction => Direction :: Rtl |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:41 | 41 |   path 7: if-let (RunMode :: ComputeSize , Some (width) , Some (height)) = (run_mode , outer_node_size . width , outer_node_size . height)  [width = width from `(run_mode , outer_node_size . width , outer_node_size . height).1.0` (pattern), height = height from `(run_mode , outer_node_size . width , outer_node_size . height).2.0` (pattern)] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:41 | 41 |   path 8: direction . is_rtl () && final_col_counts . explicit <= 1 |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:41 | 41 |   path 9: direction . is_rtl () |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:41 | 41 |   path 10: run_mode == RunMode :: ComputeSize |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:41 | 41 |   ... and 14 more paths (truncated) |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:91 | 91 | compute_grid_layout: match on direction may be non-exhaustive |
| WARN | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:677 | 677 | compute_grid_layout: `.expect()` panics on None/Err |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:693 | 693 | reverse_non_gutter_tracks: 5 branches, ~0 paths, 2 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:693 | 693 |   path 1: track_counts . explicit <= 1 |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:693 | 693 |   path 2: tracks . len () < MIN_TRACK_VEC_LEN_TO_REVERSE_COLUMNS |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:693 | 693 |   path 3: while (left < right) |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:693 | 693 |   path 4: explicit_track_count < 2  [explicit_track_count in [-9223372036854775808, 1]] |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:693 | 693 |   path 5: while (left < right) |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:726 | 726 | rtl_column_occupancy_index_for_initialization: 3 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:726 | 726 |   path 1: track_counts . explicit <= 1 |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:726 | 726 |   path 2: (explicit_start .. explicit_end) . contains (& column_index) |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/mod.rs:726 | 726 |   path 3: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/placement.rs:20 | 20 | advance_position: 2 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/placement.rs:20 | 20 |   path 1: axis_is_reversed |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/placement.rs:20 | 20 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/placement.rs:30 | 30 | search_start_line: 2 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/placement.rs:30 | 30 |   path 1: axis_is_reversed |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/placement.rs:30 | 30 |   path 2: else |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/placement.rs:44 | 44 | resolve_indefinite_grid_span: 2 branches, ~0 paths, 1 nesting |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/placement.rs:44 | 44 |   path 1: axis_is_reversed |
| INFO | correctness | path | ./crates/aether-caelum/src/compute/grid/placement.rs:44 | 44 |   path 2: else |
