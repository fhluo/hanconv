use std::collections::HashMap;

#[cfg(feature = "ahash")]
use ahash::RandomState;

#[cfg(not(feature = "ahash"))]
use std::collections::hash_map::RandomState;

pub(crate) type Map<K, V> = HashMap<K, V, RandomState>;
