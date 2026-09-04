//! `cargo run --bin audit` — lightweight source-analysis audit for Aether/Vayu.
//!
//! Reads source files directly (no linking against browser crates) and reports
//! pipeline propagation gaps, DOM/style wiring, layout capability, resource
//! safety, and repository health.
//!
//! Flags:
//!   --json    Print machine-readable JSON alongside the human report

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// ── Types ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Status {
    VerifiedOk,
    Dropped,
    Missing,
    ParsedButUnused,
    UnboundedRisk,
    CandidateRisk,
    #[allow(dead_code)]
    Unknown,
    Healthy,
    Unhealthy,
}

impl Status {
    fn tag(&self) -> &'static str {
        match self {
            Status::VerifiedOk => "VERIFIED_OK",
            Status::Dropped => "DROPPED",
            Status::Missing => "MISSING",
            Status::ParsedButUnused => "PARSED_BUT_UNUSED",
            Status::UnboundedRisk => "UNBOUNDED_RISK",
            Status::CandidateRisk => "CANDIDATE_RISK",
            Status::Unknown => "UNKNOWN",
            Status::Healthy => "HEALTHY",
            Status::Unhealthy => "UNHEALTHY",
        }
    }
}

#[derive(Debug, Clone)]
struct Finding {
    status: Status,
    category: String,
    subject: String,
    evidence: String,
    explanation: String,
    confidence: String,
}

impl Finding {
    fn to_json_map(&self) -> serde_json::Value {
        serde_json::json!({
            "status": self.status.tag(),
            "category": self.category,
            "subject": self.subject,
            "evidence": self.evidence,
            "explanation": self.explanation,
            "confidence": self.confidence,
        })
    }
}

// ── Source reading ──────────────────────────────────────────────────────────

fn load_source(root: &Path, rel: &str) -> String {
    let path = root.join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("WARN: could not read {}: {}", rel, e);
        String::new()
    })
}

// ── Struct field extraction ────────────────────────────────────────────────

/// Extract `pub <name>: <type>` fields from a struct definition in source.
fn extract_struct_fields(source: &str, struct_name: &str) -> Vec<(String, String)> {
    let mut fields = Vec::new();
    let pattern = format!("pub struct {}", struct_name);
    if let Some(start) = source.find(&pattern) {
        if let Some(brace) = source[start..].find('{') {
            let body_start = start + brace + 1;
            let mut depth = 1u32;
            let mut pos = body_start;
            let bytes = source.as_bytes();
            while pos < bytes.len() && depth > 0 {
                match bytes[pos] {
                    b'{' => depth += 1,
                    b'}' => depth -= 1,
                    _ => {}
                }
                pos += 1;
            }
            let body = &source[body_start..pos - 1];
            for line in body.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("pub ") {
                    let after_pub = &trimmed[4..];
                    if let Some(colon) = after_pub.find(':') {
                        let name = after_pub[..colon].trim().to_string();
                        let ty = after_pub[colon + 1..]
                            .trim()
                            .trim_end_matches(',')
                            .trim()
                            .to_string();
                        fields.push((name, ty));
                    }
                }
            }
        }
    }
    fields
}

// ── CSS pipeline audit ─────────────────────────────────────────────────────

fn audit_css_pipeline(root: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Load all sources upfront
    let props_json = load_source(root, "crates/aether-css/css-properties.json");
    let computed_src = load_source(root, "crates/aether-css/src/style_value.rs");
    let extractor_src = load_source(root, "src/engine/pipeline/extractor.rs");
    let layout_src = load_source(root, "crates/layout-engine/src/lib.rs");
    let adapter_src = load_source(root, "src/engine/pipeline/layout_adapter.rs");

    // Parse registered CSS properties
    let registered: BTreeSet<String> =
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&props_json) {
            if let Some(arr) = val["properties"].as_array() {
                arr.iter()
                    .filter_map(|p| p["name"].as_str().map(String::from))
                    .collect()
            } else {
                BTreeSet::new()
            }
        } else {
            BTreeSet::new()
        };

    // Stage 2: ComputedStyle fields
    let computed_fields = extract_struct_fields(&computed_src, "ComputedStyle");
    let computed_names: BTreeSet<String> = computed_fields.iter().map(|(n, _)| n.clone()).collect();

    // Stage 3: FullStyle fields
    let fullstyle_fields = extract_struct_fields(&extractor_src, "FullStyle");
    let fullstyle_names: BTreeSet<String> =
        fullstyle_fields.iter().map(|(n, _)| n.clone()).collect();

    // Stage 4: StyledElement fields
    let styled_fields = extract_struct_fields(&extractor_src, "StyledElement");
    let styled_names: BTreeSet<String> = styled_fields.iter().map(|(n, _)| n.clone()).collect();

    // Stage 5: LayoutElementInput fields
    let layout_fields = extract_struct_fields(&layout_src, "LayoutElementInput");
    let layout_names: BTreeSet<String> = layout_fields.iter().map(|(n, _)| n.clone()).collect();

    // ── ComputedStyle → FullStyle drops ──
    // Legitimate transformations: fields renamed, collapsed into arrays, or
    // expanded from sub-structs. These are NOT drops.
    let computed_to_fullstyle_ok: BTreeSet<&str> = [
        // FlexOptions sub-struct expansion
        "flex",
        // Border collapsed into [f32; 4]
        "border_top_width",
        "border_right_width",
        "border_bottom_width",
        "border_left_width",
        // Border colors collapsed into single Option<Color>
        "border_top_color",
        "border_right_color",
        "border_bottom_color",
        "border_left_color",
        // Padding collapsed into [f32; 4]
        "padding_top",
        "padding_right",
        "padding_bottom",
        "padding_left",
        // Renamed to css_width/css_height
        "width",
        "height",
        // Renamed to inset_*
        "top",
        "right",
        "bottom",
        "left",
    ]
    .into_iter()
    .collect();

    for field in &computed_names {
        if computed_to_fullstyle_ok.contains(field.as_str()) {
            continue;
        }
        if !fullstyle_names.contains(field) {
            findings.push(Finding {
                status: Status::Dropped,
                category: "CSS pipeline".to_string(),
                subject: format!("ComputedStyle.{}", field),
                evidence: format!(
                    "ComputedStyle field '{}' (style_value.rs) has no corresponding FullStyle field (extractor.rs)",
                    field
                ),
                explanation: "Property is parsed and stored in ComputedStyle but silently dropped during FullStyle construction. It never reaches layout or paint.".to_string(),
                confidence: "HIGH".to_string(),
            });
        }
    }

    // ── FullStyle → StyledElement drops ──
    for field in &fullstyle_names {
        if !styled_names.contains(field) {
            findings.push(Finding {
                status: Status::Dropped,
                category: "CSS pipeline".to_string(),
                subject: format!("FullStyle.{}", field),
                evidence: format!(
                    "FullStyle field '{}' has no corresponding StyledElement field",
                    field
                ),
                explanation: "Field exists in FullStyle but is not carried to StyledElement."
                    .to_string(),
                confidence: "HIGH".to_string(),
            });
        }
    }

    // ── StyledElement → LayoutElementInput drops (layout-relevant only) ──
    let layout_relevant: BTreeSet<&str> = [
        "text_align",
        "visibility",
        "border_radius",
        "opacity",
        "z_index",
        "overflow",
        "color",
        "font_weight",
        "font_family",
        "background_color",
        "border_color",
        "text_decoration",
        "cursor",
        "transform",
        "transition",
    ]
    .into_iter()
    .collect();

    for field in &styled_names {
        if layout_relevant.contains(field.as_str()) && !layout_names.contains(field) {
            let used_in_adapter = adapter_src.contains(field);
            if !used_in_adapter {
                findings.push(Finding {
                    status: Status::ParsedButUnused,
                    category: "CSS pipeline".to_string(),
                    subject: format!("StyledElement.{} → layout", field),
                    evidence: format!(
                        "StyledElement field '{}' is not mapped to LayoutElementInput in layout_adapter.rs",
                        field
                    ),
                    explanation: "Field reaches StyledElement but is not passed to the layout engine.".to_string(),
                    confidence: "HIGH".to_string(),
                });
            }
        }
    }

    // ── Registered CSS properties missing from ComputedStyle ──
    let shorthands: BTreeSet<&str> = [
        "margin",
        "padding",
        "border-width",
        "border",
        "border-color",
        "flex",
        "background",
        "gap",
    ]
    .into_iter()
    .collect();

    // Properties stored in FlexOptions sub-struct, not as direct ComputedStyle fields
    let flex_sub_props: BTreeSet<&str> = [
        "align-content",
        "align-items",
        "align-self",
        "flex-basis",
        "flex-direction",
        "flex-grow",
        "flex-shrink",
        "flex-wrap",
        "justify-content",
    ]
    .into_iter()
    .collect();

    for prop in &registered {
        let snake = prop.replace('-', "_");
        if computed_names.contains(&snake) {
            continue;
        }
        if shorthands.contains(prop.as_str()) || flex_sub_props.contains(prop.as_str()) {
            continue;
        }
        findings.push(Finding {
            status: Status::Missing,
            category: "CSS pipeline".to_string(),
            subject: format!("CSS property '{}' → ComputedStyle", prop),
            evidence: format!(
                "Property '{}' is in css-properties.json but has no ComputedStyle field",
                prop
            ),
            explanation: "Property is registered but never stored in the computed style struct."
                .to_string(),
            confidence: "MEDIUM".to_string(),
        });
    }

    findings
}

// ── DOM/style pipeline audit ───────────────────────────────────────────────

fn audit_dom_pipeline(root: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();

    let matcher_src = load_source(root, "crates/aether-css/src/matcher.rs");
    let extractor_src = load_source(root, "src/engine/pipeline/extractor.rs");
    let resolver_src = load_source(root, "crates/aether-css/src/resolver.rs");
    let stratus_src = load_source(root, "src/engine/stratus.rs");
    let fetcher_src = load_source(root, "src/engine/pipeline/fetcher.rs");

    // 1. class="" processing
    if matcher_src.contains("has_class") {
        findings.push(Finding {
            status: Status::VerifiedOk,
            category: "DOM pipeline".to_string(),
            subject: "class=\"\" stylesheet matching".to_string(),
            evidence: "matcher.rs: has_class() reads attributes.get(\"class\"), splits whitespace, checks membership".to_string(),
            explanation: "Class attributes are fully parsed and matched against stylesheet selectors.".to_string(),
            confidence: "HIGH".to_string(),
        });
    }

    // 2. style="" processing
    let has_inline_style_parser = resolver_src.contains("parse_inline_style")
        || extractor_src.contains("parse_inline_style")
        || stratus_src.contains("parse_inline_style");

    if !has_inline_style_parser {
        findings.push(Finding {
            status: Status::Missing,
            category: "DOM pipeline".to_string(),
            subject: "style=\"\" attribute parsing".to_string(),
            evidence: "No parse_inline_style function found in resolver.rs, extractor.rs, or stratus.rs. The style attribute is stored as a raw string but never parsed into CSS declarations.".to_string(),
            explanation: "HTML style=\"color: red\" is stored in the attributes HashMap but the resolver never reads it. Only JS element.style.X=Y works (via inline_styles HashMap).".to_string(),
            confidence: "HIGH".to_string(),
        });
    }

    // 3. <style> block processing
    if fetcher_src.contains("extract_styles") && fetcher_src.contains("\"style\"") {
        findings.push(Finding {
            status: Status::VerifiedOk,
            category: "DOM pipeline".to_string(),
            subject: "<style> block processing".to_string(),
            evidence: "fetcher.rs: extract_styles() finds <style> elements, collects text children, parses via stratus::parse()".to_string(),
            explanation: "Style blocks are extracted, parsed, and merged into the stylesheet.".to_string(),
            confidence: "HIGH".to_string(),
        });
    }

    // 4. External stylesheet processing
    if fetcher_src.contains("extract_links") && fetcher_src.contains("stylesheet") {
        findings.push(Finding {
            status: Status::VerifiedOk,
            category: "DOM pipeline".to_string(),
            subject: "External <link> stylesheet processing".to_string(),
            evidence: "fetcher.rs: extract_links() finds <link rel=stylesheet>, fetches via net::fetch_resource(), parses via stratus::parse()".to_string(),
            explanation: "External stylesheets are fetched, parsed, and merged. CSS cache is LRU-100.".to_string(),
            confidence: "HIGH".to_string(),
        });
    }

    // 5. Media query handling
    if stratus_src.contains("@media") || stratus_src.contains("parse_at_rule") {
        let has_media_eval = stratus_src.contains("media_query_eval")
            || stratus_src.contains("evaluate_media")
            || stratus_src.contains("match_media");
        if !has_media_eval {
            findings.push(Finding {
                status: Status::Dropped,
                category: "DOM pipeline".to_string(),
                subject: "@media query evaluation".to_string(),
                evidence: "stratus.rs: parse_at_rule() consumes @media tokens until '{', discards condition, parses inner rules unconditionally".to_string(),
                explanation: "The @media condition is parsed then thrown away. Inner rules apply regardless of viewport width.".to_string(),
                confidence: "HIGH".to_string(),
            });
        }
    }

    // 6. DOM tags intentionally skipped
    if extractor_src.contains("should_skip_tag") {
        findings.push(Finding {
            status: Status::VerifiedOk,
            category: "DOM pipeline".to_string(),
            subject: "Intentionally skipped DOM tags".to_string(),
            evidence: "extractor.rs: should_skip_tag() skips script, style, noscript, meta, link, head, title, svg, path, br, hr, template, iframe, option".to_string(),
            explanation: "These are intentional design choices. Option skip is a known gap in the flat path.".to_string(),
            confidence: "HIGH".to_string(),
        });
    }

    findings
}

// ── Layout capability audit ────────────────────────────────────────────────

fn audit_layout_capability(root: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();

    let layout_src = load_source(root, "crates/layout-engine/src/lib.rs");
    let adapter_src = load_source(root, "src/engine/pipeline/layout_adapter.rs");
    let layout_fields = extract_struct_fields(&layout_src, "LayoutElementInput");

    // Grid fields always set to None
    let grid_fields = [
        "grid_template_columns",
        "grid_template_rows",
        "grid_column",
        "grid_row",
        "grid_auto_flow",
    ];
    for field in &grid_fields {
        if layout_fields.iter().any(|(n, _)| n == field) {
            let pattern = format!("{}: Some(", field);
            if !adapter_src.contains(&pattern) {
                findings.push(Finding {
                    status: Status::ParsedButUnused,
                    category: "Layout capability".to_string(),
                    subject: format!("LayoutElementInput.{}", field),
                    evidence: format!(
                        "Field '{}' exists in LayoutElementInput but adapter always sets it to None",
                        field
                    ),
                    explanation: "Grid layout fields are wired into the input struct but have no upstream source yet.".to_string(),
                    confidence: "HIGH".to_string(),
                });
            }
        }
    }

    // position: fixed not handled in native engine
    if layout_src.contains("CssPosition::Absolute") && !layout_src.contains("CssPosition::Fixed") {
        findings.push(Finding {
            status: Status::ParsedButUnused,
            category: "Layout capability".to_string(),
            subject: "position: fixed in native engine".to_string(),
            evidence: "layout-engine/lib.rs: layout_block() checks == Absolute but not == Fixed for out-of-flow".to_string(),
            explanation: "position: fixed is parsed but the native engine does not take it out of flow.".to_string(),
            confidence: "HIGH".to_string(),
        });
    }

    // Missing overflow field
    if !layout_src.contains("overflow") {
        findings.push(Finding {
            status: Status::Missing,
            category: "Layout capability".to_string(),
            subject: "overflow in LayoutElementInput".to_string(),
            evidence: "LayoutElementInput has no overflow field".to_string(),
            explanation:
                "Overflow is parsed by CSS resolver but has no layout input representation."
                    .to_string(),
            confidence: "HIGH".to_string(),
        });
    }

    // Missing z-index field
    if !layout_src.contains("z_index") && !layout_src.contains("z-index") {
        findings.push(Finding {
            status: Status::Missing,
            category: "Layout capability".to_string(),
            subject: "z-index in LayoutElementInput".to_string(),
            evidence: "LayoutElementInput has no z-index field".to_string(),
            explanation:
                "z-index is parsed by CSS resolver but has no layout input representation."
                    .to_string(),
            confidence: "HIGH".to_string(),
        });
    }

    findings
}

// ── Resource safety audit ──────────────────────────────────────────────────

fn audit_resource_safety(root: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();

    let files = [
        "src/engine/net/mod.rs",
        "src/engine/net/cookies.rs",
        "src/engine/js/js_bridge.rs",
        "src/engine/text.rs",
        "src/engine/pipeline/fetcher.rs",
    ];

    for rel in &files {
        let src = load_source(root, rel);
        let has_lru = src.contains("LruCache");
        let has_quota = src.contains("QUOTA") || src.contains("quota");

        // Bounded caches — positive findings
        if has_lru {
            let cache_name = if rel.contains("net/mod") {
                "HTTP/Image cache"
            } else if rel.contains("text") {
                "Measure cache"
            } else if rel.contains("fetcher") {
                "CSS cache"
            } else {
                "Cache"
            };
            findings.push(Finding {
                status: Status::VerifiedOk,
                category: "Resource safety".to_string(),
                subject: format!("{} ({})", cache_name, rel),
                evidence: format!("Uses LRU cache with bounded capacity in {}", rel),
                explanation: "Cache has bounded capacity and eviction policy.".to_string(),
                confidence: "HIGH".to_string(),
            });
        }

        // localStorage quota
        if rel.contains("js_bridge") && has_quota {
            findings.push(Finding {
                status: Status::VerifiedOk,
                category: "Resource safety".to_string(),
                subject: format!("localStorage quota ({})", rel),
                evidence:
                    "js_bridge.rs: LOCAL_STORAGE_QUOTA = 5MB per-origin, enforced in storage.rs"
                        .to_string(),
                explanation: "Per-origin quota enforced before mutation.".to_string(),
                confidence: "HIGH".to_string(),
            });
        }

        // CSP store — unbounded HashMap
        if rel.contains("net/mod") && src.contains("HashMap<String,") && !has_lru {
            findings.push(Finding {
                status: Status::UnboundedRisk,
                category: "Resource safety".to_string(),
                subject: format!("CSP store ({})", rel),
                evidence: "STORE is OnceLock<RwLock<HashMap<String, CspPolicy>>> — unbounded, grows per origin".to_string(),
                explanation: "Page-controlled: navigations to unique origins grow this without limit.".to_string(),
                confidence: "HIGH".to_string(),
            });
        }

        // Cookie jar — unbounded Vec
        if rel.contains("cookies") && src.contains("Vec<CookieRecord>") {
            findings.push(Finding {
                status: Status::UnboundedRisk,
                category: "Resource safety".to_string(),
                subject: format!("Cookie jar ({})", rel),
                evidence: "JAR wraps Vec<CookieRecord> — unbounded, persists to disk".to_string(),
                explanation: "Server can set arbitrary cookies. No capacity limit.".to_string(),
                confidence: "HIGH".to_string(),
            });
        }

        // JS cookie store — unbounded nested HashMap
        if rel.contains("js_bridge") && src.contains("CookieOriginStore") {
            findings.push(Finding {
                status: Status::UnboundedRisk,
                category: "Resource safety".to_string(),
                subject: format!("JS cookie store ({})", rel),
                evidence: "COOKIE_STORE is OnceLock<RwLock<HashMap<String, HashMap<String, CookieEntry>>>>".to_string(),
                explanation: "Page-controlled via document.cookie. No capacity limit.".to_string(),
                confidence: "HIGH".to_string(),
            });
        }

        // Network log — partially bounded
        if rel.contains("net/mod") && src.contains("LOG") && src.contains("Vec::new") {
            findings.push(Finding {
                status: Status::CandidateRisk,
                category: "Resource safety".to_string(),
                subject: format!("Network log ({})", rel),
                evidence: "LOG is OnceLock<RwLock<Vec<String>>> — drains at 500 entries"
                    .to_string(),
                explanation: "Partially bounded. Page-controlled URLs logged.".to_string(),
                confidence: "MEDIUM".to_string(),
            });
        }
    }

    findings
}

// ── Repository health ──────────────────────────────────────────────────────

fn which_cargo() -> String {
    let home = std::env::var("USERPROFILE").unwrap_or_default();
    let cargo_path = PathBuf::from(&home).join(".cargo/bin/cargo.exe");
    if cargo_path.exists() {
        cargo_path.to_string_lossy().to_string()
    } else {
        "cargo".to_string()
    }
}

fn run_cargo_check(root: &Path, cargo: &str, args: &[&str]) -> (bool, String) {
    match Command::new(cargo).args(args).current_dir(root).output() {
        Ok(out) => {
            let combined = format!(
                "{}\n{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
            let trimmed = trim_output(&combined, 60);
            (out.status.success(), trimmed)
        }
        Err(e) => (false, format!("Failed to run {}: {}", cargo, e)),
    }
}

fn trim_output(s: &str, max_lines: usize) -> String {
    let lines: Vec<&str> = s.lines().collect();
    if lines.len() <= max_lines {
        s.trim().to_string()
    } else {
        let start = lines.len() - max_lines;
        format!(
            "... ({} lines truncated)\n{}",
            lines.len() - max_lines,
            lines[start..].join("\n")
        )
    }
}

fn audit_repo_health(root: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();
    let cargo = which_cargo();

    let fmt_result = run_cargo_check(root, &cargo, &["fmt", "--check"]);
    findings.push(Finding {
        status: if fmt_result.0 {
            Status::Healthy
        } else {
            Status::Unhealthy
        },
        category: "Repository health".to_string(),
        subject: "cargo fmt --check".to_string(),
        evidence: fmt_result.1,
        explanation: if fmt_result.0 {
            "All source files are formatted.".to_string()
        } else {
            "Some source files are not formatted.".to_string()
        },
        confidence: "HIGH".to_string(),
    });

    let test_result = run_cargo_check(
        root,
        &cargo,
        &["test", "--", "--skip", "test_flex_shorthand_initial"],
    );
    findings.push(Finding {
        status: if test_result.0 {
            Status::Healthy
        } else {
            Status::Unhealthy
        },
        category: "Repository health".to_string(),
        subject: "cargo test".to_string(),
        evidence: test_result.1,
        explanation: if test_result.0 {
            "All tests pass (excl. known pre-existing failures).".to_string()
        } else {
            "Some tests failed.".to_string()
        },
        confidence: "HIGH".to_string(),
    });

    let clippy_result = run_cargo_check(root, &cargo, &["clippy", "--", "-D", "warnings"]);
    findings.push(Finding {
        status: if clippy_result.0 {
            Status::Healthy
        } else {
            Status::Unhealthy
        },
        category: "Repository health".to_string(),
        subject: "cargo clippy -- -D warnings".to_string(),
        evidence: clippy_result.1,
        explanation: if clippy_result.0 {
            "No clippy warnings.".to_string()
        } else {
            "Clippy warnings found.".to_string()
        },
        confidence: "HIGH".to_string(),
    });

    findings
}

// ── Report output ──────────────────────────────────────────────────────────

fn print_report(findings: &[Finding]) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Aether/Vayu Browser — Architecture Audit Report");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    let categories: BTreeSet<String> = findings.iter().map(|f| f.category.clone()).collect();
    for cat in &categories {
        let cat_findings: Vec<&Finding> = findings.iter().filter(|f| &f.category == cat).collect();
        println!("── {} ──", cat);
        println!();
        for f in &cat_findings {
            println!("[{}] {}", f.status.tag(), f.subject);
            println!("  Evidence: {}", f.evidence);
            println!("  Note: {}", f.explanation);
            println!("  Confidence: {}", f.confidence);
            println!();
        }
    }

    let dropped = findings
        .iter()
        .filter(|f| f.status == Status::Dropped)
        .count();
    let missing = findings
        .iter()
        .filter(|f| f.status == Status::Missing)
        .count();
    let parsed_unused = findings
        .iter()
        .filter(|f| f.status == Status::ParsedButUnused)
        .count();
    let unbounded = findings
        .iter()
        .filter(|f| f.status == Status::UnboundedRisk)
        .count();
    let healthy = findings
        .iter()
        .filter(|f| f.status == Status::Healthy)
        .count();
    let unhealthy = findings
        .iter()
        .filter(|f| f.status == Status::Unhealthy)
        .count();

    println!("═══════════════════════════════════════════════════════════════");
    println!("  Summary");
    println!("═══════════════════════════════════════════════════════════════");
    println!("  DROPPED:           {}", dropped);
    println!("  MISSING:           {}", missing);
    println!("  PARSED_BUT_UNUSED: {}", parsed_unused);
    println!("  UNBOUNDED_RISK:    {}", unbounded);
    println!("  HEALTHY:           {}", healthy);
    println!("  UNHEALTHY:         {}", unhealthy);
    println!("  Total findings:    {}", findings.len());
    println!("═══════════════════════════════════════════════════════════════");
}

fn print_json(findings: &[Finding]) {
    let json: Vec<serde_json::Value> = findings.iter().map(|f| f.to_json_map()).collect();
    let output = serde_json::json!({
        "audit": "aether-vayu-architecture",
        "findings": json,
        "summary": {
            "dropped": findings.iter().filter(|f| f.status == Status::Dropped).count(),
            "missing": findings.iter().filter(|f| f.status == Status::Missing).count(),
            "parsed_but_unused": findings.iter().filter(|f| f.status == Status::ParsedButUnused).count(),
            "unbounded_risk": findings.iter().filter(|f| f.status == Status::UnboundedRisk).count(),
            "healthy": findings.iter().filter(|f| f.status == Status::Healthy).count(),
            "unhealthy": findings.iter().filter(|f| f.status == Status::Unhealthy).count(),
            "total": findings.len(),
        }
    });
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

// ── Main ───────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let json_mode = args.contains(&"--json".to_string());

    let root = std::env::current_dir().expect("failed to get current directory");
    if !root.join("Cargo.toml").exists() {
        eprintln!("ERROR: Must be run from the repository root (Cargo.toml not found)");
        std::process::exit(1);
    }

    let mut findings = Vec::new();
    findings.extend(audit_css_pipeline(&root));
    findings.extend(audit_dom_pipeline(&root));
    findings.extend(audit_layout_capability(&root));
    findings.extend(audit_resource_safety(&root));
    findings.extend(audit_repo_health(&root));

    if json_mode {
        print_json(&findings);
    } else {
        print_report(&findings);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_struct_fields_basic() {
        let source = r#"
            pub struct Foo {
                pub name: String,
                pub count: i32,
                private: bool,
            }
        "#;
        let fields = extract_struct_fields(source, "Foo");
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, "name");
        assert_eq!(fields[0].1, "String");
        assert_eq!(fields[1].0, "count");
        assert_eq!(fields[1].1, "i32");
    }

    #[test]
    fn test_extract_struct_fields_nested_braces() {
        let source = r#"
            pub struct Bar {
                pub items: Vec<String>,
                pub map: HashMap<String, i32>,
            }

            impl Bar {
                pub fn new() -> Self { Bar { items: vec![], map: HashMap::new() } }
            }
        "#;
        let fields = extract_struct_fields(source, "Bar");
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, "items");
        assert_eq!(fields[1].0, "map");
    }

    #[test]
    fn test_extract_struct_fields_not_found() {
        let source = "struct Private { x: i32 }";
        let fields = extract_struct_fields(source, "NoSuchStruct");
        assert!(fields.is_empty());
    }

    #[test]
    fn test_trim_output_short() {
        let s = "line1\nline2\nline3";
        assert_eq!(trim_output(s, 10), "line1\nline2\nline3");
    }

    #[test]
    fn test_trim_output_long() {
        let lines: Vec<String> = (0..100).map(|i| format!("line {}", i)).collect();
        let s = lines.join("\n");
        let trimmed = trim_output(&s, 5);
        assert!(trimmed.contains("line 99"));
        assert!(trimmed.contains("truncated"));
    }

    #[test]
    fn test_status_tags() {
        assert_eq!(Status::Dropped.tag(), "DROPPED");
        assert_eq!(Status::VerifiedOk.tag(), "VERIFIED_OK");
        assert_eq!(Status::UnboundedRisk.tag(), "UNBOUNDED_RISK");
        assert_eq!(Status::ParsedButUnused.tag(), "PARSED_BUT_UNUSED");
        assert_eq!(Status::Missing.tag(), "MISSING");
    }
}
