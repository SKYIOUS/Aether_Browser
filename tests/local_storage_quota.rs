use vayu_browser::engine::js::js_bridge::{JsBridge, LOCAL_STORAGE_QUOTA};

fn bridge_for(origin: &str) -> JsBridge {
    JsBridge::load_dom(&vayu_browser::engine::dom::Node::new_document(), origin)
}

// ---------------------------------------------------------------------------
// 1. Normal set/get
// ---------------------------------------------------------------------------

#[test]
fn set_get_basic() {
    let mut b = bridge_for("https://a.example.com");
    b.local_storage_set_item("k".into(), "v".into());
    assert_eq!(b.local_storage_get_item("k"), Some("v".into()));
    assert_eq!(b.local_storage_get_item("missing"), None);
}

// ---------------------------------------------------------------------------
// 2. Replacement of an existing value
// ---------------------------------------------------------------------------

#[test]
fn replace_existing_value() {
    let mut b = bridge_for("https://b.example.com");
    b.local_storage_set_item("k".into(), "old".into());
    b.local_storage_set_item("k".into(), "new".into());
    assert_eq!(b.local_storage_get_item("k"), Some("new".into()));
    assert_eq!(b.local_storage_length(), 1);
}

// ---------------------------------------------------------------------------
// 3. Quota rejection
// ---------------------------------------------------------------------------

#[test]
fn quota_rejection() {
    let mut b = bridge_for("https://c.example.com");
    let big = "x".repeat(LOCAL_STORAGE_QUOTA + 1);
    b.local_storage_set_item("k".into(), big);
    assert_eq!(b.local_storage_get_item("k"), None);
    assert_eq!(b.local_storage_length(), 0);
}

// ---------------------------------------------------------------------------
// 4. Quota accounting after replacement
// ---------------------------------------------------------------------------

#[test]
fn quota_accounting_after_replace() {
    let mut b = bridge_for("https://d.example.com");
    b.local_storage_set_item("k".into(), "short".into());
    let used_after_small = b.local_storage_used_bytes();
    assert!(used_after_small > 0, "should track bytes");

    b.local_storage_set_item("k".into(), "a longer value".into());
    let used_after_replace = b.local_storage_used_bytes();
    assert!(
        used_after_replace > used_after_small,
        "replacing with larger value should increase usage"
    );

    b.local_storage_set_item("k".into(), "s".into());
    let used_after_shrink = b.local_storage_used_bytes();
    assert!(
        used_after_shrink < used_after_replace,
        "replacing with smaller value should decrease usage"
    );
}

// ---------------------------------------------------------------------------
// 5. Removal freeing quota
// ---------------------------------------------------------------------------

#[test]
fn removal_frees_quota() {
    let mut b = bridge_for("https://e.example.com");
    b.local_storage_set_item("k1".into(), "a".repeat(1000));
    b.local_storage_set_item("k2".into(), "b".repeat(1000));
    let used_before = b.local_storage_used_bytes();
    assert!(used_before >= 2000);

    b.local_storage_remove_item("k1");
    let used_after = b.local_storage_used_bytes();
    assert!(used_after < used_before, "removal should free bytes");
    assert_eq!(b.local_storage_get_item("k1"), None);
    assert_eq!(b.local_storage_get_item("k2"), Some("b".repeat(1000)));
}

// ---------------------------------------------------------------------------
// 6. Clear resets quota
// ---------------------------------------------------------------------------

#[test]
fn clear_resets_quota() {
    let mut b = bridge_for("https://f.example.com");
    b.local_storage_set_item("a".into(), "x".repeat(5000));
    b.local_storage_set_item("b".into(), "y".repeat(5000));
    assert!(b.local_storage_used_bytes() > 10000);

    b.local_storage_clear();
    assert_eq!(b.local_storage_length(), 0);
    assert_eq!(b.local_storage_used_bytes(), 0);
}

// ---------------------------------------------------------------------------
// 7. Independent quotas between origins
// ---------------------------------------------------------------------------

#[test]
fn independent_origins() {
    let mut b1 = bridge_for("https://origin-a.example.com");
    let mut b2 = bridge_for("https://origin-b.example.com");

    b1.local_storage_set_item("k".into(), "from-a".into());
    b2.local_storage_set_item("k".into(), "from-b".into());

    assert_eq!(b1.local_storage_get_item("k"), Some("from-a".into()));
    assert_eq!(b2.local_storage_get_item("k"), Some("from-b".into()));

    b1.local_storage_clear();
    assert_eq!(b1.local_storage_get_item("k"), None);
    assert_eq!(b2.local_storage_get_item("k"), Some("from-b".into()));
}

// ---------------------------------------------------------------------------
// 8. Boundary / exact-quota behavior
// ---------------------------------------------------------------------------

#[test]
fn exact_quota_boundary() {
    let mut b = bridge_for("https://g.example.com");
    // Fill to exactly the quota: key "only" (4 bytes) + value
    let val = "z".repeat(LOCAL_STORAGE_QUOTA - 4);
    b.local_storage_set_item("only".into(), val.clone());
    assert_eq!(b.local_storage_length(), 1);
    assert_eq!(b.local_storage_get_item("only"), Some(val));

    // One more byte should fail
    let overfull = "z".repeat(LOCAL_STORAGE_QUOTA);
    b.local_storage_set_item("second".into(), overfull);
    assert_eq!(b.local_storage_length(), 1);
    assert_eq!(b.local_storage_get_item("second"), None);
}

// ---------------------------------------------------------------------------
// 9. Repeated rejected writes don't grow memory
// ---------------------------------------------------------------------------

#[test]
fn rejected_writes_no_growth() {
    let mut b = bridge_for("https://h.example.com");
    let big = "x".repeat(LOCAL_STORAGE_QUOTA + 100);
    for _ in 0..10 {
        b.local_storage_set_item("k".into(), big.clone());
    }
    assert_eq!(b.local_storage_length(), 0);
    assert_eq!(b.local_storage_used_bytes(), 0);
}

// ---------------------------------------------------------------------------
// 10. Key/value byte accounting: key length counts too
// ---------------------------------------------------------------------------

#[test]
fn key_length_counts_toward_quota() {
    let mut b = bridge_for("https://i.example.com");
    let long_key = "k".repeat(LOCAL_STORAGE_QUOTA);
    b.local_storage_set_item(long_key.clone(), "v".into());
    // key alone exceeds quota
    assert_eq!(b.local_storage_get_item(&long_key), None);
    assert_eq!(b.local_storage_length(), 0);
}

// ---------------------------------------------------------------------------
// 11. Removal of non-existent key is a no-op
// ---------------------------------------------------------------------------

#[test]
fn remove_nonexistent_key() {
    let mut b = bridge_for("https://j.example.com");
    b.local_storage_remove_item("nope");
    assert_eq!(b.local_storage_length(), 0);
}

// ---------------------------------------------------------------------------
// 12. replace with smaller value frees exact difference
// ---------------------------------------------------------------------------

#[test]
fn exact_byte_accounting() {
    let mut b = bridge_for("https://k.example.com");
    b.local_storage_set_item("k".into(), "abc".into());
    let used1 = b.local_storage_used_bytes(); // "k".len() + "abc".len() = 4

    b.local_storage_set_item("k".into(), "ab".into());
    let used2 = b.local_storage_used_bytes(); // "k".len() + "ab".len() = 3

    assert_eq!(used1 - used2, 1, "should free exactly 1 byte");
}
