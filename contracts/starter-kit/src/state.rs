use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const COUNTERS: Map<&str, Uint128> = Map::new("counters");

pub const COUNT_TEST: Item<u64> = Item::new("count_test");

pub const ADMIN: Item<Addr> = Item::new("admin");

pub const DOMAIN_PKS: Map<String, String> = Map::new("domain_pks");
