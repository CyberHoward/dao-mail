use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetDnsServerResponse, InstantiateMsg, QueryMsg, WardenRequestFuture};
use crate::state::{State, DNS_SERVER, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:warden-poller";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    DNS_SERVER.save(deps.storage, &msg.dns_server)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("dns_server", msg.dns_server.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,

    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::StartDkimTxtRequestFuture { domain } => {
            start_dkim_txt_request_future(deps, info, &domain)
        }
    }
}

fn start_dkim_txt_request_future(
    deps: DepsMut,
    info: MessageInfo,
    domain: &String,
) -> Result<Response, ContractError> {
    if info.sender != STATE.load(deps.storage)?.owner {
        return Err(ContractError::Unauthorized {});
    }
    // TODO HACKATHON: get it working with testnet
    let dns_server = DNS_SERVER.load(deps.storage)?;
    let futures_request = CosmosMsg::Stargate {
        type_url: "/warden/futures/v1beta1/RequestFuture".to_string(),
        value: to_json_binary(&WardenRequestFuture {
            host: dns_server,
            request: "GET /dkim-txt".to_string(),
            params: vec![format!("domain={domain}")],
        })
        .unwrap(),
    };
    // TODO HACKATHON: save futures
    Ok(Response::new()
        .add_attribute("action", "start_dkim_txt_request_future")
        .add_attribute("domain", domain)
        .add_message(futures_request))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetDnsServer {} => to_json_binary(&query::dns_server(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn dns_server(deps: Deps) -> StdResult<GetDnsServerResponse> {
        let dns_server = DNS_SERVER.load(deps.storage)?;
        Ok(GetDnsServerResponse { host: dns_server })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_json};

    pub const MOCK_DNS_SERVER: &str = "https://api.cloudflare.com";
    pub const MOCK_DOMAIN: &str = "abstract.money";

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg::new(MOCK_DNS_SERVER);
        let info = message_info(&deps.api.addr_make("creator"), &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetDnsServer {}).unwrap();
        let res: GetDnsServerResponse = from_json(&res).unwrap();
        assert_eq!(17, res.host);
    }

    #[test]
    fn request_future() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg::new(MOCK_DNS_SERVER);
        let info = message_info(&deps.api.addr_make("creator"), &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = message_info(&deps.api.addr_make("anyone"), &coins(2, "token"));
        let msg = ExecuteMsg::StartDkimTxtRequestFuture {
            domain: MOCK_DOMAIN.into(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // TODO HACKATHON: search for events for the domain in the future
    }
}
