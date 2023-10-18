use std::{borrow::Borrow, hash::Hash, sync::Arc};

use engine::{External, ExternalEngine, Local, LocalEngine};

pub mod engine;

pub struct Storage<Engine, EngineKind> {
    engine: Engine,
    _marker: EngineKind,
}

impl<Engine, K, V> Storage<Engine, Local>
where
    Engine: LocalEngine<Key = K, Value = V>,
{
    pub fn with_local(engine: Engine) -> Self {
        Self {
            engine,
            _marker: Local,
        }
    }
}

impl<Engine, T> Storage<Engine, External>
where
    Engine: ExternalEngine<Value = T>,
{
    pub fn with_external(engine: Engine) -> Self {
        Self {
            engine,
            _marker: External,
        }
    }
}

/// An implementation of `ToString` (`Display`) for `K`:
/// 1. Must give the same results for identical `K`.
/// 2. Must map a key to a string in a unique way (should not allow two different keys
///    to produce identical strings), without collisions.
pub trait KeyValueStorage<K, V> {
    /// Receives the `value` enclosed in the `Arc` to maximize user's freedom of decision regarding allocation.
    fn insert(&self, key: K, value: Arc<V>);

    /// An implementation of `ToString` (`Display`) for `Q` must give identical results as `K`'s one.
    /// This requirement, which users must fulfill, is of course a potential source of errors,
    /// but it is no worse than the Rust's requirement imposed on the `Hash` implementation.
    fn get<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ToString + ?Sized;

    /// An implementation of `ToString` (`Display`) for `Q` must give identical results as `K`'s one.
    fn remove<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ToString + ?Sized;
}

impl<K, V, Engine> KeyValueStorage<K, V> for Storage<Engine, Local>
where
    Engine: LocalEngine<Key = K, Value = V>,
    K: Eq + Hash,
{
    fn insert(&self, key: K, value: Arc<V>) {
        self.engine.insert(key, value)
    }

    fn get<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.engine.get(key)
    }

    fn remove<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.engine.remove(key)
    }
}

impl<K, V, Engine> KeyValueStorage<K, V> for Storage<Engine, External>
where
    Engine: ExternalEngine<Value = V>,
    K: ToString,
{
    fn insert(&self, key: K, value: Arc<V>) {
        self.engine.insert(&key.to_string(), &value)
    }

    fn get<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: ToString + ?Sized,
    {
        self.engine.get(&key.to_string()).map(Into::into)
    }

    fn remove<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: ToString + ?Sized,
    {
        self.engine.remove(&key.to_string()).map(Into::into)
    }
}
