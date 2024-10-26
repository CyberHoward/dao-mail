use cosmwasm_std::{CosmosMsg, DepsMut, entry_point, Env, MessageInfo, Response, SubMsg, to_json_binary, wasm_execute, WasmMsg};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::smartaccount::v1beta1::MsgAddAuthenticator;
use crate::authenticator::CosmwasmAuthenticatorData;
use crate::contract::{CONTRACT_NAME, CONTRACT_VERSION};
use crate::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::replies::reply::INSTANTIATE_REPLY_ID;
use crate::state::DOMAIN_PKS;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(info.sender.as_str()))?;

    let auth_data = CosmwasmAuthenticatorData {
        contract: env.contract.address.to_string(),
        params: to_json_binary(&msg.params).unwrap().to_vec(),
    };

    let add_auth_msg = MsgAddAuthenticator {
        sender: env.contract.address.to_string(),
        r#type: "CosmwasmAuthenticatorV1".to_string(),
        data: to_json_binary(&auth_data).unwrap().to_vec(),
    };

    msg.auth.verify()?;

    Ok(Response::new().add_attribute("action", "instantiate").add_message(add_auth_msg)
    // Ok(Response::new().add_attribute("action", "instantiate").add_submessage(SubMsg::reply_always(add_auth_msg, INSTANTIATE_REPLY_ID))
    //     .add_submessage(SubMsg::new(wasm_execute(env.contract.address, &ExecuteMsg::Execute {
    //     msgs: vec![add_auth_msg.into()],
    // }, vec![])?))
    )
}
