use serde::Deserialize;

#[derive(Default, Debug, Clone, Deserialize, PartialEq)]
pub(crate) struct Root<T> {
    pub(crate) data: T,
}
