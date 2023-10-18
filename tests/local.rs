use storage::{engine::HashMapEngine, Storage};

use util::{test_key_value_storage_multi_thread, test_key_value_storage_single_thread};

mod util;

#[test]
fn single_thread() {
    let s = Storage::with_local(HashMapEngine::default());
    test_key_value_storage_single_thread(s);
}

#[test]
fn multi_thread() {
    let s = Storage::with_local(HashMapEngine::default());
    test_key_value_storage_multi_thread(s);
}
