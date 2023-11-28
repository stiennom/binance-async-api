use serde::{
    de::{
        Deserialize,
        Deserializer,
    },
    ser::{
        Serialize,
        Serializer,
    },
};
use serde_json::{
    self,
    Value,
};


pub fn treat_error_as_none<'a, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'a>,
    D: Deserializer<'a>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    Ok(T::deserialize(value).ok())
}

pub fn serialize_bool_as_str<S>(bool: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let serialized_str = serde_json::to_string(bool).unwrap();
    serializer.serialize_str(&serialized_str)
}

pub fn serialize_opt_bool_as_str<S>(opt_bool: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(bool) = opt_bool {
        serialize_bool_as_str(bool, serializer)
    } else {
        serializer.serialize_none()
    }
}

pub fn serialize_array_as_str<S, T>(array: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let serialized_str = serde_json::to_string(array).unwrap();
    serializer.serialize_str(&serialized_str)
}

pub fn serialize_opt_array_as_str<S, T>(opt_array: &Option<&[T]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    if let Some(array) = opt_array {
        serialize_array_as_str(array, serializer)
    } else {
        serializer.serialize_none()
    }
}
