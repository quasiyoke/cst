pub trait LocalStorage<T> {
    fn insert(&self, key: &str, value: &T);

    fn get(&self, key: &str) -> Option<T>;

    fn remove(&self, key: &str) -> Option<T>;
}

pub trait ExternalStorage<T> {
    fn insert(&self, key: &str, value: &T);

    fn get(&self, key: &str) -> Option<T>;

    fn remove(&self, key: &str) -> Option<T>;
}
