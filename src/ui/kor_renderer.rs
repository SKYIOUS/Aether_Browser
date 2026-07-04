use korlang::vm::{Value, KorObject, VirtualMachine};
use iced::widget::{button, column, row, text, text_input, container, Space};
use crate::ui::screens::browser::BrowserMessage;
use iced::{Length, Alignment, Color, Background, Border};
use std::sync::Arc;

pub fn render_kor_vm(vm: &VirtualMachine) -> iced::Element<'static, BrowserMessage> {
    if let Some(Value::Object(root)) = vm.stack.last() {
        convert_object(root.clone(), vm)
    } else {
        text("Empty Korlang UI").into()
    }
}

fn convert_object(obj_arc: Arc<std::sync::Mutex<KorObject>>, vm: &VirtualMachine) -> iced::Element<'static, BrowserMessage> {
    let obj = obj_arc.lock().unwrap_or_else(|e| e.into_inner());
    match obj.tag.as_str() {
        "Row" | "NavBar" => {
            let spacing = get_number(&obj.properties, "spacing").unwrap_or(0.0);
            let padding = get_number(&obj.properties, "padding").unwrap_or(0.0);
            let mut r = row![].spacing(spacing).padding(padding).align_y(Alignment::Center);
            for child in &obj.children {
                if let Value::Object(c) = child { r = r.push(convert_object(c.clone(), vm)); }
            }
            r.into()
        }
        "Column" => {
            let spacing = get_number(&obj.properties, "spacing").unwrap_or(0.0);
            let padding = get_number(&obj.properties, "padding").unwrap_or(0.0);
            let width = get_number(&obj.properties, "width").map(Length::Fixed).unwrap_or(Length::Shrink);
            let mut c = column![].spacing(spacing).padding(padding).width(width);
            for child in &obj.children {
                if let Value::Object(ch) = child { c = c.push(convert_object(ch.clone(), vm)); }
            }
            c.into()
        }
        "Container" => {
            let padding = get_number(&obj.properties, "padding").unwrap_or(0.0);
            let radius = get_number(&obj.properties, "radius").unwrap_or(0.0);
            let bg_hex = get_string(&obj.properties, "background");
            let bg_color = bg_hex.and_then(|s| hex_to_color(&s)).unwrap_or(Color::TRANSPARENT);

            let mut inner: iced::Element<BrowserMessage> = column![].into();
            if let Some(Value::Object(c)) = obj.children.first() {
                inner = convert_object(c.clone(), vm);
            }

            container(inner)
                .padding(padding)
                .style(move |_| container::Style {
                    background: Some(Background::Color(bg_color)),
                    border: Border { radius: radius.into(), ..Default::default() },
                    ..Default::default()
                })
                .into()
        }
        "Button" => {
            let label = if let Some(Value::String(s)) = obj.properties.get("text") { s.clone() } else { "Button".to_string() };
            let mut b = button(text(label).size(14)).padding([10, 20]);
            if let Some(Value::String(handler)) = obj.properties.get("on_click") {
                match handler.as_str() {
                    "back" => b = b.on_press(BrowserMessage::NavBack),
                    "forward" => b = b.on_press(BrowserMessage::NavForward),
                    "reload" => b = b.on_press(BrowserMessage::Refresh),
                    "refresh" => b = b.on_press(BrowserMessage::Refresh),
                    "settings" => b = b.on_press(BrowserMessage::OpenSettings),
                    "palette" => b = b.on_press(BrowserMessage::OpenPalette),
                    "bookmark" => b = b.on_press(BrowserMessage::Bookmark),
                    "ws0" => b = b.on_press(BrowserMessage::WorkspaceSelected(0)),
                    "ws1" => b = b.on_press(BrowserMessage::WorkspaceSelected(1)),
                    "ws2" => b = b.on_press(BrowserMessage::WorkspaceSelected(2)),
                    unknown => {
                        eprintln!("kor_renderer: unknown handler '{}'", unknown);
                        b = b.on_press(BrowserMessage::None);
                    }
                }
            }
            b.into()
        }
        "Text" => {
            let content = if let Some(Value::String(s)) = obj.properties.get("text") { s.clone() } else { "".to_string() };
            let size = get_number(&obj.properties, "size").unwrap_or(14.0);
            text(content).size(size).into()
        }
        "TextInput" => {
            let val = get_string(&obj.properties, "value")
                .or_else(|| vm.heap.get("url_input").and_then(|v| {
                    if let Value::String(s) = v { Some(s.clone()) } else { None }
                }))
                .unwrap_or_default();
            let ph = if let Some(Value::String(s)) = obj.properties.get("placeholder") { s.clone() } else { "Search...".to_string() };
            let width = get_number(&obj.properties, "width").map(Length::Fixed).unwrap_or(Length::Fill);
            text_input(&ph, &val).on_input(BrowserMessage::UrlChanged).on_submit(BrowserMessage::UrlSubmit).width(width).padding(12).into()
        }
        "If" => {
            let cond = obj.properties.get("condition").and_then(|v| {
                match v {
                    Value::Bool(b) => Some(*b),
                    _ => None,
                }
            }).unwrap_or(true);
            if cond {
                if let Some(Value::Object(c)) = obj.children.first() {
                    return convert_object(c.clone(), vm);
                }
            }
            column![].into()
        }
        "Space" => Space::with_height(Length::Fixed(get_number(&obj.properties, "height").unwrap_or(0.0))).into(),
        _ => text(format!("Unknown: {}", obj.tag)).into()
    }
}

fn get_number(props: &std::collections::HashMap<String, korlang::vm::Value>, name: &str) -> Option<f32> {
    if let Some(korlang::vm::Value::Number(n)) = props.get(name) { Some(*n as f32) } else { None }
}
fn get_string(props: &std::collections::HashMap<String, korlang::vm::Value>, name: &str) -> Option<String> {
    if let Some(korlang::vm::Value::String(s)) = props.get(name) { Some(s.clone()) } else { None }
}
fn hex_to_color(hex: &str) -> Option<Color> {
    if !hex.starts_with('#') || (hex.len() != 7 && hex.len() != 4) { return None; }
    if hex.len() == 7 {
        let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
        Some(Color::from_rgb8(r, g, b))
    } else {
        let r = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
        let g = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
        let b = u8::from_str_radix(&hex[3..4], 16).ok()? * 17;
        Some(Color::from_rgb8(r, g, b))
    }
}
