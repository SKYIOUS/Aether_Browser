struct PageCanvas<'a> {
    elements: &'a [StyledElement],
}

impl iced::widget::canvas::Program<BrowserMessage> for PageCanvas<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        plog!("DRAW", "Rendering {} elements into {:?}", self.elements.len(), bounds.size());
        let mut frame = Frame::new(renderer, bounds.size());
        frame.fill_rectangle(Point::new(0.0, 0.0), bounds.size(), iced::Color::WHITE);
        for el in self.elements {
            if let Some(ref handle) = el.image_handle {
                let iw = if el.width.is_finite() { el.width.max(50.0) } else { 50.0 };
                let ih = if el.height.is_finite() { el.height.max(50.0) } else { 50.0 };
                let ix = el.x.max(0.0);
                let iy = el.y.max(0.0);
                if ix.is_finite() && iy.is_finite() && iw.is_finite() && ih.is_finite() {
                    frame.draw_image(Rectangle::new(Point::new(ix, iy), Size::new(iw, ih)), CanvasImage::new(handle.clone()));
                }
                continue;
            }
            let bg = el.background_color;
            let bw = el.border_widths;
            let bc = el.border_color;
            let ex = if el.x.is_finite() { el.x.max(0.0) } else { 0.0 };
            let ey = if el.y.is_finite() { el.y.max(0.0) } else { 0.0 };
            let ew = if el.width.is_finite() { el.width.max(1.0) } else { 1.0 };
            let eh = if el.height > 0.0 && el.height.is_finite() { el.height } else { let f = if el.font_size.is_finite() { el.font_size.clamp(6.0, 200.0) } else { 16.0 }; f * el.line_height.max(1.0) };
            if bg.is_some() || bc.is_some() {
                let fill = bg.unwrap_or(iced::Color::TRANSPARENT);
                frame.fill_rectangle(Point::new(ex, ey), Size::new(ew, eh), fill);
            }
            if let Some(color) = bc {
                if bw[0] > 0.0 { frame.fill_rectangle(Point::new(ex, ey), Size::new(ew, bw[0]), color); }
                if bw[2] > 0.0 { frame.fill_rectangle(Point::new(ex, ey + eh - bw[2]), Size::new(ew, bw[2]), color); }
                if bw[3] > 0.0 { frame.fill_rectangle(Point::new(ex, ey), Size::new(bw[3], eh), color); }
                if bw[1] > 0.0 { frame.fill_rectangle(Point::new(ex + ew - bw[1], ey), Size::new(bw[1], eh), color); }
            }
            let weight = if el.font_weight == "bold" { iced::font::Weight::Bold } else { iced::font::Weight::Normal };
            let fs = if el.font_size.is_finite() { el.font_size.clamp(6.0, 200.0) } else { 16.0 };
            let line_h = fs * el.line_height.max(1.0);
            let px0 = el.x.max(0.0) + bw[3];
            let py0 = el.y.max(0.0) + bw[0];
            let lines: Vec<&str> = if el.wrapped_lines.is_empty() { vec![&el.text] } else { el.wrapped_lines.iter().map(|s| s.as_str()).collect() };
            for (li, line) in lines.iter().enumerate() {
                let py = py0 + li as f32 * line_h;
                if fs.is_finite() && px0.is_finite() && py.is_finite() && !line.is_empty() {
                    frame.fill_text(iced::widget::canvas::Text {
                        content: line.to_string(),
                        position: Point::new(px0, py),
                        color: el.color,
                        size: iced::Pixels(fs),
                        font: iced::Font { weight, ..Default::default() },
                        shaping: iced::widget::text::Shaping::Advanced,
                        ..Default::default()
                    });
                }
            }
        }
        vec![frame.into_geometry()]
    }
