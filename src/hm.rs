use std::hash::{DefaultHasher, Hash, Hasher};

pub struct SimpleHashMap<K: Hash + Eq, V> {
    buckets: Vec<(K, V)>,
    cap: usize,
    len: usize,
}

impl<K: Hash + Eq, V> SimpleHashMap<K, V> {
    pub fn new(cap: usize) -> Self {
        SimpleHashMap {
            buckets: Vec::with_capacity(cap),
            cap,
            len: 0,
        }
    }

    fn position(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.cap
    }

    pub fn insert(&mut self, key: K, value: V) -> usize {
        let pos = self.position(&key);
        self.buckets.insert(pos, (key, value));
        self.len += 1;
        self.len
    }

    pub fn delete(&mut self, key: K) -> Option<(&K, &V)> {
        todo!()
    }

    pub fn get(&self, key: K) -> Option<(&K, &V)> {
        let pos = self.position(&key);
        let res = self.buckets.get(pos);
        match res {
            Some((k, v)) => Some((k, v)),
            None => None,
        }
    }
}
