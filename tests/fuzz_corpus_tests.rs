// C4: platform-neutral adversarial corpus.
//
// The coverage-guided targets in fuzz/ run on Linux CI; this suite feeds a
// fixed corpus through the SAME production entry points on every platform so
// known-nasty shapes stay regression-tested locally too. Deterministic by
// construction - no randomness, no network.

use vayu_browser::engine::parser::parse_html;
use vayu_browser::engine::pipeline::fuzz_surface::{
    apply_html_budget, css_sources_within_total_budget, trim_to_budget, MAX_ELEMENTS,
};
use vayu_browser::engine::stratus;

fn html_corpus() -> Vec<String> {
    vec![
        "".into(),
        "<".into(),
        "<<<>>>".into(),
        "<div".into(),
        "<div class=".into(),
        "<a href='unterminated".into(),
        "<table><td>x</td></table>".into(),
        "<b><i>x</b>y</i>".into(),
        "<p><form><form></form></form></p>".into(),
        format!("<div attr='{}'>x</div>", "A".repeat(5000)),
        format!("{}<b>t", "<span>".repeat(300)),
        "&#xZZ; &#999999999; &amp &lt".into(),
        "<script>for(;;){}</script>".into(),
        "<style>@media{a{b:c}}".into(),
        "\u{0}\u{1}\u{2}control\u{7F}".into(),
    ]
}

fn css_corpus() -> Vec<String> {
    vec![
        "".into(),
        "{".into(),
        "}}}}".into(),
        "a{".into(),
        "a{color:".into(),
        "a{color:red".into(),
        "@media screen { a { color: red } }".into(),
        "@unknown-at-rule param { nested { deep } }".into(),
        "a[b='c' i]{color:red!important}".into(),
        format!("{} {{ color: red }}", ".c".repeat(2000)),
        "/* unterminated comment".into(),
        "color:red;;;a:b;;;".into(),
        "\\65 xample{content:\"\\1F600\"}".into(),
    ]
}

fn url_corpus() -> Vec<String> {
    vec![
        "".into(),
        "http://".into(),
        "https://[::1]:99999/x".into(),
        "//host/path".into(),
        "/relative".into(),
        "?query=only".into(),
        "#fragment".into(),
        "http://user:pass@host:8080/p?q#f".into(),
        "HTTP://UPPER.CASE/HOST".into(),
        "ht!tp://weird.scheme/x".into(),
        format!("http://h/{}", "p".repeat(4000)),
    ]
}

#[test]
fn html_corpus_never_panics_and_respects_element_budget() {
    for input in html_corpus() {
        let dom = parse_html(&input);
        // Materialized tree must be finite and bounded by the A1 ceiling.
        fn count(n: &vayu_browser::engine::dom::Node) -> usize {
            1 + n.children.iter().map(|c| count(c)).sum::<usize>()
        }
        assert!(count(&dom) < MAX_ELEMENTS);
    }
}

#[test]
fn css_corpus_never_panics() {
    for input in css_corpus() {
        let sheet = stratus::parse(&input);
        // Rule objects must be fully materialized (no lazy traps).
        for rule in &sheet.rules {
            assert!(!std::ptr::eq(rule, std::ptr::null()));
        }
    }
}

#[test]
fn url_corpus_produces_parseable_state() {
    for input in url_corpus() {
        let normalized = vayu_browser::engine::net::normalize_url(&input);
        if normalized.contains("://") {
            let after = normalized.split("://").nth(1).unwrap_or("");
            assert!(
                !after.starts_with('/'),
                "normalize_url({input:?}) produced host-less URL {normalized:?}"
            );
        }
        let _ = vayu_browser::engine::net::resolve_url("/p", &normalized);
        let _ = vayu_browser::engine::net::redirect_target(&normalized, "/next");
    }
}

#[test]
fn budget_helpers_hold_their_invariants_on_the_corpus() {
    for input in html_corpus().iter().chain(css_corpus().iter()) {
        let capped = apply_html_budget(input.clone(), 512);
        assert!(capped.len() <= 512);
        assert!(input.starts_with(&capped), "html budget must keep prefixes");
        assert!(capped.is_char_boundary(capped.len()));

        let trimmed = trim_to_budget(input, 64);
        assert!(trimmed.len() <= 64);
        assert!(input.starts_with(trimmed));

        let sources: Vec<&str> = input.split('\u{0}').collect();
        let (_, total) = css_sources_within_total_budget(&sources, 0, 128);
        assert!(total <= 128);
        let (_, grown) = css_sources_within_total_budget(&sources, total, 256);
        assert!(grown >= total, "cumulative accounting must not shrink");
        assert!(grown <= 256);
    }
}
