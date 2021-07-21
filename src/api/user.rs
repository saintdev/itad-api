use http::Method;

use super::endpoint::Endpoint;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserInfo {}

impl UserInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Endpoint for UserInfo {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "v01/user/info".into()
    }

    fn requires_oauth_token(&self) -> bool {
        true
    }
}
