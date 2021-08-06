use std::{borrow::Cow, collections::BTreeSet, fmt::Display};

use http::Method;

use super::endpoint::Endpoint;
use derive_builder::Builder;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegionDisplayOptions {
    Names,
}

impl RegionDisplayOptions {
    fn as_str(&self) -> &'static str {
        match self {
            RegionDisplayOptions::Names => "names",
        }
    }
}

impl Display for RegionDisplayOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct Regions {
    #[builder(setter(name = "_optional"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    optional: BTreeSet<RegionDisplayOptions>,
}

impl Regions {
    pub fn builder() -> RegionsBuilder {
        RegionsBuilder::default()
    }
}

impl RegionsBuilder {
    pub fn option(&mut self, option: RegionDisplayOptions) -> &mut Self {
        self.optional
            .get_or_insert_with(BTreeSet::new)
            .insert(option);
        self
    }

    pub fn options<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = RegionDisplayOptions>,
    {
        self.optional.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

impl Endpoint for Regions {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "v01/web/regions/".into()
    }

    fn query_parameters(&self) -> Result<std::borrow::Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StoresDisplayOptions {
    Deals,
    Catalog,
}

impl StoresDisplayOptions {
    fn as_str(&self) -> &'static str {
        match self {
            StoresDisplayOptions::Deals => "deals",
            StoresDisplayOptions::Catalog => "catalog",
        }
    }
}

impl Display for StoresDisplayOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct StoresInRegion<'a> {
    region: Cow<'a, str>,
    country: Option<Cow<'a, str>>,
    #[builder(setter(name = "_optional"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    optional: BTreeSet<StoresDisplayOptions>,
}

impl<'a> StoresInRegion<'a> {
    pub fn builder() -> StoresInRegionBuilder<'a> {
        StoresInRegionBuilder::default()
    }
}

impl<'a> StoresInRegionBuilder<'a> {
    pub fn option(&mut self, option: StoresDisplayOptions) -> &mut Self {
        self.optional
            .get_or_insert_with(BTreeSet::new)
            .insert(option);
        self
    }

    pub fn options<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = StoresDisplayOptions>,
    {
        self.optional.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

impl Endpoint for StoresInRegion<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v02/web/stores/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

#[derive(Default, Debug, Clone)]
pub struct CoveredStores {}

impl CoveredStores {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Endpoint for CoveredStores {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/web/stores/all/".into()
    }
}
