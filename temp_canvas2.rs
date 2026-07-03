fn update(
    &self,
    _state: &mut (),
    event: iced::widget::canvas::Event,
    bounds: Rectangle,
    cursor: mouse::Cursor,
) -> (iced::widget::canvas::event::Status, Option<BrowserMessage>) {
    use iced::widget::canvas::event;
    if let iced::widget::canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
        if let Some(pos) = cursor.position_in(bounds) {
            plog!("CLICK", "Click at pos=({:.0},{:.0})", pos.x, pos.y);
            for (i, el) in self.elements.iter().enumerate() {
                if el.is_link {
                    let text_w = el.text.len() as f32 * el.font_size * 0.55;
                    let hit = Rectangle::new(Point::new(el.x, el.y), Size::new(text_w, el.font_size + 4.0));
                    if hit.contains(pos) {
                        plog!("CLICK", "Link hit at element {} href={:?}", i, el.href);
                        if let Some(ref href) = el.href {
                            return (event::Status::Captured, Some(BrowserMessage::LinkClicked(href.clone())));
                        }
                    }
                }
                let ex = el.x.max(0.0);
                let ey = el.y.max(0.0);
                let ew = if el.width.is_finite() { el.width.max(1.0) } else { 200.0 };
                let eh = if el.height > 0.0 && el.height.is_finite() { el.height } else { 30.0 };
                let hit = Rectangle::new(Point::new(ex, ey), Size::new(ew, eh));
                if hit.contains(pos) {
                    plog!("CLICK", "Element {} hit at [{:.0},{:.0} {:.0}x{:.0}] tag={}", i, ex, ey, ew, eh, el.tag);
                    return (event::Status::Captured, Some(BrowserMessage::ElementClicked(i)));
                }
            }
        }
    }
    (event::Status::Ignored, None)
}
