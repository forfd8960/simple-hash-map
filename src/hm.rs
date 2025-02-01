use std::{
    collections::LinkedList,
    hash::{DefaultHasher, Hash, Hasher},
};

pub struct SimpleHashMap<K: Hash + Eq + Clone, V: Clone> {
    buckets: Vec<Bucket<K, V>>,
    cap: usize,
    len: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Bucket<K, V> {
    kv_list: LinkedList<(K, V)>,
}

impl<K: Hash + Eq + Clone, V: Clone> Bucket<K, V> {
    fn new() -> Self {
        let kv_list: LinkedList<(K, V)> = LinkedList::new();
        Bucket { kv_list }
    }

    fn add(&mut self, key: K, value: V) {
        self.kv_list.push_back((key, value));
    }

    fn get(&self, key: K) -> Option<(K, V)> {
        for (k, v) in self.kv_list.clone() {
            if k == key {
                return Some((k.clone(), v.clone()));
            }
        }
        None
    }

    fn remove(&mut self, key: K) {
        let kv_list = self
            .kv_list
            .clone()
            .into_iter()
            .filter(|(k, _)| k != &key)
            .collect();
        self.kv_list = kv_list;
    }
}

impl<K: Hash + Eq + Clone, V: Clone> SimpleHashMap<K, V> {
    pub fn new(cap: usize) -> Self {
        let mut buckets: Vec<Bucket<K, V>> = Vec::with_capacity(cap);
        for _ in 0..cap {
            buckets.push(Bucket::new());
        }

        SimpleHashMap {
            buckets,
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
        let bucket = self.buckets.get_mut(pos).unwrap();
        bucket.add(key, value);

        self.len += 1;
        self.len
    }

    pub fn delete(&mut self, key: K) -> usize {
        let pos = self.position(&key);
        let bucket = self.buckets.get_mut(pos).unwrap();
        bucket.remove(key);
        self.len -= 1;
        self.len
    }

    pub fn get(&self, key: K) -> Option<(K, V)> {
        let pos = self.position(&key);
        let res = self.buckets.get(pos).unwrap();
        res.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleHashMap;

    #[test]
    fn test_hash_map_insert_get() {
        let mut map = SimpleHashMap::new(10);
        map.insert("A", 1);
        map.insert("B", 2);
        map.insert("C", 3);
        assert_eq!(map.len, 3);

        let res = map.get("A");
        assert_eq!(res, Some(("A", 1)));
    }

    #[test]
    fn test_hash_map_insert_delete() {
        let mut map = SimpleHashMap::new(10);
        map.insert("A", 1);
        map.insert("B", 2);
        map.insert("C", 3);
        assert_eq!(map.len, 3);

        let len = map.delete("A");
        assert_eq!(len, 2);

        let v = map.get("A");
        assert_eq!(v, None);
    }
}
