# Aether Browser Roadmap

## Phase 1 — Make It Stable ✅ COMPLETE
- [x] Basic networking (ureq with TLS)
- [x] HTML parsing with attribute handling
- [x] CSS parser (robust, never panics)
- [x] Basic element extraction (headings, links, text)
- [x] Link clickable and navigable
- [x] No JS - ✓

## Phase 2 — Proper DOM ✅ COMPLETE
- [x] Tokenizer - improved parser with comment handling, CDATA support
- [x] Parser - builds proper DOM tree
- [x] Node types: Element, Text, Comment, Document
- [x] Proper attribute handling
- [x] DOM helper methods: is_element(), is_text(), tag_name(), text_content(), get_elements_by_tag_name()

### DOM Structure Now
```rust
pub enum NodeType {
    Document,
    Text(String),
    Comment(String),
    Element(ElementData),
}

pub struct ElementData {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
}

pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}
```

## Phase 3 — CSS Styling ✅ COMPLETE
- [x] CSS selector matching (element, class, ID selectors) - already in parser
- [x] Style computation/cascade with specificity sorting
- [x] Apply styles to DOM nodes - compute_style function
- [x] Colors (hex, rgb, named colors: black, white, red, green, blue, yellow, gray)
- [x] Margins and padding (all sides + shorthand)
- [x] display: block/inline/none

### New CSS Features
```rust
pub struct ComputedStyle {
    pub color: Option<Color>,
    pub background_color: Option<Color>,
    pub font_size: Option<f32>,
    pub font_weight: Option<String>,
    pub display: Display,  // Inline, Block, None
    pub margin_top/right/bottom/left: Option<f32>,
    pub padding_top/right/bottom/left: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

pub fn compute_style(node: &Node, stylesheet: &Stylesheet) -> ComputedStyle
pub fn compute_styles_for_tree(node: &Node, stylesheet: &Stylesheet, results: &mut HashMap<usize, ComputedStyle>)
```

## Phase 4 — Layout Engine ✅ COMPLETE
- [x] Block layout (vertical stacking of children)
- [x] Inline text layout (text nodes have height)
- [x] Box model (computes heights from content/style)
- [x] display: none support (skips layout)
- [x] LayoutTree::build() creates layout from DOM + CSS

### Layout API
```rust
pub struct LayoutBox { x, y, width, height, children, content }
pub enum LayoutContent { Text(String), Element { tag, style }, Empty }
pub struct LayoutTree { root: LayoutBox, viewport_width }
pub fn LayoutTree::build(&mut self, node: &Node, stylesheet: &Stylesheet)
```

## Phase 5 — Paint Engine ✅ COMPLETE
- [x] LayoutBox provides x, y, width, height for positioning
- [x] ComputedStyle has color, background_color for styling
- [x] LayoutContent helper methods for background/foreground checks
- [x] Integration with Iced UI for text rendering (existing)

Note: Full canvas-based painting deferred; uses Iced widgets for rendering.

## Phase 6 — Images ✅ COMPLETE
- [x] Image element support (<img> tag) in DOM/layout
- [x] Image fetching capability (fetch_bytes in net module)
- [x] PNG/JPEG decoding via `image` crate
- [ ] Lazy loading - deferred

### New Features
- `net::fetch_bytes(url)` - returns raw bytes for binary content
- `engine::image::decode_image(data)` - decodes PNG/JPEG to RGBA
- `engine::image::get_image_dimensions(data)` - get image size
- `LayoutContent::Image { src, alt, width, height, loaded }` - image layout nodes

## Phase 7 — Networking ✅ COMPLETE
- [x] Redirect handling with max 5 redirects
- [x] Response struct with status, headers, final_url
- [x] fetch_with_redirects() for detailed response info
- [ ] Compression (gzip) - needs additional crate
- [ ] Cookies - not implemented (no JS means not critical)

### New Networking Features
```rust
pub struct Response { body, status, headers, final_url }
pub fn fetch_with_redirects(url, max_redirects) -> Result<Response>
pub fn fetch_bytes(url) -> Vec<u8>  // for images
```

## Phase 8 — JavaScript ✅ COMPLETE
- [x] rquickjs integration at src/engine/js/mod.rs
- [x] JSEngine for executing JavaScript code
- [x] Runtime wrapper for simple script execution
- [x] **Integrated into browser** - scripts extracted and executed on page load
- [x] JS errors captured and logged

### JS Integration in Browser
- Scripts extracted from `<script>` tags in DOM
- Executed via JSEngine when page loads
- Errors captured and displayed in console

## Phase 9 — Modern CSS ✅ COMPLETE
- [x] Flexbox layout support (display: flex)
- [x] flex-direction: row, column
- [x] justify-content: flex-start, center, space-between, etc.
- [x] align-items: stretch, center, flex-start, flex-end
- [x] flex-wrap: wrap, nowrap
- [x] CSS transforms (translate, rotate, scale)
- [x] CSS transitions (property, duration, timing)
- [x] CSS animations (name, duration, timing, iteration)

### New CSS Properties
```rust
pub struct FlexOptions { flex_direction, justify_content, align_items, flex_wrap }

pub struct Transform { translate_x, translate_y, rotate, scale_x, scale_y }
pub struct Transition { property, duration, timing_function }
pub struct Animation { name, duration, timing_function, iteration_count }
```

## Roadmap Complete! ✅

All phases 1-9 are complete. Phase 10 (Optimization) intentionally skipped per user request.
- [ ] Redirects
- [ ] Compression
- [ ] Cookies

## Phase 10 — Caelum Layout Engine (DEFERRED)
- [ ] Note: Taffy source available at `C:\Users\nanda\Desktop\KUBUNTU\Browser\taffy\src\`
- [ ] Requires separate Cargo.toml crate setup (cannot be included as internal module)
- [ ] Future: Add as `[dependencies.caelum]` in Cargo.toml with path and features

### Caelum Structure (Reference)
```
src/engine/caelum/
├── lib.rs                 # Module root, re-exports
├── geometry.rs            # 2D geometry types
├── prelude.rs             # Common imports
├── style_helpers.rs       # Style builder utilities
├── compute/               # Layout algorithms
│   ├── mod.rs
│   ├── block.rs           # Block layout
│   ├── flexbox.rs         # Flexbox layout
│   ├── float.rs           # Float layout
│   ├── leaf.rs            # Leaf node layout
│   ├── common/            # Shared utilities
│   └── grid/              # CSS Grid layout
├── style/                 # CSS style types
│   ├── mod.rs
│   ├── alignment.rs
│   ├── available_space.rs
│   ├── block.rs
│   ├── compact_length.rs
│   ├── dimension.rs
│   ├── flex.rs
│   ├── float.rs
│   └── grid.rs
└── tree/                 # Tree data structures
    ├── mod.rs
    ├── cache.rs
    ├── layout.rs
    ├── node.rs
    ├── taffy_tree.rs
    └── traits.rs
```

## Phase 11 — Stratus CSSOM Engine ✅ COMPLETE
- [x] Create `src/engine/stratus/` directory
- [x] Create module files: mod.rs, parser.rs, style_value.rs, matcher.rs, resolver.rs
- [x] Export in src/engine/mod.rs as `pub mod stratus;`

### Stratus Architecture

#### Phase A: CSS Tokenizer & Parser (parser.rs)
- [x] Zero-copy string parser (30,000 char limit)
- [x] Parse raw CSS to AST (Stylesheet → Rules → Selectors + Declarations)
- [x] Support: tag selectors (body, h1), class selectors (.card), ID selectors (#nav)
- [x] Safety: iteration guards, graceful error handling

#### Phase B: Style Storage Engine (style_value.rs)
- [x] Define enums: Display (Flex, Block, Inline, None), Color (HEX/RGB), Unit (Px, Em, Rem, %, Vw, Vh)
- [x] ComputedStyle struct with normalized defaults
- [x] All CSS properties: color, background, font, margin, padding, width, height, flex, transform, transition, animation

#### Phase C: Selector Matcher (matcher.rs)
- [x] `matches(element: &ElementData, selector: &Selector) -> bool`
- [x] Tag name matching
- [x] Class matching (.class)
- [x] ID matching (#id)
- [x] Specificity calculation for cascade ordering

#### Phase D: Style Resolver (resolver.rs)
- [x] CSS Cascade logic (later rules override earlier)
- [x] `resolve_style(element, stylesheet) -> ComputedStyle`
- [x] Apply declarations in specificity order
- [x] Fallback to defaults on malformed properties (log warning, not crash)

### Stratus Module Structure
```
src/engine/stratus/
├── mod.rs          # Module root, public exports
├── style_value.rs  # CSS value types (Display, Color, Unit, ComputedStyle)
├── parser.rs       # CSS tokenizer/parser (Parser, Stylesheet, Rule, Selector, Declaration)
├── matcher.rs      # Selector matching (matches(), specificity())
└── resolver.rs     # Cascade/resolution (resolve_style())
```

### Data Flow
```
Raw CSS String → parser::parse() → Stylesheet
    → matcher::match_rules() → Vec<MatchedRule>
    → resolver::resolve_style() → ComputedStyle (→ Caelum layout)
```

## Phase 8 — JS
- [ ] Embed QuickJS (already in Cargo.toml: rquickjs)

## Phase 9 — Modern CSS
- [ ] Flexbox
- [ ] Transforms
- [ ] Animations

## Phase 12 — Optimization
- [ ] GPU compositing
- [ ] Incremental layout
- [ ] Async rendering