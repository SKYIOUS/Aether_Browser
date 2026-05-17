//! Stratus Selector Matcher
//! Matches DOM elements against CSS selectors

use super::parser::{Selector, SimpleSelector, Stylesheet, Rule, Declaration};

pub type Specificity = (usize, usize, usize);

pub struct ElementData {
    pub tag_name: String,
    pub attributes: std::collections::HashMap<String, String>,
}

impl ElementData {
    pub fn new(tag_name: String) -> Self {
        ElementData {
            tag_name,
            attributes: std::collections::HashMap::new(),
        }
    }

    pub fn with_attributes(tag_name: String, attributes: std::collections::HashMap<String, String>) -> Self {
        ElementData { tag_name, attributes }
    }

    pub fn has_class(&self, class: &str) -> bool {
        self.attributes.get("class")
            .map(|classes| classes.split_whitespace().any(|c| c == class))
            .unwrap_or(false)
    }

    pub fn get_id(&self) -> Option<&str> {
        self.attributes.get("id").map(|s| s.as_str())
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }
}

impl SimpleSelector {
    pub fn matches(&self, element: &ElementData) -> bool {
        if let Some(ref tag) = self.tag_name {
            if tag.to_lowercase() != element.tag_name.to_lowercase() {
                return false;
            }
        }

        if let Some(ref id) = self.id {
            match element.get_id() {
                Some(elem_id) if elem_id == id => {},
                _ => return false,
            }
        }

        for class in &self.class {
            if !element.has_class(class) {
                return false;
            }
        }

        if let Some(ref attr) = self.attribute {
        let (attr_name, attr_value) = attr;
            match element.get_attribute(attr_name) {
                Some(value) if value == attr_value => {},
                _ => return false,
            }
        }

        true
    }

    pub fn specificity(&self) -> Specificity {
        let a = if self.id.is_some() { 1 } else { 0 };
        let b = self.class.len() + self.attribute.iter().count() + self.pseudo_class.iter().count();
        let c = if self.tag_name.is_some() { 1 } else { 0 };
        (a, b, c)
    }
}

impl Selector {
    pub fn matches(&self, element: &ElementData) -> bool {
        match self {
            Selector::Simple(s) => s.matches(element),
            Selector::Composite(selectors) => {
                selectors.iter().all(|s| s.matches(element))
            }
        }
    }

    pub fn specificity(&self) -> Specificity {
        match self {
            Selector::Simple(s) => s.specificity(),
            Selector::Composite(selectors) => {
                selectors.iter().fold((0, 0, 0), |acc, s| {
                    let sp = s.specificity();
                    (acc.0 + sp.0, acc.1 + sp.1, acc.2 + sp.2)
                })
            }
        }
    }
}

pub fn match_element<'a, 'b>(element: &'b ElementData, stylesheet: &'a Stylesheet) -> Vec<(&'a Rule, &'a Selector)> {
    let mut matched = Vec::new();

    for rule in &stylesheet.rules {
        for selector in &rule.selectors {
            if selector.matches(element) {
                matched.push((rule, selector));
            }
        }
    }

    matched.sort_by(|a, b| {
        let sp_a = a.1.specificity();
        let sp_b = b.1.specificity();
        sp_a.cmp(&sp_b)
    });

    matched
}

pub fn match_rules<'a, 'b>(element: &'b ElementData, stylesheet: &'a Stylesheet) -> Vec<(&'a Vec<Declaration>, Specificity)> {
    let matched = match_element(element, stylesheet);

    matched.iter()
        .map(|(rule, selector)| (&rule.declarations, selector.specificity()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parser::parse;

    fn element_with_attrs(tag: &str, attrs: &[(&str, &str)]) -> ElementData {
        let mut attributes = std::collections::HashMap::new();
        for (k, v) in attrs {
            attributes.insert(k.to_string(), v.to_string());
        }
        ElementData::with_attributes(tag.to_string(), attributes)
    }

    #[test]
    fn test_tag_matching() {
        let css = "div { color: red; }";
        let stylesheet = parse(css);
        let element = element_with_attrs("div", &[]);

        let matched = match_element(&element, &stylesheet);
        assert_eq!(matched.len(), 1);
    }

    #[test]
    fn test_class_matching() {
        let css = ".card { background: white; }";
        let stylesheet = parse(css);
        let element = element_with_attrs("div", &[("class", "card container")]);

        let matched = match_element(&element, &stylesheet);
        assert_eq!(matched.len(), 1);
    }

    #[test]
    fn test_id_matching() {
        let css = "#nav { display: flex; }";
        let stylesheet = parse(css);
        let element = element_with_attrs("nav", &[("id", "nav")]);

        let matched = match_element(&element, &stylesheet);
        assert_eq!(matched.len(), 1);
    }

    #[test]
    fn test_no_match() {
        let css = "div { color: red; }";
        let stylesheet = parse(css);
        let element = element_with_attrs("span", &[]);

        let matched = match_element(&element, &stylesheet);
        assert!(matched.is_empty());
    }

    #[test]
    fn test_specificity() {
        let css = "#id { color: red; } .class { color: blue; } div { color: green; }";
        let stylesheet = parse(css);
        let element = element_with_attrs("div", &[("id", "id"), ("class", "class")]);

        let matched = match_element(&element, &stylesheet);
        assert_eq!(matched.len(), 3);

        assert_eq!(matched[0].1.specificity(), (0, 0, 1));
        assert_eq!(matched[1].1.specificity(), (0, 1, 0));
        assert_eq!(matched[2].1.specificity(), (1, 0, 0));
    }
}