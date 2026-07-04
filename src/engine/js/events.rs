use super::js_bridge::{JsBridge, EventListenerEntry};

impl JsBridge {
    pub fn add_event_listener(&mut self, node_id: u32, event_type: String, source: String) {
        self.event_listeners.push(EventListenerEntry { node_id, event_type, source });
    }

    pub fn remove_event_listener(&mut self, node_id: u32, event_type: String, source: String) {
        self.event_listeners.retain(|e| !(e.node_id == node_id && e.event_type == event_type && e.source == source));
    }

    pub fn get_event_listeners(&self, node_id: u32, event_type: &str) -> Vec<String> {
        self.event_listeners.iter()
            .filter(|e| e.node_id == node_id && e.event_type == event_type)
            .map(|e| e.source.clone())
            .collect()
    }

    /// Returns (source, node_id) for all matching event listeners, including on ancestor nodes.
    pub fn get_event_listeners_bubbling(&self, node_id: u32, event_type: &str) -> Vec<(String, u32)> {
        let mut results = vec![];
        let mut current = Some(node_id);
        while let Some(nid) = current {
            for e in &self.event_listeners {
                if e.node_id == nid && e.event_type == event_type {
                    results.push((e.source.clone(), nid));
                }
            }
            current = self.nodes.get(nid as usize).and_then(|n| n.parent);
        }
        results
    }
}