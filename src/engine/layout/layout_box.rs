use crate::engine::layout::box_::Dimensions;

pub enum Display {
    Block,
    Inline,
    None,
}

pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

pub enum BoxType<'a> {
    BlockNode(&'a crate::engine::dom::Node),
    InlineNode(&'a crate::engine::dom::Node),
    AnonymousBlock,
}
