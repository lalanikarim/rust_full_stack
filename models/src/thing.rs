use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::Id;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Thing {
    pub tb: String,
    pub id: Id,
}

impl Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.tb, self.id)
    }
}

impl Into<String> for Thing {
    fn into(self) -> String {
        format!("{}:{}", self.tb, self.id)
    }
}

impl From<String> for Thing {
    fn from(value: String) -> Self {
        let pos = value.find(":").expect("Invalid Thing provided");
        Thing {
            tb: String::from(&value[0..pos]),
            id: Id::String(String::from(&value[pos + 1..])),
        }
    }
}
