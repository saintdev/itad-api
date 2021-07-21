use thiserror::Error;

use crate::{api, auth::AuthError};

pub type ItadApiResult<T> = Result<T, ItadApiError>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ItadApiError {
    #[error("api error: {}", source)]
    Api {
        #[from]
        source: api::ApiError<RestError>,
    },
    #[error("url parse error: {}", source)]
    Parse {
        #[from]
        source: url::ParseError,
    },
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    #[error("communication: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("http error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
}
