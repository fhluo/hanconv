use ahash::AHashMap;

#[derive(Default)]
pub struct Node<T> {
    children: Option<AHashMap<char, Node<T>>>,
    value: Option<T>,
}

impl<T> From<Node<T>> for AHashMap<String, T> {
    fn from(node: Node<T>) -> Self {
        let mut map = AHashMap::default();

        fn build<T>(map: &mut AHashMap<String, T>, node: Node<T>, key: String) {
            node.value.map(|value| map.insert(key.clone(), value));

            if let Some(children) = node.children {
                children.into_iter().for_each(|(char, node)| {
                    build(map, node, {
                        let mut key = key.clone();
                        key.push(char);
                        key
                    })
                })
            }
        }

        build(&mut map, node, String::new());

        map
    }
}

#[derive(Default)]
pub struct Trie<T> {
    root: Node<T>,
    depth: usize,
}

impl<T> Trie<T> {
    pub fn get_depth(&self) -> usize {
        self.depth
    }
}

impl<T: Default> Trie<T> {
    pub fn insert(&mut self, key: &str, value: T) {
        if key.is_empty() {
            return;
        }

        let mut node = &mut self.root;
        let mut depth = 0usize;

        for char in key.chars() {
            node = node
                .children
                .get_or_insert_default()
                .entry(char)
                .or_default();

            depth += 1;
        }

        if depth > self.depth {
            self.depth = depth;
        }
        node.value = Some(value);
    }
}

impl<T> Trie<T> {
    pub fn get(&mut self, key: &str) -> Option<&T> {
        let mut node = &self.root;

        for char in key.chars() {
            if let Some(children) = &node.children
                && let Some(child) = children.get(&char)
            {
                node = child;
            } else {
                return None;
            }
        }

        node.value.as_ref()
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;

        for char in prefix.chars() {
            if let Some(children) = &node.children
                && let Some(child) = children.get(&char)
            {
                node = child;
            } else {
                return false;
            }
        }

        true
    }

    pub fn r#match(&self, chars: impl Iterator<Item = char>) -> Option<(&T, usize)> {
        let mut node = &self.root;
        let mut result = None;

        for (char, i) in chars.take(self.depth).zip(1..) {
            if let Some(children) = &node.children
                && let Some(child) = children.get(&char)
            {
                node = child;

                if let Some(value) = node.value.as_ref() {
                    result = Some((value, i));
                }
            } else {
                return result;
            }
        }

        result
    }
}

impl<T: AsRef<str>> Trie<T> {
    pub fn convert(&self, s: impl AsRef<str>) -> String {
        let s = s.as_ref();
        let mut iter = s.chars().peekable();
        let mut dst = String::with_capacity(s.len());

        while iter.peek().is_some() {
            if let Some((r, n)) = self.r#match(iter.clone()) {
                dst += r.as_ref();
                iter.by_ref().nth(n - 1);
            } else {
                dst.push(iter.next().unwrap());
            }
        }

        dst
    }
}

impl<T> From<Trie<T>> for AHashMap<String, T> {
    fn from(trie: Trie<T>) -> Self {
        trie.root.into()
    }
}

impl<K: AsRef<str>, V: Default> From<AHashMap<K, V>> for Trie<V> {
    fn from(map: AHashMap<K, V>) -> Self {
        let mut trie = Trie::default();

        for (key, value) in map {
            trie.insert(key.as_ref(), value);
        }

        trie
    }
}

impl<K: AsRef<str>, V: Default> FromIterator<(K, V)> for Trie<V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut trie = Trie::default();

        for (key, value) in iter {
            trie.insert(key.as_ref(), value);
        }

        trie
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::default();

        trie.insert("一分钟", "一分鐘");

        assert_eq!(trie.get("一分钟"), Some("一分鐘").as_ref());
        assert_eq!(trie.convert("一分钟"), "一分鐘");
    }
}
