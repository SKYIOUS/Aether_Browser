#[cfg(test)]
mod treesink_tests {
    use vayu_browser::engine::dom::{Node, NodeType};
    use vayu_browser::engine::parser::parse_html;

    fn walk<'a>(node: &'a Node, out: &mut Vec<&'a Node>) {
        out.push(node);
        for child in &node.children {
            walk(child, out);
        }
    }

    fn count_text(doc: &Node, needle: &str) -> usize {
        let mut all = Vec::new();
        walk(doc, &mut all);
        all.iter()
            .filter_map(|n| match &n.node_type {
                NodeType::Text(t) => Some(t.trim()),
                _ => None,
            })
            .filter(|t| *t == needle)
            .count()
    }

    fn texts_inside_tables(doc: &Node) -> Vec<String> {
        let mut all = Vec::new();
        walk(doc, &mut all);
        let mut out = Vec::new();
        for n in &all {
            if let NodeType::Element(e) = &n.node_type {
                if e.tag_name.to_lowercase() == "table" {
                    let mut sub = Vec::new();
                    walk(n, &mut sub);
                    for s in sub {
                        if let NodeType::Text(t) = &s.node_type {
                            if !t.trim().is_empty() {
                                out.push(t.trim().to_string());
                            }
                        }
                    }
                }
            }
        }
        out
    }

    fn dfs_order_texts(doc: &Node) -> Vec<String> {
        let mut all = Vec::new();
        walk(doc, &mut all);
        all.iter()
            .filter_map(|n| match &n.node_type {
                NodeType::Text(t) => {
                    let t = t.trim();
                    if t.is_empty() {
                        None
                    } else {
                        Some(t.to_string())
                    }
                }
                _ => None,
            })
            .collect()
    }

    // html5ever routes stray text between <table> and <tr> through
    // append_before_sibling (foster parenting). The old stub dropped the node
    // entirely, so "x" vanished from the resulting DOM.
    #[test]
    fn foster_parented_text_survives_before_table() {
        let doc = parse_html("<table>x<tr><td>c</td></tr></table>");
        assert_eq!(count_text(&doc, "x"), 1, "foster-parented 'x' was lost");
        assert_eq!(count_text(&doc, "c"), 1);
        let inside = texts_inside_tables(&doc);
        assert!(
            !inside.iter().any(|t| t == "x"),
            "'x' must be fostered OUT of the table, found inside: {inside:?}"
        );
        let order = dfs_order_texts(&doc);
        let xi = order.iter().position(|t| t == "x").expect("'x' present");
        let ci = order.iter().position(|t| t == "c").expect("'c' present");
        assert!(xi < ci, "'x' must precede the table content");
    }

    // The adoption agency exercises reparent_children / remove_from_parent /
    // append_before_sibling. Every fragment must survive exactly once - no
    // loss, no duplication.
    #[test]
    fn misnested_formatting_preserves_each_fragment_once() {
        let doc = parse_html("<b>bold<i>both</b>italic</i>tail");
        for frag in ["bold", "both", "italic", "tail"] {
            assert_eq!(
                count_text(&doc, frag),
                1,
                "fragment '{frag}' appears wrong number of times"
            );
        }
    }

    // Full adoption-agency run: </b> closes while <p> (a block) is still open
    // between the formatting element and top-of-stack, so html5ever calls
    // remove_from_parent + reparent_children. Spec outcome (matches browsers):
    // body{ b{1}, p{ b{2}, 3 } } - a reconstructed <b> nested inside the <p>.
    #[test]
    fn adoption_agency_preserves_text_once_and_nests_per_spec() {
        let doc = parse_html("<b>1<p>2</b>3</p>");
        for frag in ["1", "2", "3"] {
            assert_eq!(
                count_text(&doc, frag),
                1,
                "'{frag}' appears wrong number of times"
            );
        }
        // Spec shape: the <b> is reconstructed INSIDE the <p>, so some <b>
        // must be a descendant of some <p>.
        let has_nested = {
            fn contains_tag(node: &Node, tag: &str) -> bool {
                node.children.iter().any(|c| match &c.node_type {
                    NodeType::Element(e) => {
                        e.tag_name.to_lowercase() == tag || contains_tag(c, tag)
                    }
                    _ => contains_tag(c, tag),
                })
            }
            fn any_p_with_b_child(node: &Node) -> bool {
                match &node.node_type {
                    NodeType::Element(e) if e.tag_name.to_lowercase() == "p" => {
                        contains_tag(node, "b")
                    }
                    _ => node.children.iter().any(any_p_with_b_child),
                }
            }
            any_p_with_b_child(&doc)
        };
        assert!(has_nested, "expected a reconstructed <b> nested inside <p>");
    }
}

#[cfg(test)]
mod tests {
    use vayu_browser::engine::pipeline::extractor::should_skip_tag;

    #[test]
    fn test_should_skip_tag() {
        let skip_set = [
            "script", "style", "noscript", "meta", "link", "head", "title", "svg", "path", "br",
            "hr", "iframe", "option", "template",
        ];
        let keep_set = [
            "div",
            "p",
            "span",
            "a",
            "img",
            "h1",
            "h2",
            "h3",
            "h4",
            "body",
            "html",
            "ul",
            "ol",
            "li",
            "table",
            "tr",
            "td",
            "input",
            "button",
            "textarea",
            "select",
            "label",
            "form",
            "section",
            "article",
            "header",
            "footer",
            "nav",
            "main",
            "aside",
            "figure",
            "figcaption",
            "blockquote",
            "pre",
            "code",
            "em",
            "strong",
            "b",
            "i",
            "u",
            "small",
            "sub",
            "sup",
        ];
        for tag in &skip_set {
            assert!(
                should_skip_tag(tag),
                "expected should_skip_tag({}) = true",
                tag
            );
        }
        for tag in &keep_set {
            assert!(
                !should_skip_tag(tag),
                "expected should_skip_tag({}) = false",
                tag
            );
        }
    }
}
