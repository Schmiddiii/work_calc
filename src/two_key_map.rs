use std::collections::HashMap;
use std::hash::Hash;

pub struct TwoKeyMap<A: Eq + Hash + Clone, B: Eq + Hash + Clone, C: Clone> {
    hash_map_a: HashMap<A, Vec<(B, C)>>,
    hash_map_b: HashMap<B, Vec<(A, C)>>

}

impl<A, B, C> TwoKeyMap<A, B, C>
where A: Eq + Hash + Clone,
      B: Eq + Hash + Clone,
      C: Clone
{
    pub fn new() -> TwoKeyMap<A, B, C> {
        TwoKeyMap {
            hash_map_a: HashMap::new(),
            hash_map_b: HashMap::new()
        }
    }


    pub fn insert(&mut self, key_a: A, key_b: B, value: C) {
        let values_a = self.hash_map_a.get_mut(&key_a);
        if values_a.is_none() {
            self.hash_map_a.insert(key_a.clone(), vec![(key_b.clone(), value.clone())]);
        } else {
            values_a.unwrap().push((key_b.clone(), value.clone()));
        }

        let values_b = self.hash_map_b.get_mut(&key_b);
        if values_b.is_none() {
            self.hash_map_b.insert(key_b, vec![(key_a, value)]);
        } else {
            values_b.unwrap().push((key_a, value));
        }
    }

    pub fn get_from_a(&self, key: &A) -> Option<&Vec<(B,C)>> {
        self.hash_map_a.get(key)
    }

    pub fn get_from_b(&self, key: &B) -> Option<&Vec<(A,C)>> {
        self.hash_map_b.get(key)
    }

}

