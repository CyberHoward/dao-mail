use cosmwasm_std::{DepsMut, Env, Response};
use cw_authenticator::OnAuthenticatorAddedRequest;

use crate::authenticator::{handler::validate_and_parse_params, AuthenticatorError};

pub fn on_authenticator_added(
    _deps: DepsMut,
    _env: Env,
    OnAuthenticatorAddedRequest {
        authenticator_params,
        ..
    }: OnAuthenticatorAddedRequest,
) -> Result<Response, AuthenticatorError> {
    let _ = validate_and_parse_params(authenticator_params);

    Ok(Response::new().add_attribute("action", "on_authenticator_added"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::counter::params::CounterParams;
    use cosmwasm_std::{
        testing::{mock_dependencies_with_balances, mock_env},
        to_json_binary, Addr, Coin, Uint128,
    };

    const USDC: &str = "ibc/498A0751C798A0D9A389AA3691123DADA57DAA4FE165D5C75894505B876BA6E4";

    #[test]
    fn test_on_authenticator_added() {
        let mut deps = mock_dependencies_with_balances(&[("someoneelse", &[Coin::new(1, USDC)])]);

        // missing authenticator_params
        //   let request = OnAuthenticatorAddedRequest {
        //       authenticator_id: "2".to_string(),
        //       account: Addr::unchecked("addr"),
        //       authenticator_params: None,
        //   };
        //   assert_eq!(
        //       on_authenticator_added(deps.as_mut(), mock_env(), request).unwrap_err(),
        //       AuthenticatorError::MissingAuthenticatorParams
        //   );

        // invalid authenticator_params
        let _request = OnAuthenticatorAddedRequest {
            authenticator_id: "2".to_string(),
            account: Addr::unchecked("addr"),
            authenticator_params: Some(to_json_binary(&"invalid").unwrap()),
        };

        //        assert_eq!(
        //            on_authenticator_added(deps.as_mut(), mock_env(), request).unwrap_err(),
        //            AuthenticatorError::invalid_authenticator_params(StdError::parse_err(
        //                std::any::type_name::<SpendLimitParams>(),
        //                "Invalid type"
        //            ))
        //        );

        // valid
        let request = OnAuthenticatorAddedRequest {
            authenticator_id: "2".to_string(),
            account: Addr::unchecked("addr"),
            authenticator_params: Some(
                to_json_binary(&CounterParams {
                    limit: Uint128::new(500_000_000),
                })
                .unwrap(),
            ),
        };

        let res = on_authenticator_added(deps.as_mut(), mock_env(), request).unwrap();
        assert_eq!(
            res,
            Response::new().add_attribute("action", "on_authenticator_added")
        );

        // check the state
        //        let spending = SPENDINGS
        //            .load(deps.as_ref().storage, (&Addr::unchecked("addr"), "2"))
        //            .unwrap();
        //        assert_eq!(spending, Spending::default());

        // Adding the authenticator with the same (account, authenticator_id) should fail
        //    let request = OnAuthenticatorAddedRequest {
        //        authenticator_id: "2".to_string(),
        //        account: Addr::unchecked("addr"),
        //        authenticator_params: Some(
        //            to_json_binary(&CounterParams {
        //                limit: Uint128::new(500_000_000),
        //            })
        //            .unwrap(),
        //        ),
        //    };

        //    assert_eq!(
        //        on_authenticator_added(deps.as_mut(), mock_env(), request).unwrap_err(),
        //        AuthenticatorError::authenticator_already_exists(Addr::unchecked("addr"), "2")
        //    );
    }
}
