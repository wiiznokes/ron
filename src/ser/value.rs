use serde::ser::{Serialize, Serializer};

use crate::value::Value;

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Char(c) => serializer.serialize_char(*c),
            Value::Map(m) => Serialize::serialize(m, serializer),
            Value::Number(number) => Serialize::serialize(number, serializer),
            Value::Option(Some(o)) => serializer.serialize_some(o.as_ref()),
            Value::Option(None) => serializer.serialize_none(),
            Value::String(s) => serializer.serialize_str(s),
            Value::Bytes(b) => serializer.serialize_bytes(b),
            Value::List(s) => Serialize::serialize(s, serializer),
            Value::Unit => serializer.serialize_unit(),
            Value::Tuple(_values) => todo!(),
            Value::NamedUnit(_cow) => todo!(),
            Value::NamedMap(_cow, _map) => todo!(),
            Value::NamedTuple(_cow, _values) => todo!(),
        }
    }
}
