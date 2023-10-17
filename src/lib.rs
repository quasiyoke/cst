use engine::ExternalStorage;

pub mod engine;

pub struct Storage<Engine> {
    engine: Engine,
}

impl<Engine> Storage<Engine> {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }
}

pub trait KeyValueStorage<T>: Send + Sync {
    fn insert(&self, key: &str, data: &T);

    fn get(&self, key: &str) -> Option<T>;

    fn remove(&self, key: &str) -> Option<T>;
}

impl<Engine, T> KeyValueStorage<T> for Storage<Engine>
where
    Engine: ExternalStorage<T> + Send + Sync,
{
    fn insert(&self, key: &str, value: &T) {
        self.engine.insert(key, value)
    }

    fn get(&self, key: &str) -> Option<T> {
        self.engine.get(key)
    }

    fn remove(&self, key: &str) -> Option<T> {
        self.engine.remove(key)
    }
}
