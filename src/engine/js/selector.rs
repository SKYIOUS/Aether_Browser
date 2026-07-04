use super::js_bridge::{JsBridge, ComplexSel};

impl JsBridge {
    pub fn query_selector(&self, node_id: u32, selector: &str) -> Option<u32> {
        let sel = super::js_bridge::parse_complex(selector)?;
        self.query_sel(node_id, &sel, false).into_iter().next()
    }

    pub fn query_selector_all(&self, node_id: u32, selector: &str) -> Vec<u32> {
        if let Some(sel) = super::js_bridge::parse_complex(selector) {
            self.query_sel(node_id, &sel, true).into_iter().collect()
        } else { vec![] }
    }

    fn query_sel(&self, start: u32, sel: &ComplexSel, all: bool) -> Vec<u32> {
        let mut results = vec![];
        let mut stack: Vec<u32> = self.nodes.get(start as usize).map(|n| n.children.clone()).unwrap_or_default();
        while let Some(id) = stack.pop() {
            if super::js_bridge::matches_complex(&self.nodes, id, sel) {
                results.push(id);
                if !all { return results; }
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