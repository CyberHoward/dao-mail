use crate::ContractError;
use cosmwasm_std::{DepsMut, Env, Reply, Response, StdError};
use cw_utils::parse_execute_response_data;
use osmosis_std::types::osmosis::smartaccount::v1beta1::MsgAddAuthenticatorResponse;

pub fn reply_on_instantiate(
    _deps: DepsMut,
    env: Env,
    reply: Reply,
) -> Result<Response, ContractError> {
    // Manual protobuf decoding
    let binding = reply.result.unwrap().data.unwrap();
    let mut data = binding.as_slice();
    // let mut data = reply.result.unwrap().data.unwrap().as_slice();
    // let res: MsgAddAuthenticatorResponse = MsgAddAuthenticatorResponse::parse(&mut data)
    //     .map_err(|_| ContractError::Std(StdError::generic_err("reply parse error".to_string())))?;

    // let res_data = parse_execute_response_data(reply.result.unwrap().data.unwrap().as_slice())
    //     .map_err(|_| ContractError::Std(StdError::generic_err("reply parse error".to_string())))?;

    // println!("reply_on_instantiate: {:?}", res);

    // let mint_req = MsgMint {
    //     sender: env.contract.address.to_string(),
    //     mint_to_address: env.contract.address.to_string(),
    //     amount: Some(Coin {
    //         denom: denom.to_string(),
    //         amount: TOKENS_SUPPLY.to_string(),
    //     }),
    // };
    // let mint_sub_msg = CosmosMsg::Stargate {
    //     type_url: "/osmosis.tokenfactory.v1beta1.MsgMint".to_string(),
    //     value: Binary(mint_req.encode_to_vec()),
    // };
    //
    // // This will set metadata for the denom in the Bank Module
    // let set_metadata_req = MsgSetDenomMetadata {
    //     sender: env.contract.address.to_string(),
    //     metadata: Some(Metadata {
    //         description:
    //         "Staking token for AllianceNFT used by the NFT collection to generate rewards"
    //             .to_string(),
    //         denom_units: vec![DenomUnit {
    //             denom: denom.to_string(),
    //             exponent: 0,
    //             aliases: vec![],
    //         }],
    //         base: denom.to_string(),
    //         display: denom,
    //         name: "Alliance Token".to_string(),
    //         symbol: SUBDENOM.to_string(),
    //         uri: "".to_string(),
    //         uri_hash: "".to_string(),
    //     }),
    // };
    //
    // let sub_msg_set_metadata = CosmosMsg::Stargate {
    //     type_url: "/osmosis.tokenfactory.v1beta1.MsgSetDenomMetadata".to_string(),
    //     value: Binary(set_metadata_req.encode_to_vec()),
    // };

    Ok(
        Response::new(), // .add_submessage(SubMsg::new(sub_msg_set_metadata))
                         // .add_submessage(SubMsg::new(mint_sub_msg)))
    )
}
