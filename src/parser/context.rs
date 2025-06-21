use std::collections::HashMap;

use crate::parser::Value;

#[derive(Debug, Clone)]
pub struct RuntimeContext {
    global: Context,
    pub scopes: Vec<HashMap<String, Value>>,
}

impl<'a> RuntimeContext {
    pub fn new(global: Context) -> Self {
        Self {
            global,
            scopes: Vec::new(),
        }
    }

    pub fn resolve_path<I>(&self, path: I) -> Option<Value>
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut iter = path.into_iter();
        let first = iter.next()?;
        let mut current = self.get_from_scopes(first)?;

        for key in iter {
            current = match current {
                Value::Map(map) => map.get(key)?.clone(),
                _ => {
                    return None;
                }
            };
        }

        Some(current)
    }

    fn get_from_scopes(&self, key: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(key) {
                return Some(val.clone());
            }
        }
        None
    }

    /// Push a temporary scope (e.g., for loops or conditionals)
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pop a scope
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Insert a variable in the current scope
    pub fn insert(&mut self, key: impl Into<String>, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(key.into(), value);
        } else {
            // No temporary scope, modify global shadow layer
            self.scopes.push(HashMap::from([(key.into(), value)]));
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub data: HashMap<String, Value>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn merge_value(&mut self, incoming: Value) {
        match incoming {
            Value::Map(map) => {
                merge_nested(&mut self.data, &map);
            }
            _ => panic!(""),
        }
    }

    fn json_to_value_map(&self, json: serde_json::Value) -> Option<HashMap<String, Value>> {
        match Value::from(json) {
            Value::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn merge_json(&mut self, json: serde_json::Value) {
        if let Some(map) = self.json_to_value_map(json) {
            self.merge(&map);
        } else {
            panic!("Expected a JSON object to merge into context.");
        }
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
                Some(Value::Map(map)) => {
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
            (Some(Value::Map(target_obj)), Value::Map(source_obj)) => {
                merge_nested(target_obj, source_obj);
            }
            _ => {
                target.insert(key.clone(), value.clone());
            }
        }
    }
}
