use cosmwasm_std::{CosmosMsg, DepsMut, entry_point, Env, MessageInfo, Response, SubMsg, to_json_binary, wasm_execute, WasmMsg};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::smartaccount::v1beta1::MsgAddAuthenticator;
use osmosis_test_tube::cosmrs::bip32::secp256k1::pkcs8::der::Encode;
use crate::authenticator::CosmwasmAuthenticatorData;
use crate::contract::{CONTRACT_NAME, CONTRACT_VERSION};
use crate::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(info.sender.as_str()))?;


    //
    // let add_auth_msg = CosmosMsg::Stargate {
    //     type_url: "/osmosis.smartaccount.v1beta1.MsgAddAuthenticator".to_string(),
    //     value: to_json_binary(&add_auth_msg).unwrap(),
    // };

    Ok(Response::new().add_attribute("action", "instantiate")
    //     .add_submessage(SubMsg::new(wasm_execute(env.contract.address, &ExecuteMsg::Execute {
    //     msgs: vec![add_auth_msg.into()],
    // }, vec![])?))
    )
}
