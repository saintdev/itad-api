use std::collections::HashSet;

use serde::{Serialize, Serializer};

pub(crate) fn serialize_hash_set_urlencoded<S, T>(
    value: &Option<HashSet<T>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    if let Some(value) = value {
        serialize_iter_urlencoded(value, serializer)
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn serialize_vec_urlencoded<S, T>(
    value: &Option<Vec<T>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    if let Some(value) = value {
        serialize_iter_urlencoded(value, serializer)
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn serialize_iter_urlencoded<'a, I, S>(iter: I, serializer: S) -> Result<S::Ok, S::Error>
where
    I: IntoIterator,
    I::Item: Serialize + 'a,
    S: Serializer,
{
    let out: Vec<_> = iter
        .into_iter()
        .map(|value| serde_urlencoded::to_string(value).expect("Unable to serialize value"))
        .collect();
    serializer.serialize_str(&out.join("%2C"))
}
