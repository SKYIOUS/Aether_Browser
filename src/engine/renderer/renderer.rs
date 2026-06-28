use crate::engine::layout::LayoutBox;
use crate::engine::stratus::Color;

#[derive(Debug, Clone)]
pub enum RenderCommand {
    FillRect { x: f32, y: f32, w: f32, h: f32, color: Color },
    DrawText { x: f32, y: f32, text: String, color: Color, size: f32, font_weight: String },
    DrawImage { x: f32, y: f32, w: f32, h: f32, url: String },
    Clip { x: f32, y: f32, w: f32, h: f32 },
}

pub struct DisplayList {
    pub commands: Vec<RenderCommand>,
}

pub struct BrowserRenderer {
    pub display_list: DisplayList,
    pub scale_factor: f32,
}

impl BrowserRenderer {
    pub fn new() -> Self {
        Self {
            display_list: DisplayList { commands: vec![] },
            scale_factor: 1.0,
        }
    }

    pub fn render(&mut self, layout: &LayoutBox) {
        self.display_list.commands.clear();
        self.walk_layout(layout, 0.0, 0.0);
    }

    fn walk_layout(&mut self, node: &LayoutBox, offset_x: f32, offset_y: f32) {
        use crate::engine::layout::LayoutContent;

        let abs_x = offset_x + node.x;
        let abs_y = offset_y + node.y;

        match &node.content {
            LayoutContent::Element { tag: _, style } => {
                if let Some(color) = style.background_color.clone() {
                    self.display_list.commands.push(RenderCommand::FillRect {
                        x: abs_x,
                        y: abs_y,
                        w: node.width,
                        h: node.height,
                        color,
                    });
                }
            }
            LayoutContent::Text(text) => {
                self.display_list.commands.push(RenderCommand::DrawText {
                    x: abs_x,
                    y: abs_y,
                    text: text.clone(),
                    color: Color::BLACK,
                    size: 16.0,
                    font_weight: "normal".to_string(),
                });
            }
            LayoutContent::Image { src, .. } => {
                self.display_list.commands.push(RenderCommand::DrawImage {
                    x: abs_x,
                    y: abs_y,
                    w: node.width,
                    h: node.height,
                    url: src.clone(),
                });
            }
            LayoutContent::Empty => {}
        }

        for child in &node.children {
            self.walk_layout(child, abs_x, abs_y);
        }
    }
}
