use crate::engine::layout::LayoutBox;

pub struct BrowserRenderer {
    pub layout_tree: Option<Vec<u8>>, // Simplified placeholder for now to allow build
}

impl BrowserRenderer {
    pub fn new(_layout: LayoutBox) -> Self {
        Self { layout_tree: None }
    }
}
