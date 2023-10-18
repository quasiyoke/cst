use std::{borrow::Borrow, hash::Hash, sync::Arc};

use engine::{External, ExternalEngine, Local, LocalEngine};
pub use key::ExternalKey;

pub mod engine;
mod key;

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

pub trait KeyValueStorage<K, V> {
    /// Receives the `value` enclosed in the `Arc` to maximize user's freedom of decision regarding allocation.
    fn insert(&self, key: K, value: Arc<V>);

    /// An implementation of `ExternalKey` for `Q` must give identical results as `K`'s one.
    /// This requirement, which users must fulfill, is of course a potential source of errors,
    /// but it is no worse than the Rust's requirement imposed on the `Hash` implementation.
    fn get<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: ExternalKey + Eq + Hash + ?Sized;

    /// An implementation of `ExternalKey` for `Q` must give identical results as `K`'s one.
    fn remove<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: ExternalKey + Eq + Hash + ?Sized;
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
    K: ExternalKey,
{
    fn insert(&self, key: K, value: Arc<V>) {
        self.engine.insert(&key.external_key(), &value)
    }

    fn get<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        Q: ExternalKey + ?Sized,
    {
        self.engine.get(&key.external_key()).map(Into::into)
    }

    fn remove<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        Q: ExternalKey + ?Sized,
    {
        self.engine.remove(&key.external_key()).map(Into::into)
    }
}
