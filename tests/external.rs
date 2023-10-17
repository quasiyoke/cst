use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;

use storage::{engine::ExternalEngine, Storage};

use util::{test_key_value_storage_multi_thread, test_key_value_storage_single_thread};

mod util;

#[derive(Clone, Default)]
struct RedisMock<T> {
    map: Arc<RwLock<HashMap<String, T>>>,
}

impl<T> ExternalEngine<T> for RedisMock<T>
where
    T: Clone,
{
    fn insert(&self, id: &str, data: &T) {
        let id = id.to_owned();
        self.map.write().insert(id, data.clone());
    }

    fn get(&self, id: &str) -> Option<T> {
        self.map.read().get(id).cloned()
    }

    fn remove(&self, id: &str) -> Option<T> {
        self.map.write().remove(id)
    }
}

#[test]
fn single_thread() {
    let s = Storage::new(RedisMock::default());
    test_key_value_storage_single_thread(s);
}

#[test]
fn multi_thread() {
    let s = Storage::new(RedisMock::default());
    test_key_value_storage_multi_thread(s);
}
