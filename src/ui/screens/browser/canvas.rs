use super::BrowserMessage;
use crate::engine::pipeline::extractor::{FontWeight, StyledElement, TextDecor};
use crate::engine::stratus as css;
use crate::engine::text::measure_text_width;
use crate::plog;
use iced::mouse;
use iced::widget::canvas::{Cache, Geometry, Image as CanvasImage};
use iced::{Point, Rectangle, Size};
use std::sync::Arc;
use std::time::Instant;

// D3-A/B/C: Paint/animation profiling instrumentation
#[derive(Default, Clone, Copy)]
struct PaintProfile {
    cache_hit: bool,
    total_ms: f64,
    geometry_ms: f64,
    text_ms: f64,
    image_ms: f64,
    box_ms: f64,
    form_ms: f64,
    elements_drawn: usize,
    elements_culled: usize,
}

thread_local! {
    static LAST_PAINT: std::cell::RefCell<Option<Instant>> = const { std::cell::RefCell::new(None) };
    static PAINT_COUNT: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
    static CACHE_INVALIDATIONS: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
    static LAST_INVALIDATION_REASON: std::cell::RefCell<Option<&'static str>> = const { std::cell::RefCell::new(None) };
}

fn record_paint(profile: PaintProfile) {
    LAST_PAINT.with(|lp| *lp.borrow_mut() = Some(Instant::now()));
    PAINT_COUNT.with(|c| *c.borrow_mut() += 1);
    let count = PAINT_COUNT.with(|c| *c.borrow());

    if count % 60 == 1 || profile.total_ms > 16.0 {
        let idle_ms = LAST_PAINT.with(|lp| {
            lp.borrow()
                .map(|t| t.elapsed().as_millis() as u64)
                .unwrap_or(0)
        });
        plog!("D3", "paint #{:>5} cache={} total={:.2}ms geom={:.2}ms text={:.2}ms img={:.2}ms box={:.2}ms form={:.2}ms drawn={} culled={} idle={}ms",
            count,
            if profile.cache_hit { "HIT" } else { "MISS" },
            profile.total_ms, profile.geometry_ms, profile.text_ms, profile.image_ms, profile.box_ms, profile.form_ms,
            profile.elements_drawn, profile.elements_culled, idle_ms);
    }
}

fn record_invalidation(reason: &'static str) {
    CACHE_INVALIDATIONS.with(|c| *c.borrow_mut() += 1);
    LAST_INVALIDATION_REASON.with(|r| *r.borrow_mut() = Some(reason));
    let count = CACHE_INVALIDATIONS.with(|c| *c.borrow());
    if count % 10 == 1 {
        plog!("D3", "cache invalidation #{}: {}", count, reason);
    }
}

/// Public wrapper for cache invalidation tracking
pub fn record_canvas_invalidation(reason: &'static str) {
    record_invalidation(reason);
}

pub struct PageCanvas {
    pub elements: Arc<Vec<StyledElement>>,
    pub cache: Cache,
    pub focused_index: Option<usize>,
    pub scroll_top: f32,
    pub viewport_h: f32,
    pub cull: CullIndex,
}

impl PageCanvas {
    pub fn new(
        elements: Arc<Vec<StyledElement>>,
        focused_index: Option<usize>,
        viewport_h: f32,
    ) -> Self {
        let cull = CullIndex::from_spans(
            elements
                .iter()
                .map(painted_vertical_span)
                .collect::<Vec<_>>()
                .as_slice(),
        );
        Self {
            elements,
            cache: Cache::new(),
            focused_index,
            scroll_top: 0.0,
            viewport_h,
            cull,
        }
    }

    // Single source of truth for visibility: both paint and hit-test iterate
    // this window and apply the same in_band predicate, so culling can never
    // make the two disagree.
    fn band_window(&self, top: f32, bottom: f32) -> std::ops::Range<usize> {
        self.cull.window(top, bottom)
    }
}

// Half-open band [top, bottom): an element ending exactly at top or starting
// exactly at bottom is culled. Non-finite coordinates never match.
fn in_band(sy: f32, ey: f32, top: f32, bottom: f32) -> bool {
    if !sy.is_finite() || !ey.is_finite() {
        return false;
    }
    sy < bottom && ey > top
}

// Conservative painted vertical extent: a superset of what any of the three
// draw branches (image / form control / text+box) can render for this element.
// Wrapped-line overflow past the box is covered by the lines term.
fn painted_vertical_span(el: &StyledElement) -> (f32, f32) {
    let sy = if el.y.is_finite() { el.y.max(0.0) } else { 0.0 };
    let fs = if el.font_size.is_finite() {
        el.font_size.clamp(6.0, 200.0)
    } else {
        16.0
    };
    let lh = el.line_height.max(1.0);
    let box_h = if el.height > 0.0 && el.height.is_finite() {
        el.height
    } else {
        fs * lh
    };
    let form_min = if matches!(el.tag.as_str(), "input" | "textarea" | "select" | "button") {
        32.0
    } else {
        0.0
    };
    let img_min = if el.image_handle.is_some() { 50.0 } else { 0.0 };
    let link_h = if el.is_link { fs + 4.0 } else { 0.0 };
    let lines = el.wrapped_lines.len().max(1) as f32;
    (
        sy,
        sy + box_h.max(form_min).max(img_min).max(link_h).max(lines * lh),
    )
}

// Spatial index over elements sorted by painted start-y, with a prefix-max of
// end-y so the first element able to reach the band's top edge is findable by
// binary search even when early containers span the whole page. window()
// returns a superset range; callers filter with in_band for exactness.
pub(crate) struct CullIndex {
    order: Vec<u32>,
    starts: Vec<f32>,
    prefix_max_ends: Vec<f32>,
}

impl CullIndex {
    pub(crate) fn from_spans(spans: &[(f32, f32)]) -> Self {
        let mut order: Vec<u32> = (0..spans.len() as u32).collect();
        order.sort_by(|&a, &b| {
            let sa = sanitize(spans[a as usize].0);
            let sb = sanitize(spans[b as usize].0);
            sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
        });
        let mut starts = Vec::with_capacity(order.len());
        let mut prefix_max_ends = Vec::with_capacity(order.len());
        let mut running_max = f32::NEG_INFINITY;
        for &i in &order {
            let (s, e) = spans[i as usize];
            starts.push(sanitize(s));
            running_max = running_max.max(sanitize(e));
            prefix_max_ends.push(running_max);
        }
        Self {
            order,
            starts,
            prefix_max_ends,
        }
    }

    // Superset [lo, end) of band members: everything before lo provably ends
    // above the band; everything from end on provably starts below it.
    pub(crate) fn window(&self, top: f32, bottom: f32) -> std::ops::Range<usize> {
        if !(top.is_finite() && bottom.is_finite()) || top >= bottom {
            return 0..0;
        }
        let end = self.starts.partition_point(|&s| s < bottom);
        let lo = self.prefix_max_ends.partition_point(|&p| p <= top);
        lo..end.max(lo)
    }
}

fn sanitize(v: f32) -> f32 {
    if v.is_finite() {
        v
    } else {
        f32::INFINITY
    }
}

impl iced::widget::canvas::Program<BrowserMessage> for PageCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let size = bounds.size();
        let band_top = self.scroll_top;
        let band_bottom = self.scroll_top + self.viewport_h;

        let total_start = Instant::now();
        let mut profile = PaintProfile::default();

        // Detect cache miss by checking if closure executes
        let cache_miss = std::cell::RefCell::new(true);

        let geometry = self.cache.draw(renderer, size, |frame| {
            cache_miss.replace(false);
            let geom_start = Instant::now();

            frame.fill_rectangle(Point::new(0.0, 0.0), size, iced::Color::WHITE);

            let window = self.band_window(band_top, band_bottom);
            profile.elements_culled = self.elements.len().saturating_sub(window.len());

            for pos in window {
                let i = self.cull.order[pos] as usize;
                let el = &self.elements[i];
                let (sy, ey) = painted_vertical_span(el);
                if !in_band(sy, ey, band_top, band_bottom) {
                    continue;
                }
                if el.display == css::Display::None {
                    continue;
                }
                if el.visibility.as_deref() == Some("hidden") {
                    continue;
                }

                profile.elements_drawn += 1;

                if let Some(ref handle) = el.image_handle {
                    let img_start = Instant::now();
                    let iw = if el.width.is_finite() && el.width > 0.0 {
                        el.width
                    } else {
                        50.0
                    };
                    let ih = if el.height.is_finite() && el.height > 0.0 {
                        el.height
                    } else {
                        50.0
                    };
                    let ix = el.x.max(0.0);
                    let iy = el.y.max(0.0);
                    if ix.is_finite() && iy.is_finite() && iw.is_finite() && ih.is_finite() {
                        frame.draw_image(
                            Rectangle::new(Point::new(ix, iy), Size::new(iw, ih)),
                            CanvasImage::new(handle.clone()),
                        );
                    }
                    profile.image_ms += img_start.elapsed().as_secs_f64() * 1000.0;
                } else if matches!(el.tag.as_str(), "input" | "textarea" | "select" | "button") {
                    let form_start = Instant::now();
                    let ex = if el.x.is_finite() { el.x.max(0.0) } else { 0.0 };
                    let ey = if el.y.is_finite() { el.y.max(0.0) } else { 0.0 };
                    let ew = if el.width.is_finite() {
                        el.width.max(60.0)
                    } else {
                        200.0
                    };
                    let eh = if el.height > 0.0 && el.height.is_finite() {
                        el.height
                    } else {
                        32.0
                    };
                    let border_col = el
                        .border_color
                        .unwrap_or(iced::Color::from_rgb(0.7, 0.7, 0.7));
                    let bg_col = if el.tag == "button" {
                        iced::Color::from_rgb(0.92, 0.92, 0.92)
                    } else {
                        iced::Color::WHITE
                    };
                    frame.fill_rectangle(Point::new(ex, ey), Size::new(ew, eh), bg_col);
                    frame.stroke_rectangle(
                        Point::new(ex, ey),
                        Size::new(ew, eh),
                        iced::widget::canvas::Stroke::default()
                            .with_color(border_col)
                            .with_width(1.0),
                    );
                    if self.focused_index == Some(i) {
                        frame.stroke_rectangle(
                            Point::new(ex - 2.0, ey - 2.0),
                            Size::new(ew + 4.0, eh + 4.0),
                            iced::widget::canvas::Stroke::default()
                                .with_color(iced::Color::from_rgb(0.2, 0.5, 1.0))
                                .with_width(2.0),
                        );
                    }
                    let fs = if el.font_size.is_finite() {
                        el.font_size.clamp(8.0, 64.0)
                    } else {
                        14.0
                    };
                    let label = if el.text.is_empty() {
                        if el.tag == "button" {
                            "Button".to_string()
                        } else if el.tag == "select" {
                            "â–¾ Select".to_string()
                        } else {
                            "".to_string()
                        }
                    } else {
                        el.text.clone()
                    };
                    if !label.is_empty() {
                        frame.fill_text(iced::widget::canvas::Text {
                            content: label,
                            position: Point::new(ex + 4.0, ey + (eh - fs) / 2.0),
                            color: el.color,
                            size: iced::Pixels(fs),
                            ..Default::default()
                        });
                    }
                    profile.form_ms += form_start.elapsed().as_secs_f64() * 1000.0;
                } else {
                    let box_start = Instant::now();
                    let bg = if matches!(el.tag.as_str(), "body" | "html") {
                        None
                    } else {
                        el.background_color
                    };
                    let bw = el.border_widths;
                    let bc = el.border_color;
                    let ex = if el.x.is_finite() { el.x.max(0.0) } else { 0.0 };
                    let ey = if el.y.is_finite() { el.y.max(0.0) } else { 0.0 };
                    let ew = if el.width.is_finite() {
                        el.width.max(1.0)
                    } else {
                        1.0
                    };
                    let eh = if el.height > 0.0 && el.height.is_finite() {
                        el.height
                    } else {
                        let f = if el.font_size.is_finite() {
                            el.font_size.clamp(6.0, 200.0)
                        } else {
                            16.0
                        };
                        f * el.line_height.max(1.0)
                    };
                    if bg.is_some() || bc.is_some() {
                        let fill = bg.unwrap_or(iced::Color::TRANSPARENT);
                        frame.fill_rectangle(Point::new(ex, ey), Size::new(ew, eh), fill);
                    }
                    if let Some(color) = bc {
                        if bw[0] > 0.0 {
                            frame.fill_rectangle(Point::new(ex, ey), Size::new(ew, bw[0]), color);
                        }
                        if bw[2] > 0.0 {
                            frame.fill_rectangle(
                                Point::new(ex, ey + eh - bw[2]),
                                Size::new(ew, bw[2]),
                                color,
                            );
                        }
                        if bw[3] > 0.0 {
                            frame.fill_rectangle(Point::new(ex, ey), Size::new(bw[3], eh), color);
                        }
                        if bw[1] > 0.0 {
                            frame.fill_rectangle(
                                Point::new(ex + ew - bw[1], ey),
                                Size::new(bw[1], eh),
                                color,
                            );
                        }
                    }
                    profile.box_ms += box_start.elapsed().as_secs_f64() * 1000.0;

                    let text_start = Instant::now();
                    let weight = match el.font_weight {
                        FontWeight::Bold => iced::font::Weight::Bold,
                        FontWeight::Normal => iced::font::Weight::Normal,
                    };
                    let fs = if el.font_size.is_finite() {
                        el.font_size.clamp(6.0, 200.0)
                    } else {
                        16.0
                    };
                    let line_h = fs * el.line_height.max(1.0);
                    let px0 = el.x.max(0.0) + bw[3];
                    let py0 = el.y.max(0.0) + bw[0];
                    let lines: Vec<&str> = if el.wrapped_lines.is_empty() {
                        vec![&el.text]
                    } else {
                        el.wrapped_lines.iter().map(|s| s.as_str()).collect()
                    };
                    for (li, line) in lines.iter().enumerate() {
                        let py = py0 + li as f32 * line_h;
                        if fs.is_finite() && px0.is_finite() && py.is_finite() && !line.is_empty() {
                            frame.fill_text(iced::widget::canvas::Text {
                                content: line.to_string(),
                                position: Point::new(px0, py),
                                color: el.color,
                                size: iced::Pixels(fs),
                                font: iced::Font {
                                    weight,
                                    ..Default::default()
                                },
                                shaping: iced::widget::text::Shaping::Advanced,
                                ..Default::default()
                            });
                            let deco_y = py;
                            let deco_h = (fs * 0.06).max(1.0);
                            let deco_w = measure_text_width(line, fs);
                            if el.text_decoration.contains(TextDecor::UNDERLINE) {
                                frame.fill_rectangle(
                                    Point::new(px0, deco_y + fs * 0.1),
                                    Size::new(deco_w, deco_h),
                                    el.color,
                                );
                            }
                            if el.text_decoration.contains(TextDecor::LINE_THROUGH) {
                                frame.fill_rectangle(
                                    Point::new(px0, deco_y - fs * 0.35),
                                    Size::new(deco_w, deco_h),
                                    el.color,
                                );
                            }
                            if el.text_decoration.contains(TextDecor::OVERLINE) {
                                frame.fill_rectangle(
                                    Point::new(px0, deco_y - fs * 0.75),
                                    Size::new(deco_w, deco_h),
                                    el.color,
                                );
                            }
                        }
                    }
                    profile.text_ms += text_start.elapsed().as_secs_f64() * 1000.0;
                }
            }

            profile.geometry_ms = geom_start.elapsed().as_secs_f64() * 1000.0;
        });

        let geometry = vec![geometry];

        profile.cache_hit = *cache_miss.borrow();
        profile.total_ms = total_start.elapsed().as_secs_f64() * 1000.0;
        record_paint(profile);

        geometry
    }

    fn update(
        &self,
        _state: &mut (),
        event: iced::widget::canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (iced::widget::canvas::event::Status, Option<BrowserMessage>) {
        use iced::widget::canvas::event;
        if let iced::widget::canvas::Event::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        )) = event
        {
            if let Some(pos) = cursor.position_in(bounds) {
                plog!("CLICK", "Click at pos=({:.0},{:.0})", pos.x, pos.y);
                let band_top = self.scroll_top;
                let band_bottom = self.scroll_top + self.viewport_h;
                for pos_idx in self.band_window(band_top, band_bottom).rev() {
                    let i = self.cull.order[pos_idx] as usize;
                    let el = &self.elements[i];
                    let (sy, ey) = painted_vertical_span(el);
                    if !in_band(sy, ey, band_top, band_bottom) {
                        continue;
                    }
                    if el.display == css::Display::None {
                        continue;
                    }
                    if el.visibility.as_deref() == Some("hidden") {
                        continue;
                    }
                    if el.is_link {
                        let text_w = el.width.max(el.font_size);
                        let ex = el.x.max(0.0);
                        let ey = el.y.max(0.0);
                        let hit = Rectangle::new(
                            Point::new(ex, ey),
                            Size::new(text_w, el.font_size + 4.0),
                        );
                        if hit.contains(pos) {
                            plog!("CLICK", "Link hit at element {} href={:?}", i, el.href);
                            if let Some(ref href) = el.href {
                                return (
                                    event::Status::Captured,
                                    Some(BrowserMessage::LinkClicked(href.clone())),
                                );
                            }
                        }
                    }
                    let ex = el.x.max(0.0);
                    let ey = el.y.max(0.0);
                    let ew = if el.width.is_finite() {
                        el.width.max(1.0)
                    } else {
                        200.0
                    };
                    let eh = if el.height > 0.0 && el.height.is_finite() {
                        el.height
                    } else {
                        el.font_size.clamp(6.0, 200.0) * el.line_height.max(1.0)
                    };
                    let hit = Rectangle::new(Point::new(ex, ey), Size::new(ew, eh));
                    if hit.contains(pos) {
                        plog!(
                            "CLICK",
                            "Element {} hit at [{:.0},{:.0} {:.0}x{:.0}] tag={}",
                            i,
                            ex,
                            ey,
                            ew,
                            eh,
                            el.tag
                        );
                        let msg = if matches!(
                            el.tag.as_str(),
                            "input" | "textarea" | "select" | "button"
                        ) {
                            BrowserMessage::FormElementClicked(i)
                        } else {
                            BrowserMessage::ElementClicked(i)
                        };
                        return (event::Status::Captured, Some(msg));
                    }
                }
            }
        }
        (event::Status::Ignored, None)
    }
}

#[cfg(test)]
mod cull_tests {
    use super::{in_band, CullIndex};

    const TOP: f32 = 1000.0;
    const BOTTOM: f32 = 2000.0;

    #[test]
    fn empty_spans_empty_window() {
        let idx = CullIndex::from_spans(&[]);
        assert_eq!(idx.window(TOP, BOTTOM), 0..0);
    }

    #[test]
    fn inverted_band_empty_window() {
        let idx = CullIndex::from_spans(&[(500.0, 600.0), (1500.0, 1600.0)]);
        assert_eq!(idx.window(BOTTOM, TOP), 0..0);
        assert!(!in_band(1500.0, 1600.0, BOTTOM, TOP));
    }

    // Boundary contract: half-open [top, bottom) — ending exactly at the top
    // edge is culled, starting exactly at the bottom edge is culled.
    #[test]
    fn end_exactly_at_top_is_culled() {
        assert!(!in_band(900.0, TOP, TOP, BOTTOM));
    }

    #[test]
    fn start_exactly_at_bottom_is_culled() {
        assert!(!in_band(BOTTOM, BOTTOM + 50.0, TOP, BOTTOM));
    }

    #[test]
    fn element_spanning_full_viewport_is_included() {
        let idx = CullIndex::from_spans(&[(0.0, 5000.0)]);
        assert!(in_band(0.0, 5000.0, TOP, BOTTOM));
        assert_eq!(idx.window(TOP, BOTTOM), 0..1);
    }

    // Equal start values must resolve deterministically: stable sort keeps
    // input order, so repeated builds yield identical windows.
    #[test]
    fn equal_starts_deterministic() {
        let spans = vec![(1500.0, 1520.0), (1500.0, 1510.0), (1500.0, 1530.0)];
        let a = CullIndex::from_spans(&spans);
        let b = CullIndex::from_spans(&spans);
        assert_eq!(a.window(TOP, BOTTOM), b.window(TOP, BOTTOM));
    }

    // Zero-height elements have defined behavior: visible only when the
    // start lies strictly inside the band.
    #[test]
    fn zero_height_strictly_inside_is_visible() {
        let mid = (TOP + BOTTOM) / 2.0;
        assert!(in_band(mid, mid, TOP, BOTTOM));
    }

    #[test]
    fn zero_height_at_edges_is_culled() {
        assert!(!in_band(TOP, TOP, TOP, BOTTOM));
        assert!(!in_band(BOTTOM, BOTTOM, TOP, BOTTOM));
    }

    #[test]
    fn non_finite_span_never_in_band() {
        assert!(!in_band(f32::NAN, 100.0, TOP, BOTTOM));
        assert!(!in_band(100.0, f32::INFINITY, TOP, BOTTOM));
    }

    // The window returned by the index must be a superset of the exact
    // predicate over ALL spans — including the hard case of a tall early
    // container followed by short spans above the viewport.
    #[test]
    fn window_superset_of_predicate_with_tall_early_container() {
        let mut spans: Vec<(f32, f32)> = vec![(0.0, 4000.0)];
        let mut y = 200.0;
        while y < 900.0 {
            spans.push((y, y + 20.0)); // above-band stragglers after the tall one
            y += 30.0;
        }
        spans.push((TOP + 5.0, TOP + 25.0)); // in band
        spans.push((BOTTOM - 5.0, BOTTOM + 500.0)); // crosses bottom edge
        spans.push((BOTTOM + 10.0, BOTTOM + 30.0)); // below band
        let idx = CullIndex::from_spans(&spans);
        let win = idx.window(TOP, BOTTOM);
        for (i, &(sy, ey)) in spans.iter().enumerate() {
            if in_band(sy, ey, TOP, BOTTOM) {
                assert!(
                    win.contains(&i),
                    "span {i} ({sy}..{ey}) passes the predicate but sits outside window {win:?}"
                );
            }
        }
        assert!(win.end <= spans.len());
    }

    // D3 profiling infrastructure test
    #[test]
    fn d3_profiling_infrastructure() {
        use crate::ui::screens::browser::canvas::{
            record_canvas_invalidation, CACHE_INVALIDATIONS, LAST_INVALIDATION_REASON,
        };

        // Reset thread-local state
        CACHE_INVALIDATIONS.with(|c| *c.borrow_mut() = 0);
        LAST_INVALIDATION_REASON.with(|r| *r.borrow_mut() = None);

        // Record some invalidations
        record_canvas_invalidation("test_reason_1");
        record_canvas_invalidation("test_reason_2");
        record_canvas_invalidation("test_reason_1");

        let count = CACHE_INVALIDATIONS.with(|c| *c.borrow());
        assert_eq!(count, 3);

        let reason = LAST_INVALIDATION_REASON.with(|r| *r.borrow());
        assert_eq!(reason, Some("test_reason_1"));
    }
}
