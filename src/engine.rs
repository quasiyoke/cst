use std::{borrow::Borrow, hash::Hash, sync::Arc};

/// To store values in RAM.
pub trait LocalEngine<K, V> {
    fn insert(&self, key: K, value: Arc<V>);

    fn get<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized;

    fn remove<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized;
}

/// To store values remotely.
pub trait ExternalEngine<T> {
    fn insert(&self, key: &str, value: &T);

    fn get(&self, key: &str) -> Option<T>;

    fn remove(&self, key: &str) -> Option<T>;
}
