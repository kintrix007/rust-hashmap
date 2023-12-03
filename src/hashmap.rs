#[derive(Debug, Clone, PartialEq, Eq)]
struct HashMapItem<K, T> {
    key: K,
    value: T,
    next: Option<Box<HashMapItem<K, T>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashMap<K, V> {
    size: usize,
    buckets: Box<[Option<Box<HashMapItem<K, V>>>]>,
    hash: fn(&K) -> usize,
}

fn default_hash(x: &String) -> usize {
    x.chars()
        .map(|x| x as usize)
        .reduce(|acc, x| acc * 31 + x)
        .unwrap_or(0)
}

impl HashMapItem<String, i32> {
    pub fn from(
        key: String,
        value: i32,
        next: Option<Box<HashMapItem<String, i32>>>,
    ) -> HashMapItem<String, i32> {
        HashMapItem { key, value, next }
    }
}

impl HashMap<String, i32> {
    pub fn new(size: usize) -> HashMap<String, i32> {
        HashMap {
            size: 0,
            buckets: vec![None; size].into_boxed_slice(),
            hash: default_hash,
        }
    }

    fn get_mut_finger(&mut self, key: &String) -> &mut Option<Box<HashMapItem<String, i32>>> {
        let bucket_count = self.buckets.len();
        let idx = (self.hash)(&key) % bucket_count;

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

    // TODO: Figure out how to take `$self` instead of `&mut self` without
    // copying over the whole body
    pub fn get(&mut self, key: &String) -> Option<&i32> {
        self.get_mut_finger(key).as_ref().map(|x| &x.value)
    }
    
    pub fn get_mut(&mut self, key: &String) -> Option<&mut i32> {
        self.get_mut_finger(key).as_mut().map(|x| &mut x.value)
    }

    pub fn set(&mut self, key: String, value: i32) {
        let finger = self.get_mut_finger(&key);

        let value = match finger {
            None => HashMapItem::from(key, value, None),
            Some(x) => HashMapItem::from(key, value, x.next.take()),
        };
        *finger = Some(Box::from(value));
    }

    pub fn remove(&mut self, key: &String) -> Option<i32> {
        let bucket_count = self.buckets.len();
        let idx = (self.hash)(&key) % bucket_count;

        let mut finger = &mut self.buckets[idx];
        let mut prev = &finger;
        loop {
            match finger {
                None => break,
                Some(x) if x.key == *key => break,
                Some(x) => {
                    finger = &mut x.next;
                    prev = &finger;
                }
            };
        }

        todo!();
    }
}
