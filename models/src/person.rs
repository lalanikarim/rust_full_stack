use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Person {
    pub id: Option<Value>,
    pub name: String,
}

impl Person {
    pub fn new(name: String) -> Self {
        Self { id: None, name }
    }
}
