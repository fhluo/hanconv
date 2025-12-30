use ahash::AHashMap;

pub struct Node<T> {
    children: Option<AHashMap<char, Node<T>>>,
    value: Option<T>,
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Node {
            children: Default::default(),
            value: Default::default(),
        }
    }
}

impl<T> From<Node<T>> for AHashMap<String, T> {
    fn from(node: Node<T>) -> Self {
        let mut map = AHashMap::new();

        fn build<T>(map: &mut AHashMap<String, T>, node: Node<T>, key: &mut String) {
            if let Some(value) = node.value {
                map.insert(key.clone(), value);
            }

            if let Some(children) = node.children {
                for (c, node) in children {
                    key.push(c);
                    build(map, node, key);
                    key.pop();
                }
            }
        }

        let mut key = String::new();
        build(&mut map, node, &mut key);

        map
    }
}

pub struct Trie<T> {
    root: Node<T>,
    depth: usize,
}

impl<T> Default for Trie<T> {
    fn default() -> Self {
        Trie {
            root: Default::default(),
            depth: Default::default(),
        }
    }
}

impl<T> Trie<T> {
    pub fn get_depth(&self) -> usize {
        self.depth
    }
}

impl<T> Trie<T> {
    pub fn insert(&mut self, key: &str, value: T) {
        if key.is_empty() {
            return;
        }

        let mut node = &mut self.root;
        let mut depth = 0usize;

        for c in key.chars() {
            node = node.children.get_or_insert_default().entry(c).or_default();

            depth += 1;
        }

        if depth > self.depth {
            self.depth = depth;
        }
        node.value = Some(value);
    }
}

impl<T> Trie<T> {
    pub fn get(&self, key: &str) -> Option<&T> {
        let mut node = &self.root;

        for c in key.chars() {
            if let Some(children) = &node.children
                && let Some(child) = children.get(&c)
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

        for c in prefix.chars() {
            if let Some(children) = &node.children
                && let Some(child) = children.get(&c)
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

        for (c, i) in chars.take(self.depth).zip(1..) {
            if let Some(children) = &node.children
                && let Some(child) = children.get(&c)
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
    pub fn convert(&self, input: impl AsRef<str>) -> String {
        let input = input.as_ref();
        let mut iter = input.chars();
        let mut output = String::with_capacity(input.len());

        loop {
            if let Some((r, len)) = self.r#match(iter.clone()) {
                output.push_str(r.as_ref());
                iter.nth(len - 1);
            } else if let Some(c) = iter.next() {
                output.push(c);
            } else {
                break;
            }
        }

        output
    }
}

impl<T> From<Trie<T>> for AHashMap<String, T> {
    fn from(trie: Trie<T>) -> Self {
        trie.root.into()
    }
}

impl<K: AsRef<str>, V> From<AHashMap<K, V>> for Trie<V> {
    fn from(map: AHashMap<K, V>) -> Self {
        let mut trie = Trie::default();

        for (key, value) in map {
            trie.insert(key.as_ref(), value);
        }

        trie
    }
}

impl<K: AsRef<str>, V> FromIterator<(K, V)> for Trie<V> {
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
