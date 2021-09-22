use std::{collections::HashMap as Map, hash::Hash};

/// LayerMap to resolve paths, identifieres and fields from
pub struct LayerMap<'a, K, V> where K: Hash + Eq {
    outer: Option<&'a LayerMap<'a, K, V>>,
    items: Map<K, V>,
}

impl<'a, K, V> LayerMap<'a, K, V> where K: Hash + Eq {
    /// Create a new LayerMap with the content of the current ones,
    /// that can be modified without modifying the current one
    pub fn layer(&'a self) -> Self {
        LayerMap {
            outer: Some(self),
            items: Map::new(),
        }
    }

    pub fn new() -> Self {
        let items = Map::new();
        let outer: Option<&'a LayerMap<'a, K, V>>= None;
        LayerMap { outer, items }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.items.get(key).or_else(|| self.outer?.get(key))
    }

    /// Insert value V, overwrite the key, if previously existing
    pub fn insert(&mut self, key: K, value: V) {
        self.items.insert(key, value);
    }
}
