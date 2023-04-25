use serde::{Deserialize, Serialize};

use crate::Thing;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Person {
    pub id: Option<Thing>,
    pub name: String,
}

impl Person {
    pub fn new(name: String) -> Self {
        Self { id: None, name }
    }
}
