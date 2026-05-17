# Layout Engine

Overview:
Transforms the DOM tree and CSS styles into a positioned and sized layout. Optimized with basic layout caching for repeated tree traversal.

Key Structures/Types:
- `LayoutBox`: Represents a rectangle in the rendered document.
- `Dimensions`: Holds geometric properties like content area, padding, border, and margin.

Public Methods:
- `layout_tree(node: &Node, containing_block: Dimensions) -> LayoutBox`: Calculates the layout with rudimentary caching.
