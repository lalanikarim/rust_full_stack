use std::fmt::Display;

use serde::{
    de::{Error, Visitor},
    Deserialize, Serialize,
};

use crate::Id;

#[derive(PartialEq, Debug, Clone)]
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

impl Serialize for Thing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string()[..])
    }
}

impl<'de> Deserialize<'de> for Thing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Tb,
            Id,
        }

        struct FieldVisitor;
        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Thing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Thing")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Thing::from(v.to_string()))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut tb = None;
                let mut id = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Tb => {
                            if tb.is_some() {
                                return Err(Error::duplicate_field("tb"));
                            }
                            tb = Some(map.next_value()?);
                        }
                        Field::Id => {
                            if id.is_some() {
                                return Err(Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                    }
                }
                if tb.is_none() {
                    Err(Error::missing_field("tb"))
                } else if id.is_none() {
                    Err(Error::missing_field("id"))
                } else {
                    let tb = tb.unwrap();
                    let id = id.unwrap();
                    Ok(Thing { tb, id })
                }
            }
        }

        deserializer.deserialize_any(FieldVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Id, Thing};

    #[test]
    fn thing_to_string() {
        let id: Thing = Thing {
            tb: "table".to_string(),
            id: Id::String("strid".to_string()),
        };
        assert_eq!(id.to_string(), "table:strid".to_string());
    }

    #[test]
    fn string_to_thing() {
        let id: Thing = Thing {
            tb: "table".to_string(),
            id: Id::String("strid".to_string()),
        };
        let id2 = Thing::from("table:strid".to_string());
        assert_eq!(id, id2);
    }
}
