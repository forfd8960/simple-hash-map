use std::{
    collections::LinkedList,
    hash::{DefaultHasher, Hash, Hasher},
};

const DEFAULT_LOAD_FACTOR: f64 = 0.75;

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

    fn get_all_elements(&self) -> Vec<(K, V)> {
        let mut res = Vec::new();
        for (k, v) in self.kv_list.clone() {
            res.push((k.clone(), v.clone()));
        }
        res
    }
}

impl<K: Hash + Eq + Clone + std::fmt::Debug, V: Clone> SimpleHashMap<K, V> {
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

    fn position(&self, key: &K, length: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % length
    }

    pub fn insert(&mut self, key: K, value: V) -> usize {
        if self.len as f64 / self.cap as f64 > DEFAULT_LOAD_FACTOR {
            println!("trigger resize... to cap: {}", self.cap * 2);

            let mut new_buckets = Vec::with_capacity(self.cap * 2);
            let all_k_v = self.all_key_values();
            for _ in 0..self.cap * 2 {
                new_buckets.push(Bucket::new());
            }

            for (k, v) in all_k_v {
                let pos = self.position(&k, new_buckets.len());
                println!("moving key {:?} to pos: {}", k, pos);

                let bucket = new_buckets.get_mut(pos).unwrap();
                bucket.add(k, v);
            }

            self.cap *= 2;
            self.buckets = new_buckets;
        }

        let pos = self.position(&key, self.buckets.len());
        println!("insert {:?} to map pos: {}", key, pos);

        let bucket = self.buckets.get_mut(pos).unwrap();
        bucket.add(key, value);

        self.len += 1;
        self.len
    }

    pub fn delete(&mut self, key: K) -> usize {
        let pos = self.position(&key, self.buckets.len());
        let bucket = self.buckets.get_mut(pos).unwrap();
        bucket.remove(key);
        self.len -= 1;
        self.len
    }

    pub fn get(&self, key: K) -> Option<(K, V)> {
        let pos = self.position(&key, self.buckets.len());
        let res = self.buckets.get(pos).unwrap();
        res.get(key)
    }

    pub fn all_key_values(&self) -> Vec<(K, V)> {
        let mut res = Vec::new();
        for bucket in self.buckets.iter() {
            res.extend(bucket.get_all_elements());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleHashMap;
    use std::collections::HashSet;
    use std::hash::Hash;

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

    #[test]
    fn test_hash_map_get_all_k_v() {
        let mut map = SimpleHashMap::new(10);
        map.insert("A", 1);
        map.insert("B", 2);
        map.insert("C", 3);
        assert_eq!(map.len, 3);

        let res = map.all_key_values();
        assert_eq!(res.len(), 3);

        assert!(compare_vec_ignore_order(
            res,
            vec![("A", 1), ("B", 2), ("C", 3)]
        ));
    }

    fn compare_vec_ignore_order<K: Hash + Eq>(v1: Vec<(K, i32)>, v2: Vec<(K, i32)>) -> bool {
        let s1: HashSet<_> = v1.iter().collect();
        let s2: HashSet<_> = v2.iter().collect();
        s1 == s2
    }

    #[test]
    fn test_hash_map_resize() {
        let mut map = SimpleHashMap::new(4);
        for i in 0..10 {
            map.insert(format!("Key-{}", i), i);
        }

        assert_eq!(map.len, 10);

        let res = map.all_key_values();
        assert_eq!(res.len(), 10);

        assert!(compare_vec_ignore_order(
            res,
            (0..10)
                .into_iter()
                .map(|i| (format!("Key-{}", i), i))
                .collect::<Vec<(String, i32)>>()
        ));
    }
}
