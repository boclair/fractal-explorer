/// Implementation of a simple single element cache.
///
/// Returns the value given a key.
pub struct SingleCache<K, V> {
    cache_pair: Option<(K, V)>
}

impl<K: PartialEq, V: Clone> SingleCache<K, V> {
    pub fn new() -> Self {
        return SingleCache { cache_pair: None }
    }

    /// Returns the cached value of the given key matches the cached key.
    /// Otherwise uses the supplied funcation to generate a value, and
    /// stores that value along with the key.
    pub fn get_or_set<F>(&mut self, key: K, func: F) -> V where 
        F: Fn() -> V {
        if let Some(old_pair) = &self.cache_pair {
            if key == old_pair.0 {
                return old_pair.1.clone();
            }
        }

        let new_value = func();
        self.cache_pair = Some((key, new_value.clone()));
        new_value
    }
}
