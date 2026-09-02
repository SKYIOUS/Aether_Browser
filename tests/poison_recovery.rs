use std::sync::{Arc, Mutex, RwLock};
use std::thread;

/// Test: Mutex lock().unwrap_or_else(|e| e.into_inner()) survives poisoned lock.
/// Mirrors the recovery pattern at text.rs:121 (font_system lock).
#[test]
fn mutex_poison_recovery_preserves_data() {
    let m = Arc::new(Mutex::new(42u32));

    let m2 = Arc::clone(&m);
    let _ = thread::spawn(move || {
        let _guard = m2.lock().unwrap();
        panic!("simulate thread failure while holding lock");
    })
    .join();

    let val = *m.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(val, 42, "recovered value must match pre-poison state");
}

/// Test: RwLock write().unwrap_or_else(|e| e.into_inner()) survives poisoned lock.
/// Mirrors the recovery pattern at layout.rs:30,39,49 (width cache locks).
#[test]
fn rwlock_write_poison_recovery_preserves_data() {
    let cache = Arc::new(RwLock::new(std::collections::HashMap::<u32, f32>::new()));
    cache.write().unwrap().insert(1, 3.14);

    let c2 = Arc::clone(&cache);
    let _ = thread::spawn(move || {
        let _guard = c2.write().unwrap();
        panic!("simulate thread failure while holding write lock");
    })
    .join();

    let guard = cache.write().unwrap_or_else(|e| e.into_inner());
    let val = *guard.get(&1).expect("cached value must survive poison");
    assert!(
        (val - 3.14).abs() < f32::EPSILON,
        "recovered cache value must match"
    );
}

/// Test: thread join().unwrap_or_else() recovers from panicked thread.
/// Mirrors the recovery pattern at fetcher.rs:977,1127 (fetch thread joins).
#[test]
fn thread_join_recovery_returns_default() {
    let results: Vec<(String, Result<Vec<u8>, ()>)> = thread::scope(|s| {
        let handles: Vec<_> = vec!["ok", "panic"]
            .into_iter()
            .map(|label| {
                let l = label.to_string();
                s.spawn(move || -> (String, Result<Vec<u8>, ()>) {
                    if l == "panic" {
                        panic!("simulate fetch thread panic");
                    }
                    (l, Ok(vec![1, 2, 3]))
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().unwrap_or_else(|_| (String::new(), Err(()))))
            .collect()
    });

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0, "ok");
    assert_eq!(results[0].1, Ok(vec![1u8, 2, 3]));
    assert_eq!(results[1].0, "");
    assert_eq!(results[1].1, Err(()));
}

/// Test: thread join().unwrap_or_else() passes through successful values.
#[test]
fn thread_join_ok_passthrough() {
    let results: Vec<(String, Option<Vec<u8>>)> = thread::scope(|s| {
        let handles: Vec<_> = vec!["a", "b"]
            .into_iter()
            .map(|label| {
                let l = label.to_string();
                s.spawn(move || -> (String, Option<Vec<u8>>) { (l, Some(vec![42])) })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().unwrap_or_else(|_| (String::new(), None)))
            .collect()
    });

    assert_eq!(results.len(), 2);
    assert_eq!(results[0], ("a".to_string(), Some(vec![42])));
    assert_eq!(results[1], ("b".to_string(), Some(vec![42])));
}
