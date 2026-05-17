# DOM Tree

Overview:
Represents the hierarchical document structure.

Key Structures/Types:
- `Node`: The fundamental building block of the DOM tree.
- `NodeType`: Can be either `Element` or `Text`.
- `ElementData`: Holds tag names and attributes for element nodes.

Public Methods:
- `Node::new_text(text: String) -> Node`: Creates a new text node.
- `Node::new_element(tag_name: String, attributes: HashMap<String, String>, children: Vec<Node>) -> Node`: Creates a new element node.
