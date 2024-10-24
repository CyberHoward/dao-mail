use cosmwasm_std::Uint128;
use cw_storage_plus::Map;

pub const COUNTERS: Map<&str, Uint128> = Map::new("counters");

