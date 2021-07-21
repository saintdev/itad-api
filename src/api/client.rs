use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;
use http::{request::Builder as RequestBuilder, Response};
use url::{form_urlencoded::Serializer, Url, UrlQuery};

use super::error::ApiError;

/// A trait representing a client which can communicate with IsThereAnyDeal via
/// REST
pub trait RestClient {
    /// The error that may occur for this client
    type Error: Error + Send + Sync + 'static;

    /// Get the URL for the endpoint for the client.
    ///
    /// This method adds the hostname for the target api.
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;

    fn append_api_key_query_param(
        &self,
        query_params: &mut Serializer<'_, UrlQuery<'_>>,
    ) -> Result<(), ApiError<Self::Error>>;

    fn append_oauth_token_query_param(
        &self,
        query_params: &mut Serializer<'_, UrlQuery<'_>>,
    ) -> Result<(), ApiError<Self::Error>>;
}

/// A trait representing a client which can communicate with IsThereAnyDeal
pub trait Client: RestClient {
    /// Send a REST query
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

/// A trait representing an asynchronous client which can communicate with
/// IsThereAnyDeal
#[async_trait]
pub trait AsyncClient: RestClient {
    /// Send a REST query asynchronously
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}
