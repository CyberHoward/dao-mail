// Ignore integration tests for code coverage since there will be problems with dynamic linking libosmosistesttube
// and also, tarpaulin will not be able to read coverage out of wasm binary anyway
#![cfg(all(test, not(tarpaulin)))]

use cosmwasm_std::{Coin, Timestamp, Uint128, wasm_execute};
use osmosis_std::types::cosmwasm::wasm::v1::{MsgExecuteContractResponse, MsgInstantiateContractResponse};
use osmosis_std::types::osmosis::smartaccount;
// XXX: Leaving commented here for an example
// use osmosis_std::types::osmosis::poolmanager::v1beta1::{
//     EstimateSwapExactAmountInRequest, EstimateSwapExactAmountInResponse,
// };
//use osmosis_std::types::osmosis::{
//    gamm::v1beta1::MsgSwapExactAmountInResponse,
//    poolmanager::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute},
//    smartaccount::v1beta1::{MsgRemoveAuthenticator, MsgRemoveAuthenticatorResponse},
//};
// use osmosis_test_tube::osmosis_std::types::cosmos::bank::v1beta1::QueryBalanceRequest;
use osmosis_test_tube::{cosmrs::proto::tendermint::v0_37::abci::ResponseDeliverTx, osmosis_std::types::cosmos::bank::v1beta1::MsgSend, Account, FeeSetting, Module, OsmosisTestApp, RunnerExecuteResult, SigningAccount, Wasm, Runner, ExecuteResponse};

// use crate::ContractError;
use crate::{
    counter::params::CounterParams,
    msg::{InstantiateMsg, QueryMsg, CounterResponse},
    test_helper::authenticator_setup::{
        //add_1ct_session_authenticator, add_all_of_sig_ver_spend_limit_authenticator,
        add_email_auth_authenticator, email_auth_instantiate, spend_limit_store_code,
    },
};
use crate::dkim::DomainAuthConfig;
use crate::msg::ExecuteMsg;
use crate::test_helper::constants::{ABSTRACT_DKIM_PUBLIC_KEY, ABSTRACT_DOMAIN};
//const UUSDC: &str = "ibc/498A0751C798A0D9A389AA3691123DADA57DAA4FE165D5C75894505B876BA6E4";
//const UATOM: &str = "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2";

#[test]
fn test_happy_path_integration() {
    let app = OsmosisTestApp::new();
    set_maximum_unauthenticated_gas(&app, MAXIMUM_UNAUTHENTICATED_GAS);
    let acc_1 = app
        .init_account(&[Coin::new(1_000_000_000_000_000, "uosmo")])
        .unwrap();

    let acc_2 = app
        .init_account(&[Coin::new(1_000_000_000_000_000, "uosmo")])
        .unwrap();

    let wasm = Wasm::new(&app);

    let params = CounterParams {
        limit: Uint128::new(1_500_000),
    };

    // Store code and initialize spend limit contract
    let code_id = spend_limit_store_code(&wasm, &acc_1);
    let ExecuteResponse::<_> { data: MsgInstantiateContractResponse { address: authenticator_addr, .. }, events, .. }  = email_auth_instantiate(
        &wasm,
        code_id,
        &InstantiateMsg {
            auth: DomainAuthConfig {
                domain: ABSTRACT_DOMAIN.to_string(),
                dkim_pk: ABSTRACT_DKIM_PUBLIC_KEY.to_string(),
            },
            params: params.clone(),
        },
        &acc_1,
    );

    println!("Events: {:?}", events);

    // let authenticator_id =

    // let email_dao_addr = email_auth_instantiate(
    //     &wasm,
    //     code_id,
    //     &InstantiateMsg {},
    //     &acc_1,
    // );

    // execute_email_dao(&app, &acc_1, &authenticator_addr, ExecuteMsg::AddAuthenticator {
    //     contract: authenticator_addr.clone(),
    //     params: params.clone()
    // }).unwrap();

//    let spend_limit_querier = SpendLimitQuerier::new(&app, contract_addr.to_string());

    // Add spend limit authenticator

    let spend_limit_auth_id = add_email_auth_authenticator(
        &app,
        &acc_1,
        &authenticator_addr,
        &params,
    );

    let acc_1_custom_fee = acc_1.with_fee_setting(FeeSetting::Custom {
        amount: Coin::new(500_000, "uosmo"),
        gas_limit: 1_000_000,
    });

    // spend to the limit
    bank_send(
        &app,
        &acc_1_custom_fee,
        &acc_1_custom_fee,
        &acc_2.address(),
        vec![Coin::new(1_000_000, "uosmo")],
        spend_limit_auth_id,
    )
    .unwrap();

    let timestamp = Timestamp::from_nanos(app.get_block_time_nanos() as u64);

 //   assert_eq!(
 //       spend_limit_querier
 //           .query_spendings_by_account(acc_1_custom_fee.address())
 //           .unwrap(),
 //       vec![(
 //           "1".to_string(),
 //           Spending {
 //               value_spent_in_period: Uint128::new(1_500_000),
 //               last_spent_at: timestamp
 //           }
 //       )]
 //   );

    // spend some more
    let acc_1_custom_fee = acc_1_custom_fee.with_fee_setting(FeeSetting::Custom {
        amount: Coin::new(2500, "uosmo"),
        gas_limit: 1_000_000,
    });
    let res = bank_send(
        &app,
        &acc_1_custom_fee,
        &acc_1_custom_fee,
        &acc_2.address(),
        vec![Coin::new(1, "uosmo")],
        spend_limit_auth_id,
    );
//    assert_substring!(
//        res.as_ref().unwrap_err().to_string(),
//        SpendLimitError::overspend(1_500_000, 1_502_500).to_string()
//    );


//    app.increase_time(diff as u64);

    bank_send(
        &app,
        &acc_1_custom_fee,
        &acc_1_custom_fee,
        &acc_2.address(),
        vec![Coin::new(1_400_000, "uosmo")],
        spend_limit_auth_id,
    )
    .unwrap();

    bank_send(
        &app,
        &acc_1_custom_fee,
        &acc_1_custom_fee,
        &acc_2.address(),
        vec![Coin::new(92_500, "uosmo")],
        spend_limit_auth_id,
    )
    .unwrap();

//    assert_substring!(
//        err.to_string(),
//        SpendLimitError::overspend(1_500_000, 1_500_001).to_string()
//    );
}

fn execute_email_dao(app: &OsmosisTestApp, account: &SigningAccount, email_dao_addr: &str, msg: ExecuteMsg) -> RunnerExecuteResult<MsgExecuteContractResponse> {
    app.execute_cosmos_msgs::<MsgExecuteContractResponse>(&[wasm_execute(email_dao_addr.to_string(), &msg, vec![]).unwrap().into()], account)
}

const MAXIMUM_UNAUTHENTICATED_GAS: u64 = 120_000;
fn set_maximum_unauthenticated_gas(app: &OsmosisTestApp, maximum_unauthenticated_gas: u64) {
    app.set_param_set(
        "smartaccount",
        smartaccount::v1beta1::Params {
            maximum_unauthenticated_gas,
            is_smart_account_active: true,
            circuit_breaker_controllers: vec![],
        }
        .to_any(),
    )
    .unwrap();
}

fn bank_send(
    app: &OsmosisTestApp,
    account: &SigningAccount,
    signer: &SigningAccount,
    to_address: &str,
    amount: Vec<Coin>,
    authenticator_id: u64,
) -> RunnerExecuteResult<ResponseDeliverTx> {
    let amount: Vec<osmosis_test_tube::osmosis_std::types::cosmos::base::v1beta1::Coin> =
        amount.into_iter().map(Into::into).collect();

    app.execute_with_selected_authenticators(
        vec![MsgSend {
            from_address: account.address(),
            to_address: to_address.to_string(),
            amount,
        }
        .to_any()
        .into()],
        account,
        signer,
        &[authenticator_id],
    )?
    .try_into()
}
