use aether_dom::Node;
use html5ever::parse_document;
use html5ever::tree_builder::TreeSink;
use html5ever::ParseOpts;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use tendril::stream::TendrilSink;

// Type alias for the child map - defined at module level for use in DomSink
type ChildMapTy = HashMap<*const RefCell<Node>, Vec<Rc<RefCell<Node>>>>;

struct DomSink {
    document: Rc<RefCell<Node>>,
    qual_names: RefCell<Vec<html5ever::QualName>>,
    handle_to_index: RefCell<HashMap<*const std::cell::RefCell<Node>, usize>>,
    children: RefCell<ChildMapTy>,
}

impl DomSink {
    fn new() -> Self {
        let initial_names = vec![html5ever::QualName::new(None, "".into(), "".into())];
        DomSink {
            document: Rc::new(RefCell::new(Node::new_document())),
            qual_names: RefCell::new(initial_names),
            handle_to_index: RefCell::new(HashMap::new()),
            children: RefCell::new(HashMap::new()),
        }
    }

    fn store_qual_name(&self, handle: &Rc<RefCell<Node>>, name: html5ever::QualName) -> usize {
        let idx = self.qual_names.borrow().len();
        self.qual_names.borrow_mut().push(name);
        self.handle_to_index
            .borrow_mut()
            .insert(handle.as_ref() as *const _, idx);
        idx
    }

    fn add_child(&self, parent: &Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) {
        self.children
            .borrow_mut()
            .entry(parent.as_ref() as *const _)
            .or_default()
            .push(child);
    }

    fn build_tree(
        handle: Rc<RefCell<Node>>,
        children_map: &HashMap<*const std::cell::RefCell<Node>, Vec<Rc<RefCell<Node>>>>,
    ) -> Node {
        let mut node = (*handle).borrow().clone();
        let ptr = handle.as_ref() as *const _;
        if let Some(child_handles) = children_map.get(&ptr) {
            node.children = child_handles
                .iter()
                .map(|h| Self::build_tree(h.clone(), children_map))
                .collect();
        }
        node
    }
}

impl TreeSink for DomSink {
    type Handle = Rc<RefCell<Node>>;
    type Output = Node;

    fn parse_error(&self, _: std::borrow::Cow<'static, str>) {}
    fn get_document(&self) -> Self::Handle {
        self.document.clone()
    }
    fn set_quirks_mode(&self, _: html5ever::tree_builder::QuirksMode) {}

    fn create_element(
        &self,
        name: html5ever::QualName,
        attrs: Vec<html5ever::tree_builder::Attribute>,
        _: html5ever::tree_builder::ElementFlags,
    ) -> Self::Handle {
        let tag_name = name.local.to_string();
        let attributes: std::collections::HashMap<String, String> = attrs
            .into_iter()
            .map(|a| (a.name.local.to_string(), a.value.to_string()))
            .collect();
        let node = Rc::new(RefCell::new(Node::new_element(
            tag_name,
            attributes,
            Vec::new(),
        )));
        self.store_qual_name(&node, name);
        node
    }

    fn create_comment(&self, text: html5ever::tendril::StrTendril) -> Self::Handle {
        Rc::new(RefCell::new(Node::new_comment(text.to_string())))
    }

    fn append(
        &self,
        parent: &Self::Handle,
        child: html5ever::tree_builder::NodeOrText<Self::Handle>,
    ) {
        crate::plog!("SINK", "append");
        match child {
            html5ever::tree_builder::NodeOrText::AppendNode(node) => {
                self.add_child(parent, node);
            }
            html5ever::tree_builder::NodeOrText::AppendText(text) => {
                let text_node = Rc::new(RefCell::new(Node::new_text(text.to_string())));
                self.add_child(parent, text_node);
            }
        }
    }

    fn append_based_on_parent_node(
        &self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: html5ever::tree_builder::NodeOrText<Self::Handle>,
    ) {
        // Mirrors html5ever's reference sink: a placed `element` means "insert
        // before it" (the foster-parenting path); an unplaced one falls back
        // to appending under prev_element.
        let element_ptr = element.as_ref() as *const _;
        let placed = self
            .children
            .borrow()
            .values()
            .any(|siblings| siblings.iter().any(|c| Rc::as_ptr(c) == element_ptr));
        if placed {
            self.append_before_sibling(element, child);
        } else {
            self.append(prev_element, child);
        }
    }
    fn append_doctype_to_document(
        &self,
        _: html5ever::tendril::StrTendril,
        _: html5ever::tendril::StrTendril,
        _: html5ever::tendril::StrTendril,
    ) {
    }
    fn get_template_contents(&self, _: &Self::Handle) -> Self::Handle {
        self.document.clone()
    }
    fn same_node(&self, a: &Self::Handle, b: &Self::Handle) -> bool {
        Rc::ptr_eq(a, b)
    }
    fn append_before_sibling(
        &self,
        sibling: &Self::Handle,
        child: html5ever::tree_builder::NodeOrText<Self::Handle>,
    ) {
        let node = match child {
            html5ever::tree_builder::NodeOrText::AppendNode(n) => n,
            html5ever::tree_builder::NodeOrText::AppendText(text) => {
                Rc::new(RefCell::new(Node::new_text(text.to_string())))
            }
        };
        let sibling_ptr = sibling.as_ref() as *const _;
        let mut children = self.children.borrow_mut();
        for siblings in children.values_mut() {
            if let Some(pos) = siblings.iter().position(|c| Rc::as_ptr(c) == sibling_ptr) {
                siblings.insert(pos, node);
                return;
            }
        }
        drop(children);
        crate::plog!(
            "PARSE",
            "append_before_sibling: sibling not found in any parent; dropped"
        );
    }
    fn add_attrs_if_missing(&self, _: &Self::Handle, _: Vec<html5ever::tree_builder::Attribute>) {}
    fn remove_from_parent(&self, child: &Self::Handle) {
        let child_ptr = child.as_ref() as *const _;
        let mut children = self.children.borrow_mut();
        for siblings in children.values_mut() {
            // First hit only: a well-formed map holds each child once.
            if let Some(pos) = siblings.iter().position(|c| Rc::as_ptr(c) == child_ptr) {
                siblings.remove(pos);
                return;
            }
        }
    }
    fn reparent_children(&self, node: &Self::Handle, new_parent: &Self::Handle) {
        let old_ptr = node.as_ref() as *const _;
        let new_ptr = new_parent.as_ref() as *const _;
        if old_ptr == new_ptr {
            return;
        }
        let mut children = self.children.borrow_mut();
        // take() empties the old vector so the moved nodes cannot linger in
        // two parent lists after the transition.
        let moved = children
            .get_mut(&old_ptr)
            .map(std::mem::take)
            .unwrap_or_default();
        children.entry(new_ptr).or_default().extend(moved);
    }

    fn create_pi(
        &self,
        _: html5ever::tendril::StrTendril,
        _: html5ever::tendril::StrTendril,
    ) -> Self::Handle {
        Rc::new(RefCell::new(Node::new_comment(String::new())))
    }

    type ElemName<'a> = Ref<'a, html5ever::QualName>;

    fn elem_name<'a>(&'a self, handle: &'a Self::Handle) -> Self::ElemName<'a> {
        let ptr = handle.as_ref() as *const _;
        let idx = *self.handle_to_index.borrow().get(&ptr).unwrap_or(&0);
        Ref::map(self.qual_names.borrow(), |v| &v[idx])
    }

    fn finish(self) -> Self::Output {
        let children_map = self.children.borrow().clone();
        Self::build_tree(self.document.clone(), &children_map)
    }
}

pub fn parse_html(input: &str) -> Node {
    let sink = DomSink::new();
    let mut parser = parse_document(sink, ParseOpts::default());
    let tendril = html5ever::tendril::StrTendril::from(input);
    parser.process(tendril);
    parser.finish()
}
