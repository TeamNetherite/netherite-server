use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use stdto::ToStringForBytes;

#[derive(thiserror::Error, Debug)]
#[error("Variant of {0} not found: {1}")]
pub struct VariantNotFound(String, String);

impl VariantNotFound {
    pub const fn new(enum_name: &str, variant: &str) -> Self {
        VariantNotFound(enum_name.to_string(), variant.to_string())
    }

    pub fn enum_name(&self) -> &str {
        &self.0
    }

    pub fn variant_name(&self) -> &str {
        &self.1
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Invertable<T: Clone + FromStr + Display> {
    Only(T),
    Exclude(T),
}

impl<T: FromStr + Clone + Display> TryFrom<String> for Invertable<T> {
    type Error = T::Err;
    fn try_from(value: String) -> Result<Self, T::Err> {
        Self::from_str(value.as_str())
    }
}

impl<T: Clone + Display + FromStr> Into<String> for Invertable<T> {
    fn into(self) -> String {
        self.to_string()
    }
}

impl<T: Clone + Display + FromStr> ToString for Invertable<T> {
    fn to_string(&self) -> String {
        match self {
            Invertable::Only(only) => format!("{}", only),
            Invertable::Exclude(exclude) => format!("!{}", exclude),
        }
    }
}

impl<T: FromStr + Clone + Display> FromStr for Invertable<T> {
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with("!") {
            Invertable::Exclude(T::from_str(
                s.strip_prefix("!")
                    .expect("Prefix detected but when stripped it is None"),
            )?)
        } else {
            Invertable::Only(s.parse()?)
        })
    }
}

pub mod chat;
pub mod gamemode;
pub mod pos;
pub mod score;
