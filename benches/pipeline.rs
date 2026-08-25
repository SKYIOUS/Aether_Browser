//! PLAN D0 measurement harness.
//!
//! Fixtures are built OUTSIDE the timed closures; layout benches clone their
//! element vecs via iter_batched so construction cost never lands in the
//! measurement. The full-pipeline benches drive the real async
//! fetch_page_content against MockHttpResponder (delay knob available for
//! D1's serial-vs-parallel comparison).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vayu_browser::engine::net::mock;
use vayu_browser::engine::pipeline::{
    apply_taffy_layout, fetch_page_content,
};
use vayu_browser::engine::pipeline::extractor::extract_elements;
use vayu_browser::engine::parser::parse_html;
use vayu_browser::engine::stratus;

fn small_doc() -> String {
    "<html><body><h1>Title</h1><p>hello world</p><a href=\"https://x\">link</a></body></html>".into()
}

fn big_doc() -> String {
    // ~5000 block elements with text: exercises A1 ceilings' neighborhood and
    // the A4 wrap path.
    let mut s = String::from("<html><body>");
    for i in 0..2500 {
        s.push_str(&format!(
            "<p>paragraph {i} wraps across the line because this sentence is long enough to split</p>"
        ));
    }
    s.push_str("</body></html>");
    s
}

fn css_2k() -> String {
    let mut s = String::new();
    for i in 0..2000 {
        s.push_str(&format!(".c{i}{{color:red;margin:{i}px}}"));
    }
    s
}

fn elements_from(html: &str) -> Vec<vayu_browser::engine::pipeline::StyledElement> {
    let dom = parse_html(html);
    let sheet = stratus::parse("");
    let mut els = Vec::new();
    extract_elements(&dom, &mut els, 0, &sheet, None, None, vec![], 800.0, 600.0);
    els
}

fn bench_parsers(c: &mut Criterion) {
    let small = small_doc();
    let big = big_doc();
    let css_small = "a{color:red}";
    let css_big = css_2k();

    let mut g = c.benchmark_group("d0_parse");
    g.throughput(criterion::Throughput::Bytes(small.len() as u64));
    g.bench_function("parse_html_small", |b| b.iter(|| black_box(parse_html(black_box(&small)))));
    g.sample_size(20);
    g.throughput(criterion::Throughput::Bytes(big.len() as u64));
    g.bench_function("parse_html_big_5k", |b| b.iter(|| black_box(parse_html(black_box(&big)))));
    g.bench_function("parse_css_tiny", |b| b.iter(|| black_box(stratus::parse(css_small))));
    g.bench_function("parse_css_2k_rules", |b| b.iter(|| black_box(stratus::parse(&css_big))));
    g.finish();
}

fn bench_extract(c: &mut Criterion) {
    let big = big_doc();
    let dom = parse_html(&big);
    let sheet = stratus::parse("");

    let mut g = c.benchmark_group("d0_extract");
    g.bench_function("extract_elements_big_doc", |b| {
        b.iter(|| {
            let mut els = Vec::new();
            extract_elements(
                black_box(&dom), &mut els, 0, black_box(&sheet), None, None, vec![], 800.0, 600.0,
            );
            black_box(els.len())
        })
    });
    g.finish();
}

fn bench_layout(c: &mut Criterion) {
    let small = elements_from(&small_doc());
    let big = elements_from(&big_doc());

    let mut g = c.benchmark_group("d0_layout");
    g.bench_function("taffy_200_wrapped", |b| {
        b.iter_batched(
            || small.clone(),
            |mut els| apply_taffy_layout(black_box(&mut els), 800.0, 600.0),
            criterion::BatchSize::SmallInput,
        )
    });
    g.sample_size(20);
    g.bench_function("taffy_5k_wrapped", |b| {
        b.iter_batched(
            || big.clone(),
            |mut els| apply_taffy_layout(black_box(&mut els), 800.0, 600.0),
            criterion::BatchSize::LargeInput,
        )
    });
    g.finish();
}

/// Full pipeline over mock pages: document + 3 stylesheets + 6 images, JS off.
fn mock_page(name: &str) -> mock::MockHttpResponder {
    let doc = format!(
        "<html><head><link rel=stylesheet href=\"mock://{name}/a.css\">\
         <link rel=stylesheet href=\"mock://{name}/b.css\">\
         <link rel=stylesheet href=\"mock://{name}/c.css\"></head><body>\
         <p>full pipeline body</p>\
         <img src=\"mock://{name}/i1\"><img src=\"mock://{name}/i2\">\
         <img src=\"mock://{name}/i3\"><img src=\"mock://{name}/i4\">\
         <img src=\"mock://{name}/i5\"><img src=\"mock://{name}/i6\">\
         </body></html>"
    );
    let css = "p{color:red}div{margin:4px}";
    // Deterministic non-image bytes: decode intentionally fails and the
    // pipeline skips them - this bench targets subresource FETCH cost (D1),
    // not image decoding.
    let img: Vec<u8> =
        include_bytes!("../src/ui/screens/browser/tab_bar.rs").to_vec();
    mock::MockHttpResponder::new()
        .html(&format!("mock://{name}"), &doc)
        .css(&format!("mock://{name}/a.css"), css)
        .css(&format!("mock://{name}/b.css"), css)
        .css(&format!("mock://{name}/c.css"), css)
        .binary(format!("mock://{name}/i1").as_str(), img.clone())
        .binary(format!("mock://{name}/i2").as_str(), img.clone())
        .binary(format!("mock://{name}/i3").as_str(), img.clone())
        .binary(format!("mock://{name}/i4").as_str(), img.clone())
        .binary(format!("mock://{name}/i5").as_str(), img.clone())
        .binary(format!("mock://{name}/i6").as_str(), img)
}

fn bench_full_pipeline(c: &mut Criterion) {
    vayu_browser::engine::pipeline::set_js_enabled(false);

    mock::set_mock(mock_page("bench_fast"));
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("bench runtime");

    let mut g = c.benchmark_group("d0_fetch");
    g.bench_function("fetch_full_mock", |b| {
        b.to_async(&rt)
            .iter(|| async {
                let (url, els, _) =
                    fetch_page_content("mock://bench_fast".into(), 800.0, 600.0, Vec::new()).await;
                black_box((url, els.len()));
            })
    });

    // Serial lower bound for D1: nine subresources x 5ms each must cost ~45ms
    // today; after D1 lands, wall-clock should approach a single delay.
    mock::set_mock(mock_page("bench_slow").delay_ms(5));
    g.sample_size(10);
    g.measurement_time(std::time::Duration::from_secs(10));
    g.bench_function("fetch_full_mock_delayed_9x5ms", |b| {
        b.to_async(&rt)
            .iter(|| async {
                let (url, els, _) =
                    fetch_page_content("mock://bench_slow".into(), 800.0, 600.0, Vec::new()).await;
                black_box((url, els.len()));
            })
    });
    g.finish();
    mock::clear_mock();
}

criterion_group!(
    benches,
    bench_parsers,
    bench_extract,
    bench_layout,
    bench_full_pipeline
);
criterion_main!(benches);
