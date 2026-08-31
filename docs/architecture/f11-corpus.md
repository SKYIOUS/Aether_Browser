# F11 Corpus Manifest

> Canonical list of pages in the F11 production-readiness corpus.
> Each entry is a deterministic (HTML, CSS) pair — no live URLs.
> Updated as corpus grows during F11-A.

## Category index

| # | Category | Pages | Coverage |
|---|----------|-------|----------|
| 1 | Ordinary document | 3 | block flow, basic stacking |
| 2 | Typography-heavy | 3 | text measurement, wrapping, mixed sizes |
| 3 | Nested block layouts | 3 | margin collapse, containment, depth |
| 4 | Inline/inline-block | 3 | line breaking, baseline, wrapping |
| 5 | Flex layouts | 4 | row, column, nested, wrap |
| 6 | Block + flex + inline | 2 | mixed formatting contexts |
| 7 | Positioned elements | 3 | absolute, relative, inset |
| 8 | Padding/borders/margins | 3 | box model offsets, combined |
| 9 | Narrow/wide containers | 2 | overflow, explicit width |
| 10 | Deeply nested documents | 2 | recursion, margin stack |
| 11 | Long text | 2 | measurement at scale |
| 12 | Many elements | 3 | flat, inline, mixed |
| **Total** | | **33** | |

## Pages

### Category 1: Ordinary document

#### P01 — single_block_element
- **Source:** layout_stress.rs
- **HTML:** `<div>hello</div>`
- **CSS:** (empty)
- **Elements:** 3 (html, body, div)
- **Supported:** block flow, text height
- **Unsupported:** none

#### P02 — two_blocks_parent_child
- **Source:** layout_stress.rs
- **HTML:** `<div><p>child</p></div>`
- **CSS:** (empty)
- **Elements:** 4 (html, body, div, p)
- **Supported:** block containment, child stacking
- **Unsupported:** none

#### P03 — document_paragraphs
- **Source:** new (F11-A)
- **HTML:** `<div class="doc"><p>First paragraph with some text content.</p><p>Second paragraph with different text content.</p><p>Third paragraph to test stacking.</p></div>`
- **CSS:** `.doc { width: 800px; } p { margin: 8px 0; }`
- **Elements:** ~6
- **Supported:** block flow, margin stacking
- **Unsupported:** none

### Category 2: Typography-heavy

#### P04 — large_text_elements
- **Source:** layout_stress.rs
- **HTML:** 10 `<p>` elements each with 500+ char Lorem Ipsum
- **CSS:** (empty)
- **Elements:** ~23
- **Supported:** text measurement, wrapping
- **Unsupported:** none

#### P05 — mixed_font_sizes
- **Source:** new (F11-A)
- **HTML:** `<div><h1 style="font-size:32px">Heading</h1><p style="font-size:16px">Body text</p><p style="font-size:12px">Small text</p><p style="font-size:24px">Large body text</p></div>`
- **CSS:** `.doc { width: 600px; }`
- **Elements:** ~7
- **Supported:** font-size inheritance, text height variation
- **Unsupported:** none

#### P06 — long_paragraph_wrapping
- **Source:** new (F11-A)
- **HTML:** `<div class="wrap"><p>Supercalifragilisticexpialidocious is a long word that should force wrapping in a narrow container. The quick brown fox jumps over the lazy dog near the riverbank on a sunny afternoon.</p></div>`
- **CSS:** `.wrap { width: 300px; }`
- **Elements:** ~4
- **Supported:** word-wrap, narrow container text flow
- **Unsupported:** none

### Category 3: Nested block layouts

#### P07 — deep_nesting_50
- **Source:** layout_stress.rs
- **HTML:** 50-level nested `<div>` chain
- **CSS:** (empty)
- **Elements:** ~53
- **Supported:** recursive containment
- **Unsupported:** none

#### P08 — margins_affect_layout
- **Source:** layout_stress.rs
- **HTML:** 5 divs with margin-top=20, margin-bottom=20, height=30
- **CSS:** inline styles
- **Elements:** ~8
- **Supported:** margin collapse, vertical stacking
- **Unsupported:** none

#### P09 — nested_blocks_with_padding
- **Source:** new (F11-A)
- **HTML:** `<div class="outer"><div class="mid"><div class="inner"><p>Deep content</p></div></div></div>`
- **CSS:** `.outer { padding: 10px; } .mid { padding: 8px; margin: 4px; } .inner { padding: 6px; } p { margin: 0; }`
- **Elements:** ~7
- **Supported:** nested padding accumulation, margin collapse
- **Unsupported:** none

### Category 4: Inline/inline-block

#### P10 — mixed_inline_block
- **Source:** layout_stress.rs
- **HTML:** 20 children alternating inline/block
- **CSS:** (empty)
- **Elements:** ~23
- **Supported:** inline-block interleaving
- **Unsupported:** none

#### P11 — thousand_inline_siblings
- **Source:** layout_stress.rs
- **HTML:** 1000 inline `<span>` siblings
- **CSS:** (empty)
- **Elements:** ~1003
- **Supported:** inline reflow at scale
- **Unsupported:** none

#### P12 — inline_wrapping
- **Source:** new (F11-A)
- **HTML:** `<div class="iw"><span>A</span><span>B</span><span>C</span><span>D</span><span>E</span><span>F</span><span>G</span><span>H</span></div>`
- **CSS:** `.iw { width: 200px; } span { margin: 2px; padding: 2px; font-size: 16px; }`
- **Elements:** ~11
- **Supported:** inline wrapping across lines, margin/padding on inlines
- **Unsupported:** none

### Category 5: Flex layouts

#### P13 — flex_row
- **Source:** native_gap_regression.rs
- **HTML:** `<div class="fc"><div class="item">1</div><div class="item">2</div><div class="item">3</div></div>`
- **CSS:** `.fc { display: flex; width: 800px; } .item { flex-grow: 1; flex-shrink: 1; flex-basis: 0px; height: 50px; }`
- **Elements:** ~6
- **Supported:** flex-grow distribution
- **Unsupported:** none

#### P14 — flex_column
- **Source:** native_gap_regression.rs
- **HTML:** `<div class="fcc"><div class="col">a</div><div class="col">b</div></div>`
- **CSS:** `.fcc { display: flex; flex-direction: column; width: 400px; } .col { height: 50px; }`
- **Elements:** ~5
- **Supported:** flex-direction column
- **Unsupported:** none

#### P15 — nested_flex
- **Source:** native_gap_regression.rs
- **HTML:** `<div class="nf"><div class="flex1"><div class="inner2">inner</div></div><div class="sib">sibling</div></div>`
- **CSS:** `.nf { display: flex; width: 800px; } .flex1 { flex: 1; display: flex; } .inner2 { flex: 1; height: 30px; } .sib { flex: 1; height: 30px; }`
- **Elements:** ~7
- **Supported:** nested flex containers, flex:1 distribution
- **Unsupported:** none

#### P16 — flex_wrap
- **Source:** new (F11-A)
- **HTML:** `<div class="fw"><div class="item">1</div><div class="item">2</div><div class="item">3</div><div class="item">4</div><div class="item">5</div></div>`
- **CSS:** `.fw { display: flex; flex-wrap: wrap; width: 200px; } .item { width: 80px; height: 40px; margin: 5px; }`
- **Elements:** ~8
- **Supported:** flex-wrap, multi-line flex
- **Unsupported:** none

### Category 6: Block + flex + inline

#### P17 — all_display_types
- **Source:** native_gap_regression.rs
- **HTML:** `<div class="container"><div>block</div><div class="inline">inline</div><div class="ib">inline-block</div><div class="flex">flex</div></div>`
- **CSS:** `.container { width: 800px; margin: 4px; } .inline { display: inline; } .ib { display: inline-block; width: 100px; height: 50px; } .flex { display: flex; }`
- **Elements:** ~11
- **Supported:** mixed formatting contexts
- **Unsupported:** none

#### P18 — block_then_inline_then_flex
- **Source:** new (F11-A)
- **HTML:** `<div class="mix"><div class="block">Block content</div><span class="inline">Inline text</span><div class="flexbox"><div>A</div><div>B</div></div></div>`
- **CSS:** `.mix { width: 600px; } .block { height: 30px; } .inline { font-size: 16px; } .flexbox { display: flex; height: 40px; } .flexbox > div { flex: 1; }`
- **Elements:** ~8
- **Supported:** block→inline→flex sequence
- **Unsupported:** none

### Category 7: Positioned elements

#### P19 — absolute_positioning
- **Source:** native_gap_regression.rs
- **HTML:** `<div class="rel"><div class="abs">abs</div></div>`
- **CSS:** `.rel { position: relative; width: 800px; height: 600px; } .abs { position: absolute; top: 10px; left: 20px; width: 100px; height: 50px; }`
- **Elements:** ~5
- **Supported:** absolute positioning, containing block
- **Unsupported:** none

#### P20 — relative_positioning
- **Source:** new (F11-A)
- **HTML:** `<div class="doc"><div class="rel1">Moved right</div><div class="normal">Normal flow</div><div class="rel2">Moved down</div></div>`
- **CSS:** `.doc { width: 600px; } .rel1 { position: relative; left: 30px; height: 30px; } .normal { height: 30px; } .rel2 { position: relative; top: 20px; height: 30px; }`
- **Elements:** ~6
- **Supported:** relative positioning (offset, still in flow)
- **Unsupported:** none

#### P21 — absolute_in_nested_relative
- **Source:** new (F11-A)
- **HTML:** `<div class="outer"><div class="inner"><div class="abs">abs child</div></div></div>`
- **CSS:** `.outer { position: relative; width: 400px; height: 300px; } .inner { position: relative; width: 200px; height: 150px; margin: 20px; } .abs { position: absolute; top: 10px; left: 10px; width: 50px; height: 50px; }`
- **Elements:** ~6
- **Supported:** absolute child in nested relative context
- **Unsupported:** none

### Category 8: Padding/borders/margins

#### P22 — padding_contains_children
- **Source:** layout_stress.rs
- **HTML:** parent with padding=[10;4], width=400, height=300; child 100x50
- **CSS:** (inline styles)
- **Elements:** ~4
- **Supported:** padding-box child offset
- **Unsupported:** none

#### P23 — borders_no_crash
- **Source:** layout_stress.rs
- **HTML:** parent with 2px borders, child with 1px borders
- **CSS:** (inline styles)
- **Elements:** ~4
- **Supported:** border-box child offset
- **Unsupported:** none

#### P24 — padding_border_margin_combined
- **Source:** new (F11-A)
- **HTML:** `<div class="box"><div class="child">content</div></div>`
- **CSS:** `.box { width: 300px; height: 200px; padding: 15px; border: 3px solid black; margin: 10px; } .child { width: 100px; height: 50px; padding: 5px; border: 2px solid red; margin: 8px; }`
- **Elements:** ~4
- **Supported:** nested box model (padding + border + margin on both levels)
- **Unsupported:** none

### Category 9: Narrow/wide containers

#### P25 — wide_container
- **Source:** layout_stress.rs
- **HTML:** 50 children at 400x20 in 2000px-wide container
- **CSS:** (inline styles)
- **Elements:** ~53
- **Supported:** explicit width > viewport
- **Unsupported:** none

#### P26 — narrow_container
- **Source:** new (F11-A)
- **HTML:** `<div class="narrow"><p>This text should wrap multiple times in this narrow container to test text reflow and container height calculation.</p></div>`
- **CSS:** `.narrow { width: 150px; }`
- **Elements:** ~4
- **Supported:** narrow width text wrapping, auto height
- **Unsupported:** none

### Category 10: Deeply nested documents

#### P27 — deep_nesting_100
- **Source:** layout_stress.rs
- **HTML:** 100-level nested `<div>` chain
- **CSS:** (empty)
- **Elements:** ~103
- **Supported:** deep recursion
- **Unsupported:** none

#### P28 — nested_positions
- **Source:** layout_stress.rs
- **HTML:** child listed BEFORE parent in element vec
- **CSS:** (inline styles)
- **Elements:** ~5
- **Supported:** parent_index reordering
- **Unsupported:** none

### Category 11: Long text

#### P29 — long_single_paragraph
- **Source:** new (F11-A)
- **HTML:** `<div class="long"><p>[2000+ chars ofLorem Ipsum text]</p></div>`
- **CSS:** `.long { width: 600px; }`
- **Elements:** ~4
- **Supported:** long text measurement, multi-line wrapping
- **Unsupported:** none

#### P30 — many_short_paragraphs
- **Source:** new (F11-A)
- **HTML:** 50 `<p>` elements each with 20-30 char text
- **CSS:** `.doc { width: 600px; } p { margin: 4px 0; }`
- **Elements:** ~53
- **Supported:** repeated text measurement, margin stacking
- **Unsupported:** none

### Category 12: Many elements

#### P31 — thousand_flat_elements
- **Source:** layout_stress.rs
- **HTML:** root + 999 flat `<div>` children
- **CSS:** (empty)
- **Elements:** ~1002
- **Supported:** flat block stacking at scale
- **Unsupported:** none

#### P32 — thousand_inline_siblings
- **Source:** layout_stress.rs (also P11)
- **HTML:** 1000 inline `<span>` siblings
- **CSS:** (empty)
- **Elements:** ~1003
- **Supported:** inline reflow at scale
- **Unsupported:** none

#### P33 — mixed_elements_large
- **Source:** new (F11-A)
- **HTML:** 200 elements: alternating div/p/span with varied styles
- **CSS:** `.root { width: 600px; } div { height: 20px; margin: 2px; } p { margin: 4px 0; } span { font-size: 14px; }`
- **Elements:** ~203
- **Supported:** mixed elements at scale
- **Unsupported:** none

## Unsupported features (tracked separately)

| Feature | Pages affected | Classification |
|---------|---------------|----------------|
| CSS Grid | none in current corpus | UNSUPPORTED |
| overflow: hidden | none in current corpus | UNSUPPORTED |

## Notes

- All pages use `container_width=800.0, viewport_height=600.0` unless noted
- Element counts are approximate (html/body wrapper elements add 2-3)
- "Source: new (F11-A)" means fixture is created during this phase
- "Source: layout_stress.rs" means fixture is extracted from existing tests
