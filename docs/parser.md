# Parser

Overview:
Parses raw HTML strings into a DOM tree.

Key Structures/Types:
- `Parser`: Contains the parsing state (input and current position).

Public Methods:
- `Parser::new(input: String) -> Parser`: Initializes a new parser.
- `Parser::parse_node() -> Node`: Parses the next node from the stream.
