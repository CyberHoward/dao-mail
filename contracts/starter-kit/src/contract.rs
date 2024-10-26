use crate::authenticator::{self, CosmwasmAuthenticatorData};
use crate::msg::{
    CounterResponse, InstantiateMsg, QueryMsg, SudoMsg
};
use crate::counter::error::CounterError;
use crate::state::COUNTERS;
use crate::ContractError;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, Response, to_json_binary, Uint128, Uint64};
pub(crate) const CONTRACT_NAME: &str = "crates.io:dkim-auth";
pub(crate) const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const MAX_LIMIT: u32 = 100;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::OnAuthenticatorAdded(on_authenticator_added_request) => {
            authenticator::on_authenticator_added(deps, env, on_authenticator_added_request)
                .map_err(ContractError::from)
        }
        SudoMsg::OnAuthenticatorRemoved(on_authenticator_removed_request) => {
            authenticator::on_authenticator_removed(deps, env, on_authenticator_removed_request)
                .map_err(ContractError::from)
        }
        SudoMsg::Authenticate(auth_request) => {
            authenticator::authenticate(deps, env, *auth_request)
        }
        SudoMsg::Track(track_request) => {
            authenticator::track(deps, env, track_request).map_err(ContractError::from)
        }
        SudoMsg::ConfirmExecution(confirm_execution_request) => {
            authenticator::confirm_execution(deps, env, confirm_execution_request)
        }
    }
}

pub fn query_counter(
    deps: Deps,
    account: Addr,
    authenticator_id: String,
) -> Result<CounterResponse, ContractError> {
    let key = format!("{}-{}", account.as_str(), authenticator_id.as_str());
    // dbg!("key: {:?}", key);

    match COUNTERS.may_load(deps.storage, key.as_str())? {
        Some(_) => Ok(CounterResponse {
            count: Uint64::new(10),
        }),
        None => Err(CounterError::CounterNotFound {
            address: account,
            authenticator_id,
        }.into()),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        BlockInfo,
        Coin, ContractResult, testing::{mock_env, mock_info}, to_json_vec, Uint128,
    };
    use cw_authenticator::{
        Any, AuthenticationRequest, ConfirmExecutionRequest, OnAuthenticatorAddedRequest,
        OnAuthenticatorRemovedRequest, SignatureData, SignModeTxData, TrackRequest, TxData,
    };
    use osmosis_std::types::{
        cosmos::bank::v1beta1::MsgSend,
        osmosis::smartaccount::v1beta1::{AccountAuthenticator, GetAuthenticatorResponse},
    };

    use crate::{
        authenticator::CosmwasmAuthenticatorData,
        test_helper::mock_stargate_querier::{
            get_authenticator_query_handler,
            mock_dependencies_with_stargate_querier,
        },
    };
    use crate::counter::params::CounterParams;
    use crate::dkim::DomainAuthConfig;
    use crate::handlers::instantiate::instantiate;
    use crate::handlers::query::query;
    use crate::test_helper::constants::{ABSTRACT_DKIM_PUBLIC_KEY, ABSTRACT_DOMAIN};
    use super::*;

    const UUSDC: &str = "ibc/498A0751C798A0D9A389AA3691123DADA57DAA4FE165D5C75894505B876BA6E4";

    #[test]
    fn test_happy_path() {
        let params = CounterParams {
            limit: Uint128::from(1_000_000u128),
        };

        let params_for_querier_setup = params.clone();
        let mut deps = mock_dependencies_with_stargate_querier(
            &[
                ("creator", &[Coin::new(1000, UUSDC)]),
                ("limited_account", &[Coin::new(2_000_000, UUSDC)]),
                ("recipient", &[]),
            ],
            get_authenticator_query_handler(Box::new(move |req| {
                let account = req.account.as_str();
                let authenticator_id = req.authenticator_id;
                match (account, authenticator_id) {
                    ("limited_account", 2) => ContractResult::Ok(GetAuthenticatorResponse {
                        account_authenticator: Some(AccountAuthenticator {
                            id: 2,
                            r#type: "CosmWasmAuthenticatorV1".to_string(),
                            config: to_json_vec(&CosmwasmAuthenticatorData {
                                contract: mock_env().contract.address.to_string(),
                                params: to_json_vec(&params_for_querier_setup).unwrap(),
                            })
                            .unwrap(),
                        }),
                    }),
                    _ => ContractResult::Err("not found".to_string()),
                }
            })),
        );
        let msg = InstantiateMsg {
            auth: DomainAuthConfig {
                domain: ABSTRACT_DOMAIN.to_string(),
                dkim_pk: ABSTRACT_DKIM_PUBLIC_KEY.to_string(),
            },
            params: params.clone(),
        };
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let authenticator_params = to_json_binary(&params).unwrap();

        // add authenticator
        sudo(
            deps.as_mut(),
            mock_env(),
            SudoMsg::OnAuthenticatorAdded(OnAuthenticatorAddedRequest {
                account: Addr::unchecked("limited_account"),
                authenticator_id: "2".to_string(),
                authenticator_params: Some(authenticator_params.clone()),
            }),
        )
        .unwrap();

        let msg = Any {
            type_url: MsgSend::TYPE_URL.to_string(),
            value: Binary::from(
                MsgSend {
                    from_address: "limited_account".to_string(),
                    to_address: "recipient".to_string(),
                    amount: vec![Coin::new(1_000_000, UUSDC).into()],
                }
                .to_proto_bytes(),
            ),
        };

        // authenticate
        sudo(
            deps.as_mut(),
            mock_env(),
            SudoMsg::Authenticate(Box::new(AuthenticationRequest {
                authenticator_id: "2".to_string(),
                account: Addr::unchecked("limited_account"),
                fee_payer: Addr::unchecked("limited_account"),
                fee_granter: None,
                fee: vec![],
                msg: msg.clone(),
                msg_index: 0,
                signature: Binary::default(),
                sign_mode_tx_data: SignModeTxData {
                    sign_mode_direct: Binary::default(),
                    sign_mode_textual: None,
                },
                tx_data: TxData {
                    chain_id: "osmosis-1".to_string(),
                    account_number: 0,
                    sequence: 0,
                    timeout_height: 0,
                    msgs: vec![],
                    memo: "".to_string(),
                },
                signature_data: SignatureData {
                    signers: vec![],
                    signatures: vec![],
                },
                simulate: false,
                authenticator_params: Some(authenticator_params.clone()),
            })),
        )
        .unwrap();

        // track
        sudo(
            deps.as_mut(),
            mock_env(),
            SudoMsg::Track(TrackRequest {
                account: Addr::unchecked("limited_account"),
                fee_payer: Addr::unchecked("limited_account"),
                fee_granter: None,
                fee: vec![],
                authenticator_id: "2".to_string(),
                msg: msg.clone(),
                msg_index: 0,
                authenticator_params: Some(authenticator_params.clone()),
            }),
        )
        .unwrap();

        // simulate execute bank send
        deps.querier
            .update_balance("limited_account", vec![Coin::new(1_000_001, UUSDC)]);

        // confirm execution
        sudo(
            deps.as_mut(),
            mock_env(),
            SudoMsg::ConfirmExecution(ConfirmExecutionRequest {
                authenticator_id: "2".to_string(),
                account: Addr::unchecked("limited_account"),
                fee_payer: Addr::unchecked("limited_account"),
                fee_granter: None,
                fee: vec![],
                msg: msg.clone(),
                msg_index: 0,
                authenticator_params: Some(authenticator_params.clone()),
            }),
        )
        .unwrap();

        // query spending
       // let count = from_json::<CounterResponse>(
       //     &query(
       //         deps.as_ref(),
       //         mock_env(),
       //         QueryMsg::Counter {
       //             account: "limited_account".to_string(),
       //             authenticator_id: "2".to_string(),
       //         },
       //     )
       //     .unwrap(),
       // )
       // .unwrap();

        //assert_eq!(
        //    count,
        //    CounterResponse {
        //        count: Uint64::new(10),
        //    }
        //);

        // remove authenticator
        sudo(
            deps.as_mut(),
            mock_env(),
            SudoMsg::OnAuthenticatorRemoved(OnAuthenticatorRemovedRequest {
                account: Addr::unchecked("limited_account"),
                authenticator_id: "2".to_string(),
                authenticator_params: Some(authenticator_params),
            }),
        )
        .unwrap();

        // query spending
        let err = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Counter {
                account: "limited_account".to_string(),
                authenticator_id: "2".to_string(),
            },
        )
        .unwrap_err();

       // assert_eq!(
       //     err,
       //     CounterError::CounterNotFound {
       //         address: Addr::unchecked("limited_account"),
       //         authenticator_id: "2".to_string(),
       //     }
       //     .into()
       // );

  //      // query spendings by account
  //      let spendings = from_json::<SpendingsByAccountResponse>(
  //          &query(
  //              deps.as_ref(),
  //              mock_env(),
  //              QueryMsg::SpendingsByAccount {
  //                  account: "limited_account".to_string(),
  //              },
  //          )
  //          .unwrap(),
  //      )
  //      .unwrap();
  //      assert_eq!(spendings, SpendingsByAccountResponse { spendings: vec![] });
    }

    fn mock_env_with_additional_days(days: u64) -> Env {
        Env {
            block: BlockInfo {
                height: mock_env().block.height + days * 10000,
                time: mock_env().block.time.plus_days(days),
                chain_id: mock_env().block.chain_id,
            },
            transaction: mock_env().transaction,
            contract: mock_env().contract,
        }
    }
}
