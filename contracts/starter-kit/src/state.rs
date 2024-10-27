use crate::dkim::DkimAuthConfig;
use crate::msg::EmailAddress;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const COUNTERS: Map<&str, Uint128> = Map::new("counters");

pub const COUNT_TEST: Item<u64> = Item::new("count_test");

pub const ADMIN: Item<Addr> = Item::new("admin");

pub const DOMAIN: Item<String> = Item::new("domain");
pub const PUBLIC_DKIM_KEY: Item<String> = Item::new("public_key");

pub const DKIM_AUTH_CONFIG: Item<DkimAuthConfig> = Item::new("dkim_auth_config");

// email to weight
pub const EMAILS: Map<&EmailAddress, u64> = Map::new("emails");
