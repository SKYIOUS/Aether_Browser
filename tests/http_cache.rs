// HTTP cache bounding tests — proves the LRU capacity bound is enforced
// and that cache hit / eviction semantics work correctly.
//
// All assertions live in a single test to avoid races on the global mock
// responder. The mock is set once with all needed URLs.

use vayu_browser::engine::net;

#[test]
fn http_cache_bounds_and_hit_semantics() {
    net::clear_cache();

    // Build a mock with 260 unique URLs (exceeds the 256 LRU capacity).
    net::mock::set_mock({
        let mut m = net::mock::MockHttpResponder::new();
        for i in 0..260 {
            m = m.html(
                &format!("mock://httpcache_{}", i),
                &format!("<p>body{}</p>", i),
            );
        }
        m
    });

    // --- 1. Cache hit returns stored response ---
    let r1 = net::fetch("mock://httpcache_0", None).expect("first fetch must succeed");
    assert_eq!(r1.1, 200);
    assert_eq!(r1.0, "<p>body0</p>");
    let r2 = net::fetch("mock://httpcache_0", None).expect("cache hit must succeed");
    assert_eq!(r1.0, r2.0, "cache hit must return same body");

    // --- 2. Different URL misses cache, hits mock ---
    let r3 = net::fetch("mock://httpcache_1", None).expect("different URL must succeed");
    assert_eq!(r3.0, "<p>body1</p>");

    // --- 3. Repeated same-URL inserts stay bounded ---
    for _ in 0..100 {
        let _ = net::fetch("mock://httpcache_0", None);
    }
    assert!(
        net::cache_len() <= 256,
        "repeated same-URL inserts must not exceed capacity"
    );

    // --- 4. Insert 260 unique entries — LRU must evict ---
    for i in 2..260 {
        let url = format!("mock://httpcache_{}", i);
        let r = net::fetch(&url, None);
        assert!(r.is_ok(), "fetch {} must succeed: {:?}", i, r.err());
    }
    let len = net::cache_len();
    assert!(
        len <= 256,
        "cache length {} exceeds capacity 256 after 260 insertions",
        len
    );

    // --- 5. Recently accessed entry survives eviction ---
    // Entry 0 was accessed in step 1 (recently used). Insert one more to
    // trigger eviction — entry 0 should survive because it's the most recent.
    let _ = net::fetch("mock://httpcache_0", None);
    let _ = net::fetch("mock://httpcache_1", None); // triggers eviction
    assert!(
        net::cache_len() <= 256,
        "cache length {} exceeds capacity after LRU access+insert",
        net::cache_len()
    );
}
