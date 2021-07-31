use std::{borrow::Cow, collections::BTreeSet};

use derive_builder::Builder;
use http::Method;
use serde::{Serialize, Serializer};

use super::endpoint::Endpoint;

#[derive(Debug, Clone)]
pub enum Direction {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub enum DealsSorting {
    Time(Direction),
    Price(Direction),
    Cut(Direction),
    Expiry(Direction),
}

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "snake_case")]
pub struct DealsList<'a> {
    offset: Option<usize>,
    limit: Option<usize>,
    region: Option<Cow<'a, str>>,
    country: Option<Cow<'a, str>>,
    #[builder(setter(name = "_shops"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    shops: BTreeSet<Cow<'a, str>>,
    #[serde(serialize_with = "serialize_sorting")]
    sort: Option<DealsSorting>,
}

impl<'a> DealsList<'a> {
    pub fn builder() -> DealsListBuilder<'a> {
        DealsListBuilder::default()
    }
}

impl<'a> DealsListBuilder<'a> {
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
}

impl Endpoint for DealsList<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v01/deals/list/".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}

fn serialize_sorting<S>(value: &Option<DealsSorting>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(value) = value {
        let mut out = String::new();
        let direction = match value {
            DealsSorting::Time(direction) => {
                out.push_str("time:");
                direction
            }
            DealsSorting::Price(direction) => {
                out.push_str("price:");
                direction
            }
            DealsSorting::Cut(direction) => {
                out.push_str("cut:");
                direction
            }
            DealsSorting::Expiry(direction) => {
                out.push_str("expiry:");
                direction
            }
        };
        match direction {
            Direction::Asc => out.push_str("asc"),
            Direction::Desc => out.push_str("desc"),
        };
        serializer.serialize_str(&out)
    } else {
        serializer.serialize_none()
    }
}

//TODO: Pageable?
