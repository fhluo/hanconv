use ahash::AHashMap;

#[derive(Default)]
pub struct Node {
    children: Option<AHashMap<char, Node>>,
    value: Option<String>,
}

impl Node {
    fn build_map(&self, left: String, map: &mut AHashMap<String, String>) {
        if let Some(value) = &self.value {
            map.insert(left.clone(), value.clone());
        }

        if let Some(children) = &self.children {
            for (char, node) in children {
                node.build_map(
                    {
                        let mut left = left.clone();
                        left.push(*char);
                        left
                    },
                    map,
                );
            }
        }
    }
}

impl From<Node> for AHashMap<String, String> {
    fn from(node: Node) -> Self {
        let mut map = AHashMap::default();
        node.build_map(String::new(), &mut map);
        map
    }
}

#[derive(Default)]
pub struct Trie {
    root: Node,
    depth: usize,
}

impl Trie {
    pub fn get_depth(&self) -> usize {
        self.depth
    }

    pub fn insert(&mut self, key: &str, value: String) {
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

    pub fn get(&mut self, key: &str) -> Option<&str> {
        let mut node = &self.root;

        for char in key.chars() {
            if let Some(children) = &node.children {
                if let Some(child) = children.get(&char) {
                    node = child;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        node.value.as_ref().map(String::as_str)
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;

        for char in prefix.chars() {
            if let Some(children) = &node.children {
                if let Some(child) = children.get(&char) {
                    node = child;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    pub fn r#match(&self, s: &str) -> Option<(&str, usize)> {
        let mut node = &self.root;

        let mut result = None;

        for (char, i) in s.chars().take(self.depth).zip(1..) {
            if let Some(children) = &node.children {
                if let Some(child) = children.get(&char) {
                    node = child;

                    if let Some(value) = &node.value {
                        result = Some((value.as_str(), i));
                    }
                } else {
                    return result;
                }
            } else {
                return result;
            }
        }

        result
    }

    pub fn convert(&self, s: &str) -> String {
        let mut iter = s.chars().peekable();
        let mut dst = String::with_capacity(s.len());

        while iter.peek().is_some() {
            let s = iter.clone().take(self.depth).collect::<String>();

            if let Some((r, n)) = self.r#match(&s) {
                dst += r;
                iter.by_ref().nth(n - 1);
            } else {
                dst.push(iter.next().unwrap());
            }
        }

        dst
    }
}

impl From<Trie> for AHashMap<String, String> {
    fn from(trie: Trie) -> Self {
        trie.root.into()
    }
}

impl From<AHashMap<String, String>> for Trie {
    fn from(map: AHashMap<String, String>) -> Self {
        let mut trie = Trie::default();

        for (key, value) in map {
            trie.insert(key.as_str(), value);
        }

        trie
    }
}

impl FromIterator<(String, String)> for Trie {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut trie = Trie::default();

        for (key, value) in iter {
            trie.insert(key.as_str(), value);
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

        trie.insert("一分钟", String::from("一分鐘"));

        assert_eq!(trie.get("一分钟"), Some("一分鐘"));
        assert_eq!(trie.convert("一分钟"), "一分鐘");
    }
}
