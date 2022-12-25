use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialOrd, Ord, PartialEq)]
pub struct Key {
    idx: usize,
    // even = vacant, odd = occupied
    version: usize,
}

#[derive(Debug)]
pub struct Slot {
    idx_or_free: usize,
    version: usize,
}

#[derive(Debug)]
pub struct Arena<V> {
    values: Vec<V>,
    keys: Vec<Key>,
    slots: Vec<Slot>,
    free: usize,
}

impl<V> Default for Arena<V> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            keys: Default::default(),
            slots: Default::default(),
            free: 0,
        }
    }
}

impl<V> Arena<V> {
    #[must_use]
    pub fn insert(&mut self, value: V) -> Key {
        let key = match self.slots.get_mut(self.free) {
            Some(slot) if slot.version % 2 == 0 => {
                slot.version += 1;
                let key = Key {
                    idx: self.free,
                    version: slot.version,
                };
                self.free = slot.idx_or_free;
                slot.idx_or_free = self.values.len();
                key
            }
            _ => {
                self.slots.push(Slot {
                    version: 1,
                    idx_or_free: self.values.len(),
                });
                Key {
                    version: 1,
                    idx: self.slots.len() - 1,
                }
            }
        };
        self.values.push(value);
        self.keys.push(key);
        key
    }
    pub fn remove(&mut self, key: Key) -> Option<V> {
        if self.slots.get(key.idx)?.version != key.version {
            return None;
        }
        let idx = self.slots[key.idx].idx_or_free;
        self.slots[key.idx].version += 1;
        self.slots[key.idx].idx_or_free = self.free;
        self.free = key.idx;
        let _ = self.keys.swap_remove(idx);
        let value = self.values.swap_remove(idx);
        if idx < self.values.len() {
            // update slot if swap_remove swaped
            self.slots[self.keys[idx].idx].idx_or_free = idx;
        }
        Some(value)
    }
    pub fn get(&self, key: Key) -> Option<&V> {
        let slot = self.slots.get(key.idx)?;
        if slot.version == key.version && key.version % 2 != 0 {
            Some(self.values.get(slot.idx_or_free)?)
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, key: Key) -> Option<&mut V> {
        let slot = self.slots.get(key.idx)?;
        if slot.version == key.version && key.version % 2 != 0 {
            Some(self.values.get_mut(slot.idx_or_free)?)
        } else {
            None
        }
    }
    pub fn as_slice(&self) -> (&[Key], &[V]) {
        (&self.keys, &self.values)
    }
    pub fn values_as_slice(&self) -> &[V] {
        &self.values
    }
    pub fn keys_as_slice(&self) -> &[Key] {
        &self.keys
    }
    pub fn dense_index(&self, key: Key) -> usize {
        self.get_dense_index(key).unwrap()
    }
    pub fn get_dense_index(&self, key: Key) -> Option<usize> {
        let slot = self.slots.get(key.idx)?;
        if slot.version == key.version && key.version % 2 != 0 {
            Some(slot.idx_or_free)
        } else {
            None
        }
    }
    pub fn key(&self, dense_key: usize) -> Key {
        self.get_key(dense_key).unwrap()
    }
    pub fn get_key(&self, dense_key: usize) -> Option<Key> {
        self.keys.get(dense_key).map(|k| *k)
    }
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.values.iter()
    }
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.values.iter_mut()
    }
    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.keys.iter()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&Key, &V)> {
        self.keys.iter().map(|key| (key, self.get(*key).unwrap()))
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<V> Index<Key> for Arena<V> {
    type Output = V;

    fn index(&self, key: Key) -> &Self::Output {
        self.get(key).unwrap()
    }
}

impl<V> IndexMut<Key> for Arena<V> {
    fn index_mut(&mut self, key: Key) -> &mut Self::Output {
        self.get_mut(key).unwrap()
    }
}
