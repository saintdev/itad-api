use crate::api::{endpoint::Endpoint, error::BodyError};
use derive_builder::Builder;
use http::Method;
use serde::Serialize;
use std::{borrow::Cow, collections::BTreeSet, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CollectionCheckOptions {
    Stores,
}

impl CollectionCheckOptions {
    fn as_str(&self) -> &'static str {
        match self {
            CollectionCheckOptions::Stores => "stores",
        }
    }
}

impl Display for CollectionCheckOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct CollectionCheck<'a> {
    plain: Cow<'a, str>,
    #[builder(setter(name = "_optional"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    optional: BTreeSet<CollectionCheckOptions>,
}

impl<'a> CollectionCheck<'a> {
    pub fn builder() -> CollectionCheckBuilder<'a> {
        CollectionCheckBuilder::default()
    }
}

impl<'a> CollectionCheckBuilder<'a> {
    pub fn option(&mut self, option: CollectionCheckOptions) -> &mut Self {
        self.optional
            .get_or_insert_with(BTreeSet::new)
            .insert(option);
        self
    }

    pub fn options<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = CollectionCheckOptions>,
    {
        self.optional.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

impl Endpoint for CollectionCheck<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/user/coll/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_oauth_token(&self) -> bool {
        // coll_read
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CollectionOptions {
    Plain,
    Title,
    Gameid,
    CopyType,
}

impl CollectionOptions {
    fn as_str(&self) -> &'static str {
        match self {
            CollectionOptions::Plain => "plain",
            CollectionOptions::Title => "title",
            CollectionOptions::Gameid => "gameid",
            CollectionOptions::CopyType => "copy_type",
        }
    }
}

impl Display for CollectionOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder(setter(into, strip_option), build_fn(validate = "Self::validate"))]
#[serde(rename_all = "snake_case")]
pub struct Collection<'a> {
    shop: Option<Cow<'a, str>>,
    short: Option<bool>,
    #[builder(setter(name = "_optional"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    optional: BTreeSet<CollectionOptions>,
}

impl<'a> Collection<'a> {
    pub fn builder() -> CollectionBuilder<'a> {
        CollectionBuilder::default()
    }
}

impl CollectionBuilder<'_> {
    pub fn option(&mut self, option: CollectionOptions) -> &mut Self {
        self.optional
            .get_or_insert_with(BTreeSet::new)
            .insert(option);
        self
    }

    pub fn options<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = CollectionOptions>,
    {
        self.optional.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(optional) = &self.optional {
            if !optional.contains(&CollectionOptions::Plain)
                && !optional.contains(&CollectionOptions::Title)
                && !optional.contains(&CollectionOptions::Gameid)
            {
                return Err("You must specify one of Plain, Title or Gameid".into());
            }

            if optional.contains(&CollectionOptions::Gameid) && self.shop.is_none() {
                return Err("If you set Gameid, you must specify a shop ID".into());
            }
        }

        Ok(())
    }
}

impl Endpoint for Collection<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v02/user/coll/all/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_oauth_token(&self) -> bool {
        // coll_read
        true
    }
}

// TODO: Make body generic for T: Serialize?
#[derive(Debug, Clone, Builder)]
#[builder(setter(into, strip_option))]
pub struct ImportCollectionViaForm<'a> {
    #[doc = r"JSON encoded waitlist to import. See: https://itad.docs.apiary.io/#reference/collection/import-via-form/send-user-to-collection-import-form"]
    body: Cow<'a, str>,
}

impl<'a> ImportCollectionViaForm<'a> {
    pub fn builder() -> ImportCollectionViaFormBuilder<'a> {
        ImportCollectionViaFormBuilder::default()
    }
}

impl Endpoint for ImportCollectionViaForm<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "collection/import/".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
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
pub struct ImportCollection<'a> {
    file: Cow<'a, str>,
    #[builder(default, setter(skip))]
    #[serde(rename = "upload")]
    _upload: (),
}

impl<'a> ImportCollection<'a> {
    pub fn builder() -> ImportCollectionBuilder<'a> {
        ImportCollectionBuilder::default()
    }
}

impl Endpoint for ImportCollection<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/collection/import/".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, super::error::BodyError> {
        Ok(Some(("application/json", self.file.as_bytes().to_owned())))
    }

    fn requires_oauth_token(&self) -> bool {
        // coll_write, wait_write
        true
    }
}
