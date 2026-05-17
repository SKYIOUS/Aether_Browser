# CSS Engine

Overview:
Handles CSS parsing and representation.

Key Structures/Types:
- `Stylesheet`: Holds a collection of CSS rules.
- `Rule`: Represents a single CSS rule (selector and declarations).
- `Selector`: Discriminates between types of CSS selectors.
- `Declaration`: A key-value pair for property and value.

Public Methods:
- `parse(source: &str) -> Stylesheet`: Parses CSS source into a `Stylesheet`.
