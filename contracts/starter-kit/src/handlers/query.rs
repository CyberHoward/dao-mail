use cosmwasm_std::{Api, Binary, Deps, entry_point, Env, to_json_binary};
use crate::{contract, ContractError};
use crate::msg::QueryMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Counter {
            account,
            authenticator_id,
        } => {
            let account = deps.api.addr_validate(&account)?;
            to_json_binary(&contract::query_counter(
                deps,
                account,
                authenticator_id,
            )?)
        }
    }
    .map_err(ContractError::from)
}