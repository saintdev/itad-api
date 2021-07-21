use std::{any, error::Error};

use thiserror::Error;

/// Errors that occur when creating form data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BodyError {
    /// Error serializing body data from form paramaters
    #[error("URL encode error: {}", source)]
    UrlEncoded {
        /// The source of the error
        #[from]
        source: serde_urlencoded::ser::Error,
    },
    #[error("JSON encode error: {}", source)]
    Json {
        #[from]
        source: serde_json::Error,
    },
}

/// Errors that occur from API endpoints.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Error creating body data
    #[error("failed to create form data: {}", source)]
    Body {
        /// The source of the error
        #[from]
        source: BodyError,
    },
    /// The client encountered an error.
    #[error("client error: {}", source)]
    Client {
        /// Client error
        source: E,
    },
    /// JSON deserialization failed
    #[error("failed to parse JSON: {}", source)]
    Json {
        /// The source of the error
        #[from]
        source: serde_json::Error,
    },
    /// The URL failed to parse.
    #[error("url parse error: {}", source)]
    Parse {
        /// The source of the error
        #[from]
        source: url::ParseError,
    },
    /// IsThereAnyDeal returned an error
    #[error("ITAD server error: {}", msg)]
    ItadApi {
        /// The error message from IsThereAnyDeal
        msg: String,
    },
    /// IsThereAnyDeal returned an unknown error
    #[error("Unknown ITAD server error: {:?}", obj)]
    Unknown {
        /// The JSON object returned from IsThereAnyDeal
        obj: serde_json::Value,
    },
    /// Failed parsing data type from JSON
    #[error("Parsing type {} from JSON: {}", typename, source)]
    DataType {
        /// The source of the error
        source: serde_json::Error,
        /// The name of the type that could not be deserialized.
        typename: &'static str,
    },
    #[error("authentication error: {}", source)]
    Authentication {
        #[from]
        source: crate::auth::AuthError,
    },
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Create an API error from a client error
    pub fn client(source: E) -> Self {
        Self::Client { source }
    }

    pub(crate) fn from_itad_api(val: serde_json::Value) -> Self {
        // let val = val.pointer("/message");
        if let Some(val) = val.pointer("/message") {
            if let Some(msg) = val.as_str() {
                Self::ItadApi { msg: msg.into() }
            } else {
                Self::Unknown { obj: val.clone() }
            }
        } else {
            Self::Unknown { obj: val.clone() }
        }
    }

    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        Self::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }
}
