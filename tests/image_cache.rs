// Image cache bounding tests — proves the LRU capacity bound is enforced
// and that cache hit / eviction semantics work correctly.
//
// All assertions live in a single test to avoid races on the global cache.
// The cache is cleared once at the start with unique URL prefixes.

use vayu_browser::engine::net;

const CAPACITY: usize = 256;

#[test]
fn image_cache_bounds_and_lru_semantics() {
    net::clear_image_cache();

    // --- 1. Store and retrieve ---
    net::put_image_cache("https://example.com/bnd_a.png", vec![1, 2, 3]);
    let hit = net::fetch_bytes("https://example.com/bnd_a.png", None);
    assert!(hit.is_ok(), "cache hit must succeed after put");
    assert_eq!(hit.unwrap(), vec![1, 2, 3], "cached bytes must match");

    // --- 2. Capacity enforced ---
    for i in 0..CAPACITY + 10 {
        net::put_image_cache(
            &format!("https://example.com/bnd_cap_{}.png", i),
            vec![i as u8],
        );
    }
    let len = net::image_cache_len();
    assert!(
        len <= CAPACITY,
        "cache length {} must not exceed capacity {}",
        len,
        CAPACITY
    );

    // --- 3. Missing URL returns error, cache unaffected ---
    let before_len = net::image_cache_len();
    let miss = net::fetch_bytes("https://example.com/bnd_never.png", None);
    assert!(miss.is_err(), "missing entry must return error");
    assert_eq!(
        net::image_cache_len(),
        before_len,
        "cache must not grow from a failed fetch"
    );

    // --- 4. LRU eviction order ---
    net::clear_image_cache();
    for i in 0..CAPACITY {
        net::put_image_cache(
            &format!("https://example.com/bnd_lru_{}.png", i),
            vec![i as u8],
        );
    }
    assert_eq!(net::image_cache_len(), CAPACITY);

    // Re-insert entry 0 to refresh its recency (promote to MRU)
    net::put_image_cache("https://example.com/bnd_lru_0.png", vec![0]);
    // MRU=0, LRU=1

    // Insert 2 new entries, evicts LRU=1 then LRU=2
    net::put_image_cache("https://example.com/bnd_lru_new_a.png", vec![254]);
    net::put_image_cache("https://example.com/bnd_lru_new_b.png", vec![255]);

    // Entry 0 survives (was refreshed to MRU)
    assert!(
        net::fetch_bytes("https://example.com/bnd_lru_0.png", None).is_ok(),
        "entry 0 must survive (was recently accessed)"
    );

    // Entry 1 was LRU, evicted — cache miss → network fails
    assert!(
        net::fetch_bytes("https://example.com/bnd_lru_1.png", None).is_err(),
        "entry 1 must be evicted (was LRU)"
    );

    // Entry 2 also evicted (second eviction)
    assert!(
        net::fetch_bytes("https://example.com/bnd_lru_2.png", None).is_err(),
        "entry 2 must be evicted (second eviction)"
    );

    // Entry 3 survives (was not LRU when eviction happened)
    assert!(
        net::fetch_bytes("https://example.com/bnd_lru_3.png", None).is_ok(),
        "entry 3 must survive (was not LRU)"
    );

    // --- 5. Repeated put refreshes recency ---
    net::clear_image_cache();
    for i in 0..CAPACITY {
        net::put_image_cache(
            &format!("https://example.com/bnd_ra_{}.png", i),
            vec![i as u8],
        );
    }

    // Re-insert entry 0 to keep it at MRU
    net::put_image_cache("https://example.com/bnd_ra_0.png", vec![0]);

    // Insert one new entry — evicts the true LRU (entry 1), not entry 0
    net::put_image_cache("https://example.com/bnd_ra_new.png", vec![254]);

    // Entry 0 must survive (frequently accessed)
    assert!(
        net::fetch_bytes("https://example.com/bnd_ra_0.png", None).is_ok(),
        "frequently accessed entry 0 must survive eviction"
    );

    // Entry 1 must be evicted
    assert!(
        net::fetch_bytes("https://example.com/bnd_ra_1.png", None).is_err(),
        "entry 1 (LRU) must be evicted"
    );
}
