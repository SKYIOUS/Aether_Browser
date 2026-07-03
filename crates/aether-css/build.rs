use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=css-properties.json");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR must be set by Cargo");
    let dest_path = Path::new(&out_dir).join("css_properties.rs");

    let json = fs::read_to_string("css-properties.json")
        .expect("Failed to read css-properties.json");

    let props: serde_json::Value = serde_json::from_str(&json)
        .expect("css-properties.json is not valid JSON");

    let properties = props["properties"]
        .as_array()
        .expect("css-properties.json: properties should be an array");

    let mut from_str_arms = String::new();
    let mut display_arms = String::new();
    let mut variant_names = Vec::new();

    for prop in properties {
        let name = prop["name"].as_str().expect("property name missing");
        let enum_name = prop["enum"].as_str().expect("property enum name missing");
        variant_names.push(enum_name.to_string());
        from_str_arms.push_str(&format!(
            "            {:?} => Ok(Self::{}),\n",
            name, enum_name
        ));
        display_arms.push_str(&format!(
            "            Self::{} => write!(f, {:?}),\n",
            enum_name, name
        ));
    }

    let output = format!(
        r##"#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CssPropertyName {{
    {},
}}

impl std::str::FromStr for CssPropertyName {{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        match s {{
{}
            _ => Err(())
        }}
    }}
}}

impl std::fmt::Display for CssPropertyName {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        match self {{
{}
        }}
    }}
}}
"##,
        variant_names.join(",\n    "),
        from_str_arms,
        display_arms,
    );

    fs::write(&dest_path, &output).expect("Failed to write css_properties.rs");
}
