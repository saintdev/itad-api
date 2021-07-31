use std::{borrow::Cow, collections::BTreeSet, fmt::Display};

use derive_builder::Builder;
use http::Method;
use serde::Serialize;

use super::endpoint::Endpoint;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IdentifierOptions {
    Title,
}

impl IdentifierOptions {
    fn as_str(&self) -> &'static str {
        match self {
            IdentifierOptions::Title => "title",
        }
    }
}

impl Display for IdentifierOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder(setter(into, strip_option), build_fn(validate = "Self::validate"))]
#[serde(rename_all = "snake_case")]
pub struct Identifier<'a> {
    shop: Option<Cow<'a, str>>,
    game_id: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    title: Option<Cow<'a, str>>,
    #[builder(setter(name = "_optional"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    optional: BTreeSet<IdentifierOptions>,
}

impl<'a> Identifier<'a> {
    pub fn builder() -> IdentifierBuilder<'a> {
        IdentifierBuilder::default()
    }
}

impl IdentifierBuilder<'_> {
    pub fn option(&mut self, option: IdentifierOptions) -> &mut Self {
        self.optional
            .get_or_insert_with(BTreeSet::new)
            .insert(option);
        self
    }

    pub fn options<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = IdentifierOptions>,
    {
        self.optional.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }

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
    // This can be sent as a JSON array in a POST body, maybe do that instead?
    #[builder(setter(name = "_ids"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    ids: BTreeSet<Cow<'a, str>>,
}

impl<'a> MultiplePlainsById<'a> {
    pub fn builder() -> MultiplePlainsByIdBuilder<'a> {
        MultiplePlainsByIdBuilder::default()
    }
}

impl<'a> MultiplePlainsByIdBuilder<'a> {
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
    #[builder(setter(name = "_plains"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    plains: BTreeSet<Cow<'a, str>>,
    region: Option<Cow<'a, str>>,
    country: Option<Cow<'a, str>>,
    #[builder(setter(name = "_shops"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    shops: BTreeSet<Cow<'a, str>>,
    #[builder(setter(name = "_exclude"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    exclude: BTreeSet<Cow<'a, str>>,
    added: Option<Cow<'a, str>>,
}

impl<'a> Prices<'a> {
    pub fn builder() -> PricesBuilder<'a> {
        PricesBuilder::default()
    }
}

impl<'a> PricesBuilder<'a> {
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

    pub fn shop<T>(&mut self, shop: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.shops
            .get_or_insert_with(BTreeSet::new)
            .insert(shop.into());
        self
    }

    pub fn shops<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.shops
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    pub fn exclude<T>(&mut self, exclude: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.exclude
            .get_or_insert_with(BTreeSet::new)
            .insert(exclude.into());
        self
    }

    pub fn excludes<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.exclude
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
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
    #[builder(setter(name = "_plains"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    plains: BTreeSet<Cow<'a, str>>,
    region: Option<Cow<'a, str>>,
    country: Option<Cow<'a, str>>,
    #[builder(setter(name = "_shops"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    shops: BTreeSet<Cow<'a, str>>,
    #[builder(setter(name = "_exclude"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    exclude: BTreeSet<Cow<'a, str>>,
    since: Option<u64>,
    until: Option<u64>,
    new: Option<bool>,
}

impl<'a> HistoricalLow<'a> {
    pub fn builder() -> HistoricalLowBuilder<'a> {
        HistoricalLowBuilder::default()
    }
}

impl<'a> HistoricalLowBuilder<'a> {
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

    pub fn shop<T>(&mut self, shop: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.shops
            .get_or_insert_with(BTreeSet::new)
            .insert(shop.into());
        self
    }

    pub fn shops<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.shops
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    pub fn exclude<T>(&mut self, exclude: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.exclude
            .get_or_insert_with(BTreeSet::new)
            .insert(exclude.into());
        self
    }

    pub fn excludes<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.exclude
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
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
    #[builder(setter(name = "_plains"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    plains: BTreeSet<Cow<'a, str>>,
    region: Option<Cow<'a, str>>,
    country: Option<Cow<'a, str>>,
    #[builder(setter(name = "_shops"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    shops: BTreeSet<Cow<'a, str>>,
    #[builder(setter(name = "_exclude"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    exclude: BTreeSet<Cow<'a, str>>,
}

impl<'a> StoreLow<'a> {
    pub fn builder() -> StoreLowBuilder<'a> {
        StoreLowBuilder::default()
    }
}

impl<'a> StoreLowBuilder<'a> {
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

    pub fn shop<T>(&mut self, shop: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.shops
            .get_or_insert_with(BTreeSet::new)
            .insert(shop.into());
        self
    }

    pub fn shops<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.shops
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    pub fn exclude<T>(&mut self, exclude: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.exclude
            .get_or_insert_with(BTreeSet::new)
            .insert(exclude.into());
        self
    }

    pub fn excludes<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.exclude
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
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
    #[builder(setter(name = "_plains"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    plains: BTreeSet<Cow<'a, str>>,
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

impl<'a> BundlesBuilder<'a> {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InfoOptions {
    Metacritic,
}

impl InfoOptions {
    fn as_str(&self) -> &'static str {
        match self {
            InfoOptions::Metacritic => "metacritic",
        }
    }
}

impl Display for InfoOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct Info<'a> {
    #[builder(setter(name = "_plains"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    plains: BTreeSet<Cow<'a, str>>,
    #[builder(setter(name = "_optional"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    optional: BTreeSet<InfoOptions>,
}

impl<'a> Info<'a> {
    pub fn builder() -> InfoBuilder<'a> {
        InfoBuilder::default()
    }
}

impl<'a> InfoBuilder<'a> {
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

    pub fn option(&mut self, option: InfoOptions) -> &mut Self {
        self.optional
            .get_or_insert_with(BTreeSet::new)
            .insert(option);
        self
    }

    pub fn options<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = InfoOptions>,
    {
        self.optional.get_or_insert_with(BTreeSet::new).extend(iter);
        self
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OverviewOptions {
    Voucher,
    Local,
}

impl OverviewOptions {
    fn as_str(&self) -> &'static str {
        match self {
            OverviewOptions::Voucher => "voucher",
            OverviewOptions::Local => "local",
        }
    }
}

impl Display for OverviewOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct Overview<'a> {
    region: Option<Cow<'a, str>>,
    country: Option<Cow<'a, str>>,
    #[builder(setter(name = "_plains"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    plains: BTreeSet<Cow<'a, str>>,
    shop: Option<Cow<'a, str>>,
    #[builder(setter(name = "_ids"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    ids: BTreeSet<Cow<'a, str>>,
    #[builder(setter(name = "_allowed"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    allowed: BTreeSet<Cow<'a, str>>,
    #[builder(setter(name = "_optional"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    optional: BTreeSet<OverviewOptions>,
}

impl<'a> Overview<'a> {
    pub fn builder() -> OverviewBuilder<'a> {
        OverviewBuilder::default()
    }
}

impl<'a> OverviewBuilder<'a> {
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

    pub fn allowed<T>(&mut self, allowed: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.allowed
            .get_or_insert_with(BTreeSet::new)
            .insert(allowed.into());
        self
    }

    pub fn alloweds<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.allowed
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    pub fn option(&mut self, option: OverviewOptions) -> &mut Self {
        self.optional
            .get_or_insert_with(BTreeSet::new)
            .insert(option);
        self
    }

    pub fn options<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = OverviewOptions>,
    {
        self.optional.get_or_insert_with(BTreeSet::new).extend(iter);
        self
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
