use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const NOTINUSE: Item<String> = Item::new("not_in_use__");
