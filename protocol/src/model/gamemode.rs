use crate::model::VariantNotFound;
use netherite_common::macros::{EnumFields, EnumValues};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, EnumFields, EnumValues, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[enum_field(str: String, num: i32, variant_name name: String)]
pub enum Gamemode {
    #[ef(str = "a", num = 2)]
    Adventure,
    #[ef(str = "s", num = 0)]
    Survival,
    #[ef(str = "c", num = 1)]
    Creative,
    #[ef(str = "sp", num = 3)]
    Spectator,
}

impl FromStr for Gamemode {
    type Err = VariantNotFound;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variants =
            Gamemode::VALUES.map(|a: Gamemode| (a, [a.name(), a.str(), a.num().to_string()]));

        for (gm, matchers) in variants {
            if matchers.into_iter().any(|m| m.eq_ignore_ascii_case(s)) {
                return Ok(gm);
            }
        }

        Err(VariantNotFound::new("Gamemode", s))
    }
}

impl Display for Gamemode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}