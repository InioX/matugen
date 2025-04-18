use std::collections::HashMap;

use crate::engine::Value;

#[derive(Debug, Clone)]
pub struct Context {
    pub data: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Deeply merge Object values into the context
    pub fn merge(&mut self, incoming: &HashMap<String, Value>) {
        merge_nested(&mut self.data, incoming);
    }

    /// Remove a nested key from the context by path like ["theme", "primary"]
    pub fn remove_path<'a, I>(&mut self, path: I) -> bool
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut iter = path.into_iter();
        let Some(first) = iter.next() else {
            return false;
        };

        let mut current = &mut self.data;
        let mut last_key = first;

        for key in iter {
            match current.get_mut(last_key) {
                Some(Value::Object(map)) => {
                    current = map;
                    last_key = key;
                }
                _ => return false,
            }
        }

        current.remove(last_key).is_some()
    }

    pub fn data(&self) -> &HashMap<String, Value> {
        &self.data
    }
}

/// Deep merge two `HashMap<String, Value>`, respecting nested `Object`s
fn merge_nested(target: &mut HashMap<String, Value>, source: &HashMap<String, Value>) {
    for (key, value) in source {
        match (target.get_mut(key), value) {
            (Some(Value::Object(target_obj)), Value::Object(source_obj)) => {
                merge_nested(target_obj, source_obj);
            }
            _ => {
                target.insert(key.clone(), value.clone());
            }
        }
    }
}
