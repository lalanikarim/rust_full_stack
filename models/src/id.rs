use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Id {
    String(String),
}

impl Id {
    pub fn id_content(&self) -> String {
        match self {
            Id::String(strid) => strid.to_owned(),
        }
    }
}
impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id_content = self.id_content();
        write!(f, "{}", id_content)
    }
}

impl Into<String> for Id {
    fn into(self) -> String {
        match self {
            Id::String(strid) => strid,
        }
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
