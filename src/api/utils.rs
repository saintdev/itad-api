use serde::Serializer;

pub(crate) fn serialize_as_csv<S, T>(
    iter: impl IntoIterator<Item = T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: Serializer,
{
    let out: Vec<_> = iter.into_iter().map(|v| v.to_string()).collect();
    serializer.serialize_str(&out.join(","))
}

pub(crate) fn serialize_option_bool_as_int<S>(
    value: &Option<bool>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(value) = value {
        if *value {
            serializer.serialize_u64(1)
        } else {
            serializer.serialize_u64(0)
        }
    } else {
        serializer.serialize_none()
    }
}
