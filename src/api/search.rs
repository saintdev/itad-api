use std::borrow::Cow;

use derive_builder::Builder;
use http::Method;
use serde::Serialize;

use super::endpoint::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct FindGames<'a> {
    q: Cow<'a, str>,
    limit: Option<usize>,
    #[serde(serialize_with = "super::utils::serialize_option_bool_as_int")]
    strict: Option<bool>,
}

impl<'a> FindGames<'a> {
    pub fn builder() -> FindGamesBuilder<'a> {
        FindGamesBuilder::default()
    }
}

impl Endpoint for FindGames<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v02/search/search/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}
