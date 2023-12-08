use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Clone)]
struct HashMapItem<K: Clone, V: Clone> {
    key: K,
    value: V,
    next: Option<Box<HashMapItem<K, V>>>,
}

#[derive(Debug)]
pub struct HashMap<K: PartialEq + Hash + Clone, V: Clone> {
    size: usize,
    buckets: Box<[Option<Box<HashMapItem<K, V>>>]>,
}

impl<K: PartialEq + Hash + Clone, V: PartialEq + Clone> PartialEq for HashMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        *self.items() == *other.items()
    }
}

impl<K: Clone, V: Clone> HashMapItem<K, V> {
    pub fn from(key: K, value: V, next: Option<Box<HashMapItem<K, V>>>) -> HashMapItem<K, V> {
        HashMapItem { key, value, next }
    }
}

impl<K: PartialEq + Hash + Clone, V: Clone> HashMap<K, V> {
    pub fn new(size: usize) -> HashMap<K, V> {
        HashMap {
            size: 0,
            buckets: vec![None; size].into_boxed_slice(),
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    fn hash(&self, key: &K) -> usize {
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
                *finger = Some(Box::from(HashMapItem::from(key.clone(), value, None)));
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

    // Instead return an iterator... That would make much more sense
    pub fn items(&self) -> Vec<(&K, &V)> {
        todo!();
    }

    pub fn rehash(&mut self) {
        todo!();
    }
}
