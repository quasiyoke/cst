use std::{ops::Range, thread};

use storage::KeyValueStorage;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Data {
    field: i32,
}

pub fn test_key_value_storage_single_thread(s: impl KeyValueStorage<Data>) {
    test_key_value_storage_range(&s, 0..100_000)
}

pub fn test_key_value_storage_multi_thread(s: impl KeyValueStorage<Data> + 'static) {
    thread::scope(|scope| {
        let even = scope.spawn(|| {
            test_key_value_storage_range(&s, 0..100_000);
        });
        let odd = scope.spawn(|| {
            test_key_value_storage_range(&s, 900_000..1_000_000);
        });
        assert!(matches!(even.join(), Ok(_)), "even");
        assert!(matches!(odd.join(), Ok(_)), "odd");
    });
}

pub fn test_key_value_storage_range(s: &impl KeyValueStorage<Data>, r: Range<i32>) {
    // Insert in reverse order, negated
    for i in r.clone().rev() {
        s.insert(&key(i), &Data { field: -i });
    }
    // Check in direct order
    assert_eq!(s.get(&key(r.start - 1)), None, "check in direct order");
    for i in r.clone() {
        let actual = s.get(&key(i));
        assert_eq!(actual, Some(Data { field: -i }), "check in direct order");
    }
    assert_eq!(s.get(&key(r.end)), None, "check in direct order");
    // Replace in direct order, doubled
    for i in r.clone() {
        s.insert(&key(i), &Data { field: i * 2 });
    }
    // Remove in reverse order
    assert_eq!(s.remove(&key(r.end)), None, "remove in reverse order");
    for i in r.clone().rev() {
        let actual = s.remove(&key(i));
        assert_eq!(
            actual,
            Some(Data { field: i * 2 }),
            "remove in reverse order"
        );
    }
    assert_eq!(s.remove(&key(r.start - 1)), None, "remove in reverse order");
    assert_eq!(s.get(&key(r.start)), None, "check after remove");
}

fn key(n: i32) -> String {
    format!("k{n}")
}
