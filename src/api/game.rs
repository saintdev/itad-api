use std::{borrow::Cow, collections::HashSet};

use derive_builder::Builder;
use http::Method;
use serde::{de::IntoDeserializer, Serialize};

use super::endpoint::Endpoint;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentifierOptions {
    Title,
}

#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder(setter(into, strip_option), build_fn(validate = "Self::validate"))]
#[serde(rename_all = "snake_case")]
pub struct Identifier<'a> {
    shop: Option<Cow<'a, str>>,
    game_id: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    title: Option<Cow<'a, str>>,
    #[serde(serialize_with = "super::utils::serialize_hash_set_urlencoded")]
    optional: Option<HashSet<IdentifierOptions>>,
}

impl<'a> Identifier<'a> {
    pub fn builder() -> IdentifierBuilder<'a> {
        IdentifierBuilder::default()
    }
}

impl IdentifierBuilder<'_> {
    fn validate(&self) -> Result<(), String> {
        if self.game_id.is_some() && self.shop.is_none() {
            return Err("Shop is required to be set when looking up by ID".into());
        }

        if self.url.is_some() && self.shop.is_none() {
            return Err("Shop is required to be set when looking up by URL".into());
        }

        Ok(())
    }
}

impl Endpoint for Identifier<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v02/game/plain/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct MultiplePlainsById<'a> {
    shop: Cow<'a, str>,
    // This can be sent as a JSON array in a POST body, maybe do that?
    #[serde(serialize_with = "super::utils::serialize_vec_urlencoded")]
    ids: Option<Vec<Cow<'a, str>>>,
}

impl<'a> MultiplePlainsById<'a> {
    pub fn builder() -> MultiplePlainsByIdBuilder<'a> {
        MultiplePlainsByIdBuilder::default()
    }
}

impl Endpoint for MultiplePlainsById<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/game/plain/id/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct AllPlains<'a> {
    shop: Cow<'a, str>,
}

impl<'a> AllPlains<'a> {
    pub fn builder() -> AllPlainsBuilder<'a> {
        AllPlainsBuilder::default()
    }
}

impl Endpoint for AllPlains<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/game/plain/list/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MapType {
    #[serde(rename = "plain:id")]
    PlainToId,
    #[serde(rename = "id:plain")]
    IdToPlain,
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct IdPlainMap<'a> {
    shop: Cow<'a, str>,
    #[serde(rename = "type")]
    type_field: Option<MapType>,
}

impl<'a> IdPlainMap<'a> {
    pub fn builder() -> IdPlainMapBuilder<'a> {
        IdPlainMapBuilder::default()
    }
}

impl Endpoint for IdPlainMap<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/game/map/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct Prices<'a> {
    #[serde(serialize_with = "super::utils::serialize_iter_urlencoded")]
    plains: Vec<Cow<'a, str>>,
    region: Option<Cow<'a, str>>,
    country: Option<Cow<'a, str>>,
    #[serde(serialize_with = "super::utils::serialize_vec_urlencoded")]
    shops: Option<Vec<Cow<'a, str>>>,
    #[serde(serialize_with = "super::utils::serialize_vec_urlencoded")]
    exclude: Option<Vec<Cow<'a, str>>>,
    added: Option<Cow<'a, str>>,
}

impl<'a> Prices<'a> {
    pub fn builder() -> PricesBuilder<'a> {
        PricesBuilder::default()
    }
}

impl Endpoint for Prices<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/game/prices/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct HistoricalLow<'a> {
    #[serde(serialize_with = "super::utils::serialize_iter_urlencoded")]
    plains: Vec<Cow<'a, str>>,
    region: Option<Cow<'a, str>>,
    country: Option<Cow<'a, str>>,
    #[serde(serialize_with = "super::utils::serialize_vec_urlencoded")]
    shops: Option<Vec<Cow<'a, str>>>,
    #[serde(serialize_with = "super::utils::serialize_vec_urlencoded")]
    exclude: Option<Vec<Cow<'a, str>>>,
    since: Option<u64>,
    until: Option<u64>,
    new: Option<bool>,
}

impl<'a> HistoricalLow<'a> {
    pub fn builder() -> HistoricalLowBuilder<'a> {
        HistoricalLowBuilder::default()
    }
}

impl Endpoint for HistoricalLow<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/game/lowest/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct StoreLow<'a> {
    #[serde(serialize_with = "super::utils::serialize_iter_urlencoded")]
    plains: Vec<Cow<'a, str>>,
    region: Option<Cow<'a, str>>,
    country: Option<Cow<'a, str>>,
    #[serde(serialize_with = "super::utils::serialize_vec_urlencoded")]
    shops: Option<Vec<Cow<'a, str>>>,
    #[serde(serialize_with = "super::utils::serialize_vec_urlencoded")]
    exclude: Option<Vec<Cow<'a, str>>>,
}

impl<'a> StoreLow<'a> {
    pub fn builder() -> StoreLowBuilder<'a> {
        StoreLowBuilder::default()
    }
}

impl Endpoint for StoreLow<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/game/storelow/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BundlesSorting {
    Expiry,
    Recent,
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct Bundles<'a> {
    #[serde(serialize_with = "super::utils::serialize_iter_urlencoded")]
    plains: Vec<Cow<'a, str>>,
    limit: Option<i64>,
    expired: Option<bool>,
    sort: Option<BundlesSorting>,
    region: Option<Cow<'a, str>>,
}

impl<'a> Bundles<'a> {
    pub fn builder() -> BundlesBuilder<'a> {
        BundlesBuilder::default()
    }
}

impl Endpoint for Bundles<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/game/bundles/".into()
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InfoOptions {
    Metacritic,
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct Info<'a> {
    #[serde(serialize_with = "super::utils::serialize_iter_urlencoded")]
    plains: Vec<Cow<'a, str>>,
    #[serde(serialize_with = "super::utils::serialize_hash_set_urlencoded")]
    optional: Option<HashSet<InfoOptions>>,
}

impl<'a> Info<'a> {
    pub fn builder() -> InfoBuilder<'a> {
        InfoBuilder::default()
    }
}

impl Endpoint for Info<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/game/info/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OverviewOptions {
    Voucher,
    Local,
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct Overview<'a> {
    region: Option<Cow<'a, str>>,
    country: Option<Cow<'a, str>>,
    #[serde(serialize_with = "super::utils::serialize_vec_urlencoded")]
    plains: Option<Vec<Cow<'a, str>>>,
    shop: Option<Cow<'a, str>>,
    #[serde(serialize_with = "super::utils::serialize_vec_urlencoded")]
    ids: Option<Vec<Cow<'a, str>>>,
    #[serde(serialize_with = "super::utils::serialize_vec_urlencoded")]
    allowed: Option<Vec<Cow<'a, str>>>,
    #[serde(serialize_with = "super::utils::serialize_hash_set_urlencoded")]
    optional: Option<HashSet<OverviewOptions>>,
}

impl<'a> Overview<'a> {
    pub fn builder() -> OverviewBuilder<'a> {
        OverviewBuilder::default()
    }
}

impl Endpoint for Overview<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/game/overview/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}
