use std::borrow::Borrow;
use std::fmt;

use cssparser::ToCss;
use precomputed_hash::PrecomputedHash;
use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::context::{
    MatchingContext, MatchingForInvalidation, MatchingMode, NeedsSelectorFlags, QuirksMode,
    SelectorCaches,
};
use selectors::matching::{matches_selector_list, ElementSelectorFlags};
use selectors::parser::{
    NonTSPseudoClass, ParseRelative, Parser as SelectorParser, PseudoElement, SelectorImpl,
    SelectorList, SelectorParseErrorKind,
};
use selectors::{Element, OpaqueElement};

use super::js_bridge::FlatNode;
use super::JsBridge;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct SelStr(String);

impl<'a> From<&'a str> for SelStr {
    fn from(s: &'a str) -> Self {
        SelStr(s.to_string())
    }
}

impl fmt::Display for SelStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl ToCss for SelStr {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str(&self.0)
    }
}

impl PrecomputedHash for SelStr {
    fn precomputed_hash(&self) -> u32 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish() as u32
    }
}

impl Borrow<str> for SelStr {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SelStr {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct VayuSelectorImpl;

impl SelectorImpl for VayuSelectorImpl {
    type ExtraMatchingData<'a> = ();
    type AttrValue = SelStr;
    type Identifier = SelStr;
    type LocalName = SelStr;
    type NamespaceUrl = SelStr;
    type NamespacePrefix = SelStr;
    type BorrowedNamespaceUrl = str;
    type BorrowedLocalName = str;
    type NonTSPseudoClass = VayuPseudoClass;
    type PseudoElement = VayuPseudoElement;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VayuPseudoClass {}

impl NonTSPseudoClass for VayuPseudoClass {
    type Impl = VayuSelectorImpl;

    fn is_active_or_hover(&self) -> bool {
        false
    }

    fn is_user_action_state(&self) -> bool {
        false
    }
}

impl ToCss for VayuPseudoClass {
    fn to_css<W: fmt::Write>(&self, _dest: &mut W) -> fmt::Result {
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VayuPseudoElement;

impl PseudoElement for VayuPseudoElement {
    type Impl = VayuSelectorImpl;
}

impl ToCss for VayuPseudoElement {
    fn to_css<W: fmt::Write>(&self, _dest: &mut W) -> fmt::Result {
        Ok(())
    }
}

#[derive(Debug)]
pub struct VayuSelectorParser;

impl<'i> SelectorParser<'i> for VayuSelectorParser {
    type Impl = VayuSelectorImpl;
    type Error = SelectorParseErrorKind<'i>;
}

#[derive(Clone, Debug)]
pub(crate) struct FlatElement<'a> {
    pub(crate) nodes: &'a [FlatNode],
    pub id: u32,
}

impl<'a> Element for FlatElement<'a> {
    type Impl = VayuSelectorImpl;

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(self)
    }

    fn parent_element(&self) -> Option<Self> {
        let node = self.nodes.get(self.id as usize)?;
        node.parent.map(|pid| FlatElement {
            nodes: self.nodes,
            id: pid,
        })
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        let node = self.nodes.get(self.id as usize)?;
        let parent_id = node.parent?;
        let parent = self.nodes.get(parent_id as usize)?;
        let idx = parent.children.iter().position(|&id| id == self.id)?;
        for &sib_id in parent.children[..idx].iter().rev() {
            if let Some(sib) = self.nodes.get(sib_id as usize) {
                if !sib.is_text {
                    return Some(FlatElement {
                        nodes: self.nodes,
                        id: sib_id,
                    });
                }
            }
        }
        None
    }

    fn next_sibling_element(&self) -> Option<Self> {
        let node = self.nodes.get(self.id as usize)?;
        let parent_id = node.parent?;
        let parent = self.nodes.get(parent_id as usize)?;
        let idx = parent.children.iter().position(|&id| id == self.id)?;
        for &sib_id in parent.children[idx + 1..].iter() {
            if let Some(sib) = self.nodes.get(sib_id as usize) {
                if !sib.is_text {
                    return Some(FlatElement {
                        nodes: self.nodes,
                        id: sib_id,
                    });
                }
            }
        }
        None
    }

    fn first_element_child(&self) -> Option<Self> {
        let node = self.nodes.get(self.id as usize)?;
        for &child_id in &node.children {
            if let Some(child) = self.nodes.get(child_id as usize) {
                if !child.is_text && !child.is_document {
                    return Some(FlatElement {
                        nodes: self.nodes,
                        id: child_id,
                    });
                }
            }
        }
        None
    }

    fn is_html_element_in_html_document(&self) -> bool {
        true
    }

    fn has_local_name(&self, local_name: &<Self::Impl as SelectorImpl>::BorrowedLocalName) -> bool {
        let node = self.nodes.get(self.id as usize);
        node.is_some_and(|n| n.tag == local_name)
    }

    fn has_namespace(&self, ns: &<Self::Impl as SelectorImpl>::BorrowedNamespaceUrl) -> bool {
        ns.is_empty()
    }

    fn is_same_type(&self, other: &Self) -> bool {
        let my_tag = self.nodes.get(self.id as usize).map(|n| n.tag.clone());
        let other_tag = other.nodes.get(other.id as usize).map(|n| n.tag.clone());
        my_tag == other_tag && self.has_namespace("") && other.has_namespace("")
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&<Self::Impl as SelectorImpl>::NamespaceUrl>,
        local_name: &<Self::Impl as SelectorImpl>::LocalName,
        operation: &AttrSelectorOperation<&<Self::Impl as SelectorImpl>::AttrValue>,
    ) -> bool {
        let ns_url: &str = match ns {
            NamespaceConstraint::Any => "",
            NamespaceConstraint::Specific(url) => (**url).as_ref(),
        };
        if !ns_url.is_empty() {
            return false;
        }

        let node = match self.nodes.get(self.id as usize) {
            Some(n) => n,
            None => return false,
        };

        let attr_key: &str = local_name.as_ref();
        let attr_value = match node.attrs.get(attr_key) {
            Some(v) => v,
            None => return false,
        };

        operation.eval_str(attr_value)
    }

    fn match_non_ts_pseudo_class(
        &self,
        _pc: &<Self::Impl as SelectorImpl>::NonTSPseudoClass,
        _context: &mut MatchingContext<VayuSelectorImpl>,
    ) -> bool {
        false
    }

    fn match_pseudo_element(
        &self,
        _pe: &<Self::Impl as SelectorImpl>::PseudoElement,
        _context: &mut MatchingContext<VayuSelectorImpl>,
    ) -> bool {
        false
    }

    fn apply_selector_flags(&self, _flags: ElementSelectorFlags) {}

    fn is_link(&self) -> bool {
        false
    }

    fn is_html_slot_element(&self) -> bool {
        false
    }

    fn has_id(
        &self,
        id: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: CaseSensitivity,
    ) -> bool {
        let node = self.nodes.get(self.id as usize);
        node.is_some_and(|n| {
            n.attrs
                .get("id")
                .is_some_and(|v| case_sensitivity.eq(v.as_bytes(), id.as_ref().as_bytes()))
        })
    }

    fn has_class(
        &self,
        name: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: CaseSensitivity,
    ) -> bool {
        let node = self.nodes.get(self.id as usize);
        node.is_some_and(|n| {
            n.attrs.get("class").is_some_and(|class_attr| {
                class_attr
                    .split_whitespace()
                    .any(|c| case_sensitivity.eq(c.as_bytes(), name.as_ref().as_bytes()))
            })
        })
    }

    fn has_custom_state(&self, _name: &<Self::Impl as SelectorImpl>::Identifier) -> bool {
        false
    }

    fn imported_part(
        &self,
        _name: &<Self::Impl as SelectorImpl>::Identifier,
    ) -> Option<<Self::Impl as SelectorImpl>::Identifier> {
        None
    }

    fn is_part(&self, _name: &<Self::Impl as SelectorImpl>::Identifier) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        let node = self.nodes.get(self.id as usize);
        node.is_some_and(|n| {
            n.children.iter().all(|&c| {
                self.nodes
                    .get(c as usize)
                    .is_none_or(|child| child.is_text && child.text.trim().is_empty())
            })
        })
    }

    fn is_root(&self) -> bool {
        let node = self.nodes.get(self.id as usize);
        node.is_some_and(|n| n.parent.is_none())
    }

    fn add_element_unique_hashes(&self, _filter: &mut selectors::bloom::BloomFilter) -> bool {
        false
    }
}

pub fn parse_selector_list(selector: &str) -> Option<SelectorList<VayuSelectorImpl>> {
    use cssparser::{Parser, ParserInput};

    let mut input = ParserInput::new(selector);
    let mut p = Parser::new(&mut input);

    SelectorList::parse(&VayuSelectorParser, &mut p, ParseRelative::No).ok()
}

pub(crate) fn matches_selector_parsed<'a>(
    selector_list: &SelectorList<VayuSelectorImpl>,
    element: &FlatElement<'a>,
    caches: &mut SelectorCaches,
) -> bool {
    let mut ctx = MatchingContext::new(
        MatchingMode::Normal,
        None,
        caches,
        QuirksMode::NoQuirks,
        NeedsSelectorFlags::No,
        MatchingForInvalidation::No,
    );
    matches_selector_list(selector_list, element, &mut ctx)
}

impl JsBridge {
    pub fn select_elements(&self, start: u32, selector: &str, all: bool) -> Vec<u32> {
        let mut results = vec![];
        if self.nodes.is_empty() {
            return results;
        }

        let selector_list = match parse_selector_list(selector) {
            Some(list) => list,
            None => return results,
        };
        let mut caches = SelectorCaches::default();

        let mut stack: Vec<u32> = self
            .nodes
            .get(start as usize)
            .map(|n| {
                let mut c = n.children.clone();
                c.reverse();
                c
            })
            .unwrap_or_default();

        while let Some(id) = stack.pop() {
            let element = FlatElement {
                nodes: &self.nodes,
                id,
            };
            if matches_selector_parsed(&selector_list, &element, &mut caches) {
                results.push(id);
                if !all {
                    return results;
                }
            }
            if let Some(node) = self.nodes.get(id as usize) {
                for &child in node.children.iter().rev() {
                    stack.push(child);
                }
            }
        }

        results
    }
}
