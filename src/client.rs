use std::convert::TryInto;

use async_trait::async_trait;
use futures::TryFutureExt;
use log::debug;
use reqwest::{blocking::Client as HttpClient, Client as AsyncHttpClient};
use url::Url;

use crate::{
    api,
    auth::Auth,
    error::{ItadApiResult, RestError},
};

const DEFAULT_ITAD_API_HOST: &str = "api.isthereanydeal.com";

#[derive(Clone, Debug)]
pub struct ItadApiClient {
    client: HttpClient,
    rest_url: Url,
    auth: Auth,
}

impl ItadApiClient {
    pub fn new() -> ItadApiResult<Self> {
        Self::new_impl::<&str, &str>(None, None, None)
    }

    pub fn with_api_key<S>(api_key: S) -> ItadApiResult<Self>
    where
        S: Into<String>,
    {
        Self::new_impl(None, Some(api_key), None::<&str>)
    }

    pub fn with_oauth_token<S>(oauth_token: S) -> ItadApiResult<Self>
    where
        S: Into<String>,
    {
        Self::new_impl(None, None::<&str>, Some(oauth_token))
    }

    fn new_impl<K, T>(
        host: Option<&str>,
        api_key: Option<K>,
        oauth_token: Option<T>,
    ) -> ItadApiResult<Self>
    where
        K: Into<String>,
        T: Into<String>,
    {
        let rest_url = Url::parse(&format!(
            "https://{}/",
            host.unwrap_or(DEFAULT_ITAD_API_HOST),
        ))?;
        let auth = Auth {
            api_key: api_key.map(Into::into),
            oauth_token: oauth_token.map(Into::into),
        };

        Ok(ItadApiClient {
            client: HttpClient::new(),
            rest_url,
            auth,
        })
    }

    pub fn builder() -> ItadApiBuilder {
        ItadApiBuilder::new()
    }
}

impl api::RestClient for ItadApiClient {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!("REST api call {}", endpoint);
        self.rest_url
            .join(endpoint.trim_start_matches('/'))
            .map_err(From::from)
    }

    fn append_api_key_query_param(
        &self,
        query_params: &mut url::form_urlencoded::Serializer<'_, url::UrlQuery<'_>>,
    ) -> Result<(), api::ApiError<Self::Error>> {
        Ok(self.auth.append_api_key_query_param(query_params)?)
    }

    fn append_oauth_token_query_param(
        &self,
        query_params: &mut url::form_urlencoded::Serializer<'_, url::UrlQuery<'_>>,
    ) -> Result<(), api::ApiError<Self::Error>> {
        Ok(self.auth.append_oauth_token_query_param(query_params)?)
    }
}

impl api::Client for ItadApiClient {
    fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, api::ApiError<Self::Error>> {
        let call = || -> Result<_, RestError> {
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request)?;

            let mut http_rsp = http::Response::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, val) in rsp.headers() {
                headers.insert(key, val.clone());
            }
            http_rsp.body(rsp.bytes()?).map_err(From::from)
        };
        call().map_err(api::ApiError::client)
    }
}

#[derive(Clone, Debug)]
pub struct ItadApiClientAsync {
    client: AsyncHttpClient,
    rest_url: Url,
    auth: Auth,
}

impl ItadApiClientAsync {
    pub fn new() -> ItadApiResult<Self> {
        Self::new_impl::<&str, &str>(None, None, None)
    }

    pub fn with_api_key<S>(api_key: S) -> ItadApiResult<Self>
    where
        S: Into<String>,
    {
        Self::new_impl(None, Some(api_key), None::<&str>)
    }

    pub fn with_oauth_token<S>(oauth_token: S) -> ItadApiResult<Self>
    where
        S: Into<String>,
    {
        Self::new_impl(None, None::<&str>, Some(oauth_token))
    }

    fn new_impl<K, T>(
        host: Option<&str>,
        api_key: Option<K>,
        oauth_token: Option<T>,
    ) -> ItadApiResult<Self>
    where
        K: Into<String>,
        T: Into<String>,
    {
        let rest_url = Url::parse(&format!(
            "https://{}/",
            host.unwrap_or(DEFAULT_ITAD_API_HOST)
        ))?;
        let client = AsyncHttpClient::new();
        let auth = Auth {
            api_key: api_key.map(Into::into),
            oauth_token: oauth_token.map(Into::into),
        };
        let api = Self {
            client,
            rest_url,
            auth,
        };
        Ok(api)
    }
}

impl api::RestClient for ItadApiClientAsync {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!("REST api call {}", endpoint);
        self.rest_url
            .join(endpoint.trim_start_matches('/'))
            .map_err(From::from)
    }

    fn append_api_key_query_param(
        &self,
        query_params: &mut url::form_urlencoded::Serializer<'_, url::UrlQuery<'_>>,
    ) -> Result<(), api::ApiError<Self::Error>> {
        Ok(self.auth.append_api_key_query_param(query_params)?)
    }

    fn append_oauth_token_query_param(
        &self,
        query_params: &mut url::form_urlencoded::Serializer<'_, url::UrlQuery<'_>>,
    ) -> Result<(), api::ApiError<Self::Error>> {
        Ok(self.auth.append_oauth_token_query_param(query_params)?)
    }
}

#[async_trait]
impl api::AsyncClient for ItadApiClientAsync {
    async fn rest_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, api::ApiError<Self::Error>> {
        let call = || async {
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request).await?;

            let mut http_rsp = http::Response::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, val) in rsp.headers() {
                headers.insert(key, val.clone());
            }
            http_rsp.body(rsp.bytes().await?).map_err(From::from)
        };
        call().map_err(api::ApiError::client).await
    }
}

#[derive(Debug, Default)]
pub struct ItadApiBuilder {
    host: Option<String>,
    api_key: Option<String>,
    oauth_token: Option<String>,
}

impl ItadApiBuilder {
    pub fn new() -> Self {
        ItadApiBuilder::default()
    }

    pub fn host<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.host = Some(value.into());
        self
    }

    pub fn api_key<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.api_key = Some(value.into());
        self
    }

    pub fn oauth_token<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.oauth_token = Some(value.into());
        self
    }

    pub fn build(&self) -> ItadApiResult<ItadApiClient> {
        ItadApiClient::new_impl(
            self.host.as_deref(),
            self.api_key.as_ref(),
            self.oauth_token.as_ref(),
        )
    }

    pub fn build_async(&self) -> ItadApiResult<ItadApiClientAsync> {
        ItadApiClientAsync::new_impl(
            self.host.as_deref(),
            self.api_key.as_ref(),
            self.oauth_token.as_ref(),
        )
    }
}
