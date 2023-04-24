pub mod person;
pub use person::Person;

use serde_json::Value;
pub fn id_from_thing(id: Option<Value>) -> Option<String> {
    if let Some(id) = id {
        if let Some(id) = id.get("id") {
            if let Some(id) = id.get("String") {
                id.as_str().map(String::from)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
