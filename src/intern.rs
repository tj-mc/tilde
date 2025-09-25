use std::collections::HashMap;

/// String interning system to reduce memory allocations and speed up comparisons
/// Variables like "counter", "result", "n" get reused frequently in recursive functions
#[derive(Debug)]
pub struct StringInterner {
    strings: Vec<String>,
    indices: HashMap<String, usize>,
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}

impl StringInterner {
    pub fn new() -> Self {
        StringInterner {
            strings: Vec::new(),
            indices: HashMap::new(),
        }
    }

    /// Get or create an interned string, returning its index
    pub fn intern(&mut self, s: &str) -> usize {
        if let Some(&index) = self.indices.get(s) {
            index
        } else {
            let index = self.strings.len();
            let owned = s.to_string();
            self.indices.insert(owned.clone(), index);
            self.strings.push(owned);
            index
        }
    }

    /// Get the string for a given index
    pub fn get(&self, index: usize) -> Option<&str> {
        self.strings.get(index).map(|s| s.as_str())
    }

    /// Get the index for a string if it exists
    pub fn get_index(&self, s: &str) -> Option<usize> {
        self.indices.get(s).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_interning() {
        let mut interner = StringInterner::new();

        let index1 = interner.intern("hello");
        let index2 = interner.intern("world");
        let index3 = interner.intern("hello"); // Should reuse

        assert_eq!(index1, index3); // Same string, same index
        assert_ne!(index1, index2); // Different strings, different indices

        assert_eq!(interner.get(index1), Some("hello"));
        assert_eq!(interner.get(index2), Some("world"));
    }

    #[test]
    fn test_get_index() {
        let mut interner = StringInterner::new();

        let index = interner.intern("test");
        assert_eq!(interner.get_index("test"), Some(index));
        assert_eq!(interner.get_index("nonexistent"), None);
    }
}
