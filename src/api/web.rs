use std::{borrow::Cow, collections::HashSet};

use http::Method;

use super::endpoint::Endpoint;
use derive_builder::Builder;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RegionDisplayOptions {
    Names,
}

#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Regions {
    #[serde(serialize_with = "super::utils::serialize_hash_set_urlencoded")]
    optional: Option<HashSet<RegionDisplayOptions>>,
}

impl Regions {
    pub fn builder() -> RegionsBuilder {
        RegionsBuilder::default()
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum StoresDisplayOptions {
    Deals,
    Catalog,
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct StoresInRegion<'a> {
    region: Cow<'a, str>,
    country: Option<Cow<'a, str>>,
    #[serde(serialize_with = "super::utils::serialize_hash_set_urlencoded")]
    optional: Option<HashSet<StoresDisplayOptions>>,
}

impl<'a> StoresInRegion<'a> {
    pub fn builder() -> StoresInRegionBuilder<'a> {
        StoresInRegionBuilder::default()
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
