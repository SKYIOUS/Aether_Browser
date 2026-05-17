use iced::widget::{
    button, column, container, row, scrollable, text, text_input, Space,
};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Task};
use crate::ui::style::*;

fn stratus_color(c: &crate::engine::stratus::Color) -> Color {
    Color::from_rgba(c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0, c.a as f32 / 255.0)
}

// ── Module-level helpers (lifted from navigate) ─────────────────────────────

fn extract_styles(node: &crate::engine::dom::Node, styles: &mut Vec<String>) {
    match &node.node_type {
        NodeType::Element(elem) => {
            if elem.tag_name.to_lowercase() == "style" {
                for child in &node.children {
                    if let NodeType::Text(text) = &child.node_type {
                        styles.push(text.clone());
                    }
                }
            }
            for child in &node.children {
                extract_styles(child, styles);
            }
        }
        _ => {}
    }
}

fn extract_links(node: &crate::engine::dom::Node, links: &mut Vec<String>) {
    match &node.node_type {
        NodeType::Element(elem) => {
            if elem.tag_name.to_lowercase() == "link" {
                if let Some(rel) = elem.attributes.get("rel") {
                    if rel.contains("stylesheet") {
                        if let Some(href) = elem.attributes.get("href") {
                            links.push(href.clone());
                        }
                    }
                }
            }
            for child in &node.children {
                extract_links(child, links);
            }
        }
        _ => {}
    }
}

fn should_skip_tag(tag: &str) -> bool {
    matches!(tag, "script" | "style" | "noscript" | "meta" | "link" | "head" | "svg" | "path" | "br" | "hr" | "input" | "button" | "iframe" | "textarea" | "select" | "option" | "form" | "img" | "template")
}

fn should_skip_content(tag: &str) -> bool {
    matches!(tag, "script" | "style" | "noscript" | "template" | "svg")
}

fn get_all_text(node: &crate::engine::dom::Node) -> String {
    match &node.node_type {
        NodeType::Text(t) => t.trim().to_string(),
        NodeType::Element(_) => {
            node.children.iter()
                .map(get_all_text)
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        }
        NodeType::Document | NodeType::Comment(_) => String::new(),
    }
}

fn style_for_node(node: &crate::engine::dom::Node, ss: &crate::engine::css::Stylesheet) -> (Color, f32, String) {
    let cs = crate::engine::css::compute_style(node, ss);
    let c = cs.color.as_ref().map(stratus_color).unwrap_or(C::PAGE_TEXT);
    let f = cs.font_size.unwrap_or(16.0);
    let w = cs.font_weight.unwrap_or_else(|| "normal".to_string());
    (c, f, w)
}

fn extract_elements(
    node: &crate::engine::dom::Node,
    elements: &mut Vec<StyledElement>,
    depth: usize,
    ss: &crate::engine::css::Stylesheet,
    parent_style: Option<(Color, f32, String)>,
) {
    if depth > 30 || elements.len() >= 300 { return; }

    match &node.node_type {
        NodeType::Document | NodeType::Comment(_) => { return; }
        NodeType::Text(text) => {
            let txt = text.trim();
            if !txt.is_empty() && txt.len() < 1000 && !txt.chars().all(|c| c.is_whitespace()) {
                let (color, font_size, font_weight) = parent_style.unwrap_or_else(|| style_for_node(node, ss));
                elements.push(StyledElement {
                    tag: "text".to_string(),
                    text: txt.to_string(),
                    is_link: false,
                    href: None,
                    indent_level: 0,
                    color, font_size, font_weight,
                });
            }
        }
        NodeType::Element(elem) => {
            let tag = elem.tag_name.to_lowercase();
            if should_skip_tag(&tag) {
                if !should_skip_content(&tag) && tag != "head" && tag != "meta" && tag != "link" {
                    for child in &node.children {
                        extract_elements(child, elements, depth + 1, ss, parent_style.clone());
                    }
                }
                return;
            }

            if tag == "a" {
                let href = elem.attributes.get("href").cloned();
                let text = get_all_text(node);
                if !text.is_empty() {
                    let (color, font_size, font_weight) = style_for_node(node, ss);
                    elements.push(StyledElement {
                        tag: "a".to_string(),
                        text,
                        is_link: true,
                        href,
                        indent_level: 0,
                        color, font_size, font_weight,
                    });
                }
                return;
            }

            if tag == "h1" {
                let text = get_all_text(node);
                if !text.is_empty() {
                    let (color, font_size, font_weight) = style_for_node(node, ss);
                    elements.push(StyledElement { tag: "h1".to_string(), text, is_link: false, href: None, indent_level: 0, color, font_size, font_weight });
                }
                return;
            }
            if tag == "h2" {
                let text = get_all_text(node);
                if !text.is_empty() {
                    let (color, font_size, font_weight) = style_for_node(node, ss);
                    elements.push(StyledElement { tag: "h2".to_string(), text, is_link: false, href: None, indent_level: 0, color, font_size, font_weight });
                }
                return;
            }
            if ["h3","h4","h5","h6"].contains(&tag.as_str()) {
                let text = get_all_text(node);
                if !text.is_empty() {
                    let (color, font_size, font_weight) = style_for_node(node, ss);
                    elements.push(StyledElement { tag: tag.clone(), text, is_link: false, href: None, indent_level: 1, color, font_size, font_weight });
                }
                return;
            }

            if tag == "p" {
                let direct_text: String = node.children.iter()
                    .filter_map(|c| {
                        if let NodeType::Text(t) = &c.node_type {
                            let txt = t.trim().to_string();
                            if !txt.is_empty() { Some(txt) } else { None }
                        } else { None }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                if !direct_text.is_empty() {
                    let (color, font_size, font_weight) = style_for_node(node, ss);
                    elements.push(StyledElement { tag: "p".to_string(), text: direct_text, is_link: false, href: None, indent_level: 0, color, font_size, font_weight });
                }
                for child in &node.children {
                    if !matches!(child.node_type, NodeType::Text(_)) {
                        extract_elements(child, elements, depth + 1, ss, parent_style.clone());
                    }
                }
                return;
            }

            if tag == "li" {
                let has_link = node.children.iter().any(|c| {
                    if let NodeType::Element(e) = &c.node_type {
                        e.tag_name.to_lowercase() == "a"
                    } else { false }
                });
                if !has_link {
                    let text = get_all_text(node);
                    if !text.is_empty() {
                        let (color, font_size, font_weight) = style_for_node(node, ss);
                        elements.push(StyledElement { tag: "li".to_string(), text: format!("• {}", text), is_link: false, href: None, indent_level: 1, color, font_size, font_weight });
                    }
                }
                return;
            }

            let our_style = Some(style_for_node(node, ss));
            for child in &node.children {
                extract_elements(child, elements, depth + 1, ss, our_style.clone());
            }
        }
    }
}

async fn fetch_page_content(url: String) -> (String, Vec<StyledElement>) {
    use crate::engine::net::fetch;
    use crate::engine::parser::Parser;

    let html = fetch(&url);

    if html.starts_with("Error") || html.is_empty() {
        return (url, vec![]);
    }

    let max_html = 500_000;
    let html = if html.len() > max_html {
        html[..max_html].to_string()
    } else {
        html
    };

    let mut parser = Parser::new(html);
    let dom_node = parser.parse_node();

    let mut styles = Vec::new();
    extract_styles(&dom_node, &mut styles);

    let mut stylesheet = crate::engine::css::Stylesheet { rules: Vec::new() };
    let style_limit = styles.len().min(1);
    for style_content in styles.iter().take(style_limit) {
        let max_css_len = 50_000;
        let trimmed = if style_content.len() > max_css_len {
            &style_content[..max_css_len]
        } else {
            style_content.as_str()
        };
        let mut parser = crate::engine::css::Parser::new(trimmed.to_string());
        stylesheet.rules.extend(parser.parse_rules());
    }

    let mut link_urls = Vec::new();
    extract_links(&dom_node, &mut link_urls);
    let link_limit = link_urls.len().min(1);
    for link_url in link_urls.iter().take(link_limit) {
        let resolved = crate::engine::net::resolve_url(link_url, &url);
        let css_content = crate::engine::net::fetch(&resolved);
        if !css_content.starts_with("Error") && !css_content.is_empty() {
            let max_css_len = 50_000;
            let trimmed = if css_content.len() > max_css_len {
                css_content[..max_css_len].to_string()
            } else {
                css_content
            };
            let mut parser = crate::engine::css::Parser::new(trimmed);
            stylesheet.rules.extend(parser.parse_rules());
        }
    }

    let mut elements = Vec::new();
    extract_elements(&dom_node, &mut elements, 0, &stylesheet, None);
    elements.truncate(300);

    (url, elements)
}

// ── Messages ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum BrowserMessage {
    UrlChanged(String),
    UrlSubmit,
    NavBack,
    NavForward,
    Refresh,
    WorkspaceSelected(usize),
    OpenSettings,
    OpenPalette,
    PillAction(usize),
    LinkClicked(String),
    PageLoaded(String, Vec<StyledElement>),
}

// ── Workspace / Collection data ───────────────────────────────────────────────

const WORKSPACES: [(&str, &str); 3] = [
    ("⬡", "Design Studio"),
    ("⬡", "Research Lab"),
    ("⬡", "Deep Work"),
];

const COLLECTIONS: [(&str, &str); 2] = [
    ("▤", "Aether UI"),
    ("▤", "Rust / Iced Docs"),
];

const PILL_ICONS: [(&str, bool); 4] = [
    ("✦", false),
    ("◈", false),
    ("⬡", false),
    ("◫", true),
];

use crate::engine::dom::NodeType;

// ── State ─────────────────────────────────────────────────────────────────────

pub struct BrowserScreen {
    pub url: String,
    pub active_workspace: usize,
    pub content: String,
    pub dom_root: Option<Box<crate::engine::dom::Node>>,
    pub styled_elements: Vec<StyledElement>,
    pub loading: bool,
}

#[derive(Debug, Clone)]
pub struct StyledElement {
    pub tag: String,
    pub text: String,
    pub is_link: bool,
    pub href: Option<String>,
    pub indent_level: usize,
    pub color: Color,
    pub font_size: f32,
    pub font_weight: String,
}

impl BrowserScreen {
    pub fn new() -> Self {
        Self {
            url: String::from("aether://design/spatial-minimalism"),
            active_workspace: 0,
            content: "Welcome to Aether Browser".to_string(),
            dom_root: None,
            styled_elements: vec![],
            loading: false,
        }
    }

    pub fn update(&mut self, msg: BrowserMessage) -> Task<BrowserMessage> {
        match msg {
            BrowserMessage::UrlChanged(s) => { self.url = s; Task::none() }
            BrowserMessage::UrlSubmit => {
                self.loading = true;
                Task::perform(fetch_page_content(self.url.clone()), |(u, els)| BrowserMessage::PageLoaded(u, els))
            }
            BrowserMessage::LinkClicked(url) => {
                let target = if url.starts_with("http") { url } else { format!("https://{}", url) };
                self.url = target.clone();
                self.loading = true;
                Task::perform(fetch_page_content(target), |(u, els)| BrowserMessage::PageLoaded(u, els))
            }
            BrowserMessage::Refresh => {
                self.loading = true;
                Task::perform(fetch_page_content(self.url.clone()), |(u, els)| BrowserMessage::PageLoaded(u, els))
            }
            BrowserMessage::PageLoaded(_url, elements) => {
                self.loading = false;
                let count = elements.len();
                self.styled_elements = elements;
                self.content = format!("Loaded ({} elements)", count);
                Task::none()
            }
            BrowserMessage::WorkspaceSelected(i) => { self.active_workspace = i; Task::none() }
            _ => Task::none(),
        }
    }

    // ── View ──────────────────────────────────────────────────────────────────

    pub fn view(&self) -> Element<'_, BrowserMessage> {
        let sidebar = self.sidebar();
        let main = self.main_area();

        row![sidebar, main].into()
    }

    // ── Main Area ─────────────────────────────────────────────────────────────

    fn main_area(&self) -> Element<'_, BrowserMessage> {
        let top = self.top_bar();
        let status = self.status_bar();

            let body: Element<'_, BrowserMessage> = if self.loading {
            container(
                column![
                    text("Loading...").size(20).color(C::PAGE_MUTED),
                    text("Fetching page content").size(13).color(C::DIM),
                ]
                .align_x(Alignment::Center)
                .spacing(8)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(C::PAGE_BG)),
                ..Default::default()
            })
            .into()
        } else if !self.styled_elements.is_empty() {
            let lines: Vec<Element<'_, BrowserMessage>> = self.styled_elements
                .iter()
                .map(|el| {
                    let weight = if el.font_weight == "bold" { iced::font::Weight::Bold } else { iced::font::Weight::Normal };

                    let content: Element<'_, BrowserMessage> = if el.is_link {
                        let href = el.href.clone().unwrap_or_default();
                        let link_bg = Color::from_rgba(0.02, 0.33, 0.75, 0.08);
                        button(text(&el.text).size(el.font_size).color(el.color).font(iced::Font { weight, ..Default::default() }))
                            .style(move |_, status| {
                                let bg = match status {
                                    button::Status::Hovered | button::Status::Pressed => Some(Background::Color(link_bg)),
                                    _ => None,
                                };
                                button::Style {
                                    background: bg,
                                    text_color: el.color,
                                    border: Border::default(),
                                    ..Default::default()
                                }
                            })
                            .padding(0)
                            .on_press(BrowserMessage::LinkClicked(href))
                            .into()
                    } else {
                        text(&el.text).size(el.font_size).color(el.color).font(iced::Font { weight, ..Default::default() }).into()
                    };

                    let top: f32 = match el.tag.as_str() {
                        "h1" => 24.0,
                        "h2" => 20.0,
                        "h3" | "h4" | "h5" | "h6" => 16.0,
                        "p" => 12.0,
                        "li" => 8.0,
                        "a" => 4.0,
                        _ => 2.0,
                    };
                    container(content).padding(Padding { top, right: 0.0, bottom: 0.0, left: 0.0 }).into()
                })
                .collect();

            container(
                scrollable(
                    column(lines)
                        .spacing(0)
                        .padding(40)
                        .max_width(800)
                )
                .width(Length::Fill)
                .height(Length::Fill)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(C::PAGE_BG)),
                ..Default::default()
            })
            .into()
        } else {
            container(
                scrollable(
                    column(
                        vec![
                            text(&self.content).size(14).color(C::PAGE_TEXT).into()
                        ]
                    )
                    .padding(40)
                    .max_width(800)
                )
                .width(Length::Fill)
                .height(Length::Fill)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(C::PAGE_BG)),
                ..Default::default()
            })
            .into()
        };

        container(column![top, body, status])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(main_area_style())
            .into()
    }

    // ── Sidebar ───────────────────────────────────────────────────────────────

    fn sidebar(&self) -> Element<'_, BrowserMessage> {
        // Logo
        let logo = row![
            container(
                text("⬡").size(18).color(C::ACCENT)
            )
            .width(28).height(28)
            .center_x(Length::Fixed(28.0))
            .center_y(Length::Fixed(28.0))
            .style(|_| container::Style {
                background: Some(iced::Background::Color(
                    iced::Color::from_rgba(1.0, 1.0, 1.0, 0.07)
                )),
                border: iced::Border { radius: 8.0.into(), ..Default::default() },
                ..Default::default()
            }),
            text("AETHER").size(16).color(C::FG)
                .font(iced::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        // Workspaces
        let ws_label = text("WORKSPACES")
            .size(10)
            .color(C::DIM)
            .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() });

        let workspaces = column(
            WORKSPACES.iter().enumerate().map(|(i, (icon, name))| {
                let active = i == self.active_workspace;
                let label = row![
                    text(*icon).size(16).color(if active { C::ACCENT } else { C::MUTED }),
                    text(*name).size(13).color(if active { C::ACCENT } else { C::MUTED }),
                ]
                .spacing(12)
                .align_y(Alignment::Center);

                button(label)
                    .padding([10, 16])
                    .width(Length::Fill)
                    .style(sidebar_item_button_style(active))
                    .on_press(BrowserMessage::WorkspaceSelected(i))
                    .into()
            })
            .collect::<Vec<_>>(),
        )
        .spacing(2);

        // Collections
        let col_label = text("COLLECTIONS")
            .size(10)
            .color(C::DIM)
            .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() });

        let collections = column(
            COLLECTIONS.iter().map(|(icon, name)| {
                let label = row![
                    text(*icon).size(15).color(C::MUTED),
                    text(*name).size(13).color(C::MUTED),
                ]
                .spacing(12)
                .align_y(Alignment::Center);

                button(label)
                    .padding([8, 16])
                    .width(Length::Fill)
                    .style(sidebar_item_button_style(false))
                    .on_press(BrowserMessage::NavBack)
                    .into()
            })
            .collect::<Vec<_>>(),
        )
        .spacing(2);

        // Bottom links
        let bottom = column![
            button(
                row![text("⏱").size(16).color(C::MUTED), text("History").size(13).color(C::MUTED)]
                    .spacing(12).align_y(Alignment::Center)
            )
            .padding([8, 0])
            .width(Length::Fill)
            .style(ghost_button_style())
            .on_press(BrowserMessage::NavBack),

            button(
                row![text("⚙").size(16).color(C::MUTED), text("Settings").size(13).color(C::MUTED)]
                    .spacing(12).align_y(Alignment::Center)
            )
            .padding([8, 0])
            .width(Length::Fill)
            .style(ghost_button_style())
            .on_press(BrowserMessage::OpenSettings),
        ]
        .spacing(8);

        let content = column![
            logo,
            Space::with_height(16),
            ws_label,
            Space::with_height(8),
            workspaces,
            Space::with_height(24),
            col_label,
            Space::with_height(8),
            collections,
            Space::with_height(Length::Fill),
            bottom,
        ]
        .padding([32, 24])
        .spacing(0)
        .height(Length::Fill);

        container(content)
            .width(Length::Fixed(260.0))
            .height(Length::Fill)
            .style(sidebar_style())
            .into()
    }

    fn top_bar(&self) -> Element<'_, BrowserMessage> {
        let nav = row![
            button(text("←").size(16).color(C::MUTED))
                .padding([6, 8]).style(nav_icon_button_style())
                .on_press(BrowserMessage::NavBack),
            button(text("→").size(16).color(C::MUTED))
                .padding([6, 8]).style(nav_icon_button_style())
                .on_press(BrowserMessage::NavForward),
            button(text("↻").size(16).color(C::MUTED))
                .padding([6, 8]).style(nav_icon_button_style())
                .on_press(BrowserMessage::Refresh),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        let url_bar = container(
            row![
                text("⌕").size(16).color(C::MUTED),
                text_input("Search or navigate", &self.url)
                    .on_input(BrowserMessage::UrlChanged)
                    .on_submit(BrowserMessage::UrlSubmit)
                    .size(13)
                    .style(url_input_style()),
                text("☆").size(14).color(C::DIM),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
            .padding([0, 16])
        )
        .height(Length::Fixed(40.0))
        .width(Length::Fill)
        .max_width(600.0)
        .center_y(Length::Fixed(40.0))
        .style(url_bar_style());

        let right_icons = row![
            button(text("⊞").size(18).color(C::MUTED))
                .padding([6, 8]).style(nav_icon_button_style())
                .on_press(BrowserMessage::OpenPalette),
            container(text("A").size(13).color(C::ACCENT))
                .width(32).height(32)
                .center_x(Length::Fixed(32.0))
                .center_y(Length::Fixed(32.0))
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(
                        iced::Color::from_rgba(1.0, 1.0, 1.0, 0.08)
                    )),
                    border: iced::Border { radius: 999.0.into(), color: C::BORDER_MID, width: 1.0 },
                    ..Default::default()
                }),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let bar = container(
            row![nav, url_bar, right_icons]
                .spacing(16)
                .align_y(Alignment::Center)
                .padding([0, 40])
        )
        .height(Length::Fixed(64.0))
        .width(Length::Fill)
        .center_y(Length::Fixed(64.0))
        .style(|_| container::Style {
            background: None,
            border: iced::Border { color: C::BORDER, width: 0.0, radius: 0.0.into() },
            ..Default::default()
        });

        container(column![
            bar,
            container(Space::with_height(1.0))
                .width(Length::Fill)
                .height(Length::Fixed(1.0))
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(C::BORDER)),
                    ..Default::default()
                }),
        ])
        .width(Length::Fill)
        .into()
    }

    fn status_bar(&self) -> Element<'_, BrowserMessage> {
        let dot = text(" · ").size(10).color(C::DIM);

        container(
            row![
                text("Secure Core").size(10).color(C::DIM)
                    .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
                dot,
                text("Flow Active").size(10).color(C::DIM)
                    .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
                text(" · ").size(10).color(C::DIM),
                text("1.2ms Latency").size(10).color(C::DIM)
                    .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
        )
        .height(Length::Fixed(40.0))
        .width(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fixed(40.0))
        .style(status_bar_style())
        .into()
    }
}
