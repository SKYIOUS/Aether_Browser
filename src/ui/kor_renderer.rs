use korlang::vm::{Value, KorObject, VirtualMachine};
use iced::widget::{button, column, row, text, text_input, container, Space};
use crate::ui::screens::browser::BrowserMessage;
use iced::{Length, Alignment, Color, Background, Border, Shadow, Vector};

pub fn render_kor_vm(vm: &VirtualMachine) -> iced::Element<'static, BrowserMessage> {
    if let Some(Value::Object(root)) = vm.stack.last() {
        convert_object(root.clone())
    } else {
        text("Empty Korlang UI").into()
    }
}

fn convert_object(obj_arc: std::sync::Arc<std::sync::Mutex<KorObject>>) -> iced::Element<'static, BrowserMessage> {
    let obj = obj_arc.lock().unwrap();
    match obj.tag.as_str() {
        "Row" => {
            let spacing = get_number(&obj.properties, "spacing").unwrap_or(10.0);
            let padding = get_number(&obj.properties, "padding").unwrap_or(0.0);
            let mut r = row![].spacing(spacing).padding(padding).align_y(Alignment::Center);
            for child in &obj.children {
                if let Value::Object(c) = child { r = r.push(convert_object(c.clone())); }
            }
            r.into()
        }
        "Column" => {
            let spacing = get_number(&obj.properties, "spacing").unwrap_or(10.0);
            let padding = get_number(&obj.properties, "padding").unwrap_or(0.0);
            let width = get_number(&obj.properties, "width").map(Length::Fixed).unwrap_or(Length::Shrink);
            let mut c = column![].spacing(spacing).padding(padding).width(width);
            for child in &obj.children {
                if let Value::Object(ch) = child { c = c.push(convert_object(ch.clone())); }
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
                inner = convert_object(c.clone());
            }

            container(inner)
                .padding(padding)
                .style(move |_| container::Style {
                    background: Some(Background::Color(bg_color)),
                    border: Border { radius: radius.into(), ..Default::default() },
                    shadow: Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.1), offset: Vector::new(0.0, 2.0), blur_radius: 10.0 },
                    ..Default::default()
                })
                .into()
        }
        "Button" => {
            let label = if let Some(Value::String(s)) = obj.properties.get("text") { s.clone() } else { "Button".to_string() };
            let mut b = button(text(label).size(13)).padding([10, 20]);
            if let Some(Value::String(handler)) = obj.properties.get("on_click") {
                match handler.as_str() {
                    "back" => b = b.on_press(BrowserMessage::NavBack),
                    "forward" => b = b.on_press(BrowserMessage::NavForward),
                    "reload" => b = b.on_press(BrowserMessage::Refresh),
                    "settings" => b = b.on_press(BrowserMessage::OpenSettings),
                    _ => b = b.on_press(BrowserMessage::None),
                }
            } else { b = b.on_press(BrowserMessage::Refresh); }
            b.into()
        }
        "Text" => {
            let content = if let Some(Value::String(s)) = obj.properties.get("text") { s.clone() } else { "".to_string() };
            let size = get_number(&obj.properties, "size").unwrap_or(14.0);
            text(content).size(size).into()
        }
        "TextInput" => {
            let val = if let Some(Value::String(s)) = obj.properties.get("value") { s.clone() } else { "".to_string() };
            let ph = if let Some(Value::String(s)) = obj.properties.get("placeholder") { s.clone() } else { "Search...".to_string() };
            text_input(&ph, &val).on_input(BrowserMessage::UrlChanged).on_submit(BrowserMessage::UrlSubmit).padding(12).into()
        }
        "Space" => Space::with_height(Length::Fixed(get_number(&obj.properties, "height").unwrap_or(0.0))).into(),
        _ => text(format!("Unknown: {}", obj.tag)).into()
    }
}

fn get_number(props: &std::collections::HashMap<String, Value>, name: &str) -> Option<f32> {
    if let Some(Value::Number(n)) = props.get(name) { Some(*n as f32) } else { None }
}
fn get_string(props: &std::collections::HashMap<String, Value>, name: &str) -> Option<String> {
    if let Some(Value::String(s)) = props.get(name) { Some(s.clone()) } else { None }
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
