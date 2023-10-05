use crate::int::BfValue;

pub struct KVVec<K: BfValue, V: BfValue> {
    vec: Vec<(K, V)>,
}

impl<K: BfValue, V: BfValue> KVVec<K, V> {
    pub fn new() -> Self {
        KVVec { vec: Vec::new() }
    }

    pub fn len(&self) -> i32 {
        self.vec.len() as i32
    }

    pub fn into_boxed_slice(self) -> Box<[(K, V)]> {
        self.vec.into_boxed_slice()
    }

    pub fn push(&mut self, key: K, value: V) {
        self.vec.push((key, value));
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (K, V)> {
        self.vec.iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, (K, V)> {
        self.vec.iter_mut()
    }

    pub fn get(&self, key: K) -> V {
        for (k, v) in self.iter() {
            if *k == key {
                return *v;
            }
        }
        num::zero()
    }

    /*pub fn set(&mut self, key: K, value: V) {
        for (k, v) in self.iter_mut() {
            if *k == key {
                *v = value;
                return;
            }
        }
        self.push(key, value);
    }*/

    pub fn incr(&mut self, key: K) {
        for (k, v) in self.iter_mut() {
            if *k == key {
                *v = v.wrapping_add(&num::one());
                return;
            }
        }
        self.push(key, num::one());
    }

    pub fn decr(&mut self, key: K) {
        for (k, v) in self.iter_mut() {
            if *k == key {
                *v = v.wrapping_sub(&num::one());
                return;
            }
        }
        let zero: V = num::zero();
        self.push(key, zero.wrapping_sub(&num::one()));
    }
}
