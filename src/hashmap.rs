use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct HashMapItem<K, V> {
    key: K,
    value: V,
    next: Option<Box<HashMapItem<K, V>>>,
}

#[derive(Debug, Clone)]
pub struct HashMap<K, V> {
    size: usize,
    buckets: Box<[Option<Box<HashMapItem<K, V>>>]>,
    // hasher: DefaultHasher,
}

#[derive(Debug)]
pub struct HashMapIterator<'a, K, V> {
    hashmap: &'a HashMap<K, V>,
    bucket_idx: usize,
    item: Option<&'a HashMapItem<K, V>>,
}

impl<'a, K, V> Iterator for HashMapIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let current_item = self.item.map(|x| (&x.key, &x.value));

        self.item = self.item.and_then(|x| x.next.as_ref().map(|x| x.as_ref()));
        if self.item.is_some() {
            return current_item;
        }

        let Some(next_idx) = self.hashmap.buckets[self.bucket_idx + 1..]
            .iter()
            .position(|x| x.is_some())
        else {
            return current_item;
        };

        self.bucket_idx += next_idx + 1;
        self.item = self.hashmap.buckets[self.bucket_idx]
            .as_ref()
            .map(|x| x.as_ref());

        current_item
    }
}

impl<K: Clone, V: Clone> HashMapItem<K, V> {
    pub fn new(key: K, value: V, next: Option<Box<HashMapItem<K, V>>>) -> HashMapItem<K, V> {
        HashMapItem { key, value, next }
    }
}

impl<K, V> HashMap<K, V> {
    pub fn new(size: usize) -> HashMap<K, V> {
        HashMap {
            size: 0,
            buckets: (0..size).map(|_| None).collect(),
            // hasher: DefaultHasher::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn iter(&self) -> HashMapIterator<K, V> {
        let (idx, first_item) = self
            .buckets
            .iter()
            .position(|x| x.is_some())
            .map(|idx| (idx, self.buckets[idx].as_ref().map(|x| x.as_ref())))
            .unwrap_or((0, None));

        let iterator = HashMapIterator {
            hashmap: self,
            bucket_idx: idx,
            item: first_item,
        };

        iterator
    }
}

impl<K: PartialEq + Hash + Clone, V: Clone> HashMap<K, V> {
    fn hash(&mut self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }

    fn get_mut_finger(&mut self, key: &K) -> &mut Option<Box<HashMapItem<K, V>>> {
        let idx = self.hash(key) % self.buckets.len();

        let mut finger = &mut self.buckets[idx];
        loop {
            match finger {
                None => break,
                Some(x) if x.key == *key => break,
                Some(x) => finger = &mut x.next,
            };
        }

        finger
    }

    // TODO: Figure out how to take `&self` instead of `&mut self` without
    // copying over the whole body
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.get_mut_finger(key).as_ref().map(|x| &x.value)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut_finger(key).as_mut().map(|x| &mut x.value)
    }

    pub fn set(&mut self, key: &K, value: V) {
        let finger = self.get_mut_finger(key);

        match finger {
            None => {
                *finger = Some(Box::from(HashMapItem::new(key.clone(), value, None)));
                self.size += 1;
            }
            Some(x) => x.value = value,
        };
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket_count = self.buckets.len();
        let idx = self.hash(key) % bucket_count;

        let mut finger = &mut self.buckets[idx];
        if finger.as_ref().is_some_and(|x| x.key == *key) {
            // Is there any way to get rid of this cloning?
            let val = finger.as_ref().map(|x| x.value.to_owned());
            *finger = finger.as_mut().and_then(|x| x.next.take());
            self.size -= 1;
            return val;
        }

        loop {
            let is_next_matching = finger
                .as_ref()
                .is_some_and(|x| x.next.as_ref().is_some_and(|x| x.key == *key));
            if is_next_matching {
                break;
            }

            match finger {
                None => break,
                Some(x) => finger = &mut x.next,
            }
        }

        // Is there any way to get rid of this cloning?
        let val = finger.as_ref().map(|x| x.value.to_owned());

        match finger {
            None => {}
            Some(x) => {
                let next = x.next.as_mut().and_then(|y| y.next.take());
                x.next = next;
                self.size -= 1;
            }
        };

        val
    }

    pub fn rehash(&mut self, _hasher: DefaultHasher) {
        let old_buckets = self.buckets.clone();
        self.buckets = (0..self.buckets.len()).map(|_| None).collect();
        // self.hasher = hasher;

        for bucket in old_buckets.iter() {
            let mut item = bucket.as_ref().map(|x| x.as_ref());
            while let Some(x) = item {
                self.set(&x.key, x.value.clone());
                item = x.next.as_ref().map(|x| x.as_ref());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn len_method_works() {
        let mut map = super::HashMap::new(3);
        map.set(&String::from("foo"), 1);
        assert_eq!(map.len(), 1);
        map.set(&String::from("bar"), 2);
        assert_eq!(map.len(), 2);
        map.set(&String::from("baz"), 3);
        assert_eq!(map.len(), 3);
        map.set(&String::from("Hello!"), 42);
        assert_eq!(map.len(), 4);
        map.remove(&String::from("foo"));
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn set_get_methods_work() {
        let mut map = super::HashMap::new(10);
        map.set(&String::from("aaa"), 10);
        map.set(&String::from("bbb"), 20);
        map.set(&String::from("ccc"), 30);
        map.set(&String::from("ddd"), 40);

        assert_eq!(map.get(&String::from("aaa")), Some(&10));
        assert_eq!(map.get(&String::from("bbb")), Some(&20));
        assert_eq!(map.get(&String::from("ccc")), Some(&30));
        assert_eq!(map.get(&String::from("ddd")), Some(&40));
    }

    #[test]
    fn get_mut_method_works() {
        let mut map = super::HashMap::new(5);
        map.set(&String::from("aaa"), 10);
        let _ = map.get_mut(&String::from("aaa")).map(|x| *x = 20);
        assert_eq!(map.get(&String::from("aaa")), Some(&20));
    }

    #[test]
    fn remove_method_works() {
        let mut map = super::HashMap::new(10);
        map.set(&String::from("aaa"), 10);
        map.set(&String::from("bbb"), 20);
        map.set(&String::from("ccc"), 30);
        map.set(&String::from("ddd"), 40);

        assert_eq!(map.get(&String::from("aaa")), Some(&10));
        map.remove(&String::from("aaa"));
        assert_eq!(map.get(&String::from("aaa")), None);
        assert_eq!(map.get(&String::from("bbb")), Some(&20));
        map.remove(&String::from("bbb"));
        assert_eq!(map.get(&String::from("bbb")), None);
        assert_eq!(map.get(&String::from("ccc")), Some(&30));
        map.remove(&String::from("ccc"));
        assert_eq!(map.get(&String::from("ccc")), None);
        assert_eq!(map.get(&String::from("ddd")), Some(&40));
        map.remove(&String::from("ddd"));
        assert_eq!(map.get(&String::from("ddd")), None);
    }

    #[test]
    fn rehash_method_works() {
        let mut map = super::HashMap::new(2);
        map.set(&String::from("aaa"), 10);
        map.set(&String::from("bbb"), 20);
        map.set(&String::from("ccc"), 30);
        map.set(&String::from("ddd"), 40);

        map.rehash(super::DefaultHasher::new());

        assert_eq!(map.get(&String::from("aaa")), Some(&10));
        assert_eq!(map.get(&String::from("bbb")), Some(&20));
        assert_eq!(map.get(&String::from("ccc")), Some(&30));
        assert_eq!(map.get(&String::from("ddd")), Some(&40));
    }

    #[test]
    fn iterator_few_buckets_works() {
        let mut items = vec![
            (String::from("foo"), 99),
            (String::from("bar"), -5),
            (String::from("baz"), 42),
        ];
        items.sort();

        let mut map = super::HashMap::new(2);
        for (k, v) in items.iter() {
            map.set(k, *v);
        }
        let mut got_items = map.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<_>>();
        got_items.sort();

        assert_eq!(got_items.len(), items.len());
        assert_eq!(got_items, items);
    }

    #[test]
    fn iterator_many_buckets_works() {
        let mut items = vec![
            (String::from("foo"), 99),
            (String::from("bar"), -5),
            (String::from("baz"), 42),
        ];
        items.sort();

        let mut map = super::HashMap::new(10);
        for (k, v) in items.iter() {
            map.set(k, *v);
        }
        let mut got_items = map.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<_>>();
        got_items.sort();

        assert_eq!(got_items.len(), items.len());
        assert_eq!(got_items, items);
    }
}
