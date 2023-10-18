use std::{borrow::Borrow, collections::HashMap, hash::Hash, sync::Arc};

use parking_lot::RwLock;

pub struct Local;

pub struct External;

/// To store values in RAM.
pub trait LocalEngine {
    type Key;
    type Value;

    fn insert(&self, key: Self::Key, value: Arc<Self::Value>);

    fn get<Q>(&self, key: &Q) -> Option<Arc<Self::Value>>
    where
        Self::Key: Borrow<Q>,
        Q: Eq + Hash + ?Sized;

    fn remove<Q>(&self, key: &Q) -> Option<Arc<Self::Value>>
    where
        Self::Key: Borrow<Q>,
        Q: Eq + Hash + ?Sized;
}

/// To store values remotely.
pub trait ExternalEngine {
    type Value;

    fn insert(&self, key: &str, value: &Self::Value);

    fn get(&self, key: &str) -> Option<Self::Value>;

    fn remove(&self, key: &str) -> Option<Self::Value>;
}

#[derive(Clone, Default)]
pub struct HashMapEngine<K, V> {
    inner: Arc<RwLock<HashMap<K, Arc<V>>>>,
}

impl<K, V> LocalEngine for HashMapEngine<K, V>
where
    K: Eq + Hash,
{
    type Key = K;
    type Value = V;

    fn insert(&self, key: K, value: Arc<V>) {
        self.inner.write().insert(key, value);
    }

    fn get<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.read().get(key).cloned()
    }

    fn remove<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.write().remove(key)
    }
}
