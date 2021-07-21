use std::fmt::Debug;

use thiserror::Error;
use url::{form_urlencoded::Serializer, UrlQuery};

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    #[error("Missing API key")]
    MissingApiKey,
    #[error("Missing Oauth token")]
    MissingOauthToken,
}

#[derive(Clone)]
pub(crate) struct Auth {
    pub(crate) api_key: Option<String>,
    pub(crate) oauth_token: Option<String>,
}

impl Auth {
    pub(crate) fn append_api_key_query_param(
        &self,
        query_params: &mut Serializer<'_, UrlQuery<'_>>,
    ) -> Result<(), AuthError> {
        self.api_key
            .as_ref()
            .map(|key| {
                query_params.append_pair("key", key);
            })
            .ok_or(AuthError::MissingApiKey)
    }

    pub(crate) fn append_oauth_token_query_param(
        &self,
        query_params: &mut Serializer<'_, UrlQuery<'_>>,
    ) -> Result<(), AuthError> {
        self.oauth_token
            .as_ref()
            .map(|token| {
                query_params.append_pair("access_token", token);
            })
            .ok_or(AuthError::MissingOauthToken)
    }
}

impl Debug for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Auth")
            .field("api_key", &self.api_key.is_some())
            .field("oauth_token", &self.oauth_token.is_some())
            .finish()
    }
}
