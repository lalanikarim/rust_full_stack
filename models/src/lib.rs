use serde::{Deserialize, Serialize};
//use surrealdb::sql::Thing;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize, Hash)]
pub struct Thing {
    pub tb: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Person {
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub id: Option<Thing>,
    pub name: String,
}

impl Person {
    pub fn new(name: String) -> Self {
        Self {
            /*id: None,*/ name,
        }
    }
}
