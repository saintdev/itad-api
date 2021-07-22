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
        let out: Vec<_> = value
            .iter()
            .map(|value| serde_urlencoded::to_string(value).expect("Unable to serialize value"))
            .collect();
        serializer.serialize_str(&out.join(","))
    } else {
        serializer.serialize_none()
    }
}
