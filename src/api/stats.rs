use std::borrow::Cow;

use derive_builder::Builder;
use http::Method;
use serde::Serialize;

use super::endpoint::Endpoint;

/// This module contains two private endpoints: `Waitlist Price Limits` and
/// `Waitlist Cut Limits`. As these endpoints are private, and require explicit
/// permission, they are not implemented here.

// TODO: Pagination
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct WaitlistChart {
    offset: Option<usize>,
    limit: Option<usize>,
}

impl WaitlistChart {
    pub fn builder() -> WaitlistChartBuilder {
        WaitlistChartBuilder::default()
    }
}

impl Endpoint for WaitlistChart {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/stats/waitlist/chart/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}

// TODO: Pagination
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct CollectionChart {
    offset: Option<usize>,
    limit: Option<usize>,
}

impl CollectionChart {
    pub fn builder() -> CollectionChartBuilder {
        CollectionChartBuilder::default()
    }
}

impl Endpoint for CollectionChart {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/stats/collection/chart/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}

// TODO: Pagination
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct PopularityChart {
    offset: Option<usize>,
    limit: Option<usize>,
}

impl PopularityChart {
    pub fn builder() -> PopularityChartBuilder {
        PopularityChartBuilder::default()
    }
}

impl Endpoint for PopularityChart {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/stats/popularity/chart/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}
