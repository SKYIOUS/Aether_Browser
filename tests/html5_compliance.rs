#[cfg(test)]
mod tests {
    use aether_browser::engine::pipeline::extractor::should_skip_tag;

    #[test]
    fn test_should_skip_tag() {
        let skip_set = [
            "script", "style", "noscript", "meta", "link", "head",
            "title", "svg", "path", "br", "hr", "iframe",
            "option", "template",
        ];
        let keep_set = [
            "div", "p", "span", "a", "img", "h1", "h2", "h3", "h4",
            "body", "html", "ul", "ol", "li", "table", "tr", "td",
            "input", "button", "textarea", "select", "label", "form", "section", "article",
            "header", "footer", "nav", "main", "aside", "figure",
            "figcaption", "blockquote", "pre", "code", "em", "strong",
            "b", "i", "u", "small", "sub", "sup",
        ];
        for tag in &skip_set {
            assert!(should_skip_tag(tag), "expected should_skip_tag({}) = true", tag);
        }
        for tag in &keep_set {
            assert!(!should_skip_tag(tag), "expected should_skip_tag({}) = false", tag);
        }
    }
}
