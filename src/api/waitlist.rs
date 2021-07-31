use std::{borrow::Cow, collections::BTreeSet};

use derive_builder::Builder;
use http::Method;
use serde::Serialize;

use super::endpoint::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct WaitlistCheck<'a> {
    plain: Cow<'a, str>,
}

impl<'a> WaitlistCheck<'a> {
    pub fn builder() -> WaitlistCheckBuilder<'a> {
        WaitlistCheckBuilder::default()
    }
}

impl Endpoint for WaitlistCheck<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/user/wait/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_oauth_token(&self) -> bool {
        // wait_read
        true
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WaitlistOptions {
    Title,
    Gameid,
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct Waitlist<'a> {
    shop: Option<Cow<'a, str>>,
    optional: Option<WaitlistOptions>,
}

impl<'a> Waitlist<'a> {
    pub fn builder() -> WaitlistBuilder<'a> {
        WaitlistBuilder::default()
    }
}

impl Endpoint for Waitlist<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/user/wait/all/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_oauth_token(&self) -> bool {
        // wait_read
        true
    }
}

// TODO: Make body generic for T: Serialize?
#[derive(Debug, Clone, Builder)]
#[builder(setter(into, strip_option))]
pub struct ImportWaitlistViaForm<'a> {
    #[doc = r"JSON encoded waitlist to import. See: https://itad.docs.apiary.io/#reference/waitlist/import-via-form/send-user-to-waitlist-import-form"]
    body: Cow<'a, str>,
}

impl<'a> ImportWaitlistViaForm<'a> {
    pub fn builder() -> ImportWaitlistViaFormBuilder<'a> {
        ImportWaitlistViaFormBuilder::default()
    }
}

impl Endpoint for ImportWaitlistViaForm<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "waitlist/import/".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, super::error::BodyError> {
        #[derive(Debug, Serialize)]
        struct Body<'a> {
            file: Cow<'a, str>,
            #[serde(rename = "upload")]
            _upload: (),
        }
        let file = base64::encode(self.body.as_ref());
        let body = Body {
            file: file.into(),
            _upload: (),
        };
        Ok(Some((
            "application/x-www-form-urlencoded",
            serde_urlencoded::to_string(&body)?.into_bytes(),
        )))
    }
}

// TODO: Make file generic for T: Serialize?
#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct ImportWaitlist<'a> {
    file: Cow<'a, str>,
    #[builder(default, setter(skip))]
    #[serde(rename = "upload")]
    _upload: (),
}

impl<'a> ImportWaitlist<'a> {
    pub fn builder() -> ImportWaitlistBuilder<'a> {
        ImportWaitlistBuilder::default()
    }
}

impl Endpoint for ImportWaitlist<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/waitlist/import/".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, super::error::BodyError> {
        Ok(Some(("application/json", self.file.as_bytes().to_owned())))
    }

    fn requires_oauth_token(&self) -> bool {
        // wait_write
        true
    }
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct WaitlistRemove<'a> {
    #[builder(setter(name = "_plains"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    plains: BTreeSet<Cow<'a, str>>,
    shop: Option<Cow<'a, str>>,
    #[builder(setter(name = "_ids"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    ids: BTreeSet<Cow<'a, str>>,
}

impl<'a> WaitlistRemove<'a> {
    pub fn builder() -> WaitlistRemoveBuilder<'a> {
        WaitlistRemoveBuilder::default()
    }
}

impl<'a> WaitlistRemoveBuilder<'a> {
    pub fn plain<T>(&mut self, plain: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.plains
            .get_or_insert_with(BTreeSet::new)
            .insert(plain.into());
        self
    }

    pub fn plains<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.plains
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    pub fn id<T>(&mut self, id: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.ids.get_or_insert_with(BTreeSet::new).insert(id.into());
        self
    }

    pub fn ids<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.ids
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }
}

impl Endpoint for WaitlistRemove<'_> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v02/user/wait/remove/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_oauth_token(&self) -> bool {
        // wait_write
        true
    }
}
