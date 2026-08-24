use super::JsBridge;

impl JsBridge {
    pub fn query_selector(&self, node_id: u32, selector: &str) -> Option<u32> {
        self.select_elements(node_id, selector, false)
            .into_iter()
            .next()
    }

    pub fn query_selector_all(&self, node_id: u32, selector: &str) -> Vec<u32> {
        self.select_elements(node_id, selector, true)
    }
}
