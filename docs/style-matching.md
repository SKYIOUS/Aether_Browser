# Style Matching

Overview:
Applies CSS rules to the DOM tree to produce a `StyledNode` tree. Optimized with a memoization layer (StyleCache) to cache computed styles.

Key Structures/Types:
- `StyledNode`: A wrapper node containing references to the original DOM node and its computed CSS styles.
- `StyleCache`: A memoization structure for caching `StyledNode` instances.

Public Methods:
- `style_tree(node: &Node, stylesheet: &Stylesheet, cache: &mut StyleCache) -> StyledNode`: Matches CSS rules to a DOM node using the provided StyleCache.
