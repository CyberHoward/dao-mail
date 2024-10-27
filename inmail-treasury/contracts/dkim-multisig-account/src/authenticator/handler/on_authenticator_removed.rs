use cosmwasm_std::{DepsMut, Env, Response};
use cw_authenticator::OnAuthenticatorRemovedRequest;

use crate::authenticator::{handler::validate_and_parse_params, AuthenticatorError};

pub fn on_authenticator_removed(
    _deps: DepsMut,
    _env: Env,
    OnAuthenticatorRemovedRequest {
        authenticator_params,
        ..
    }: OnAuthenticatorRemovedRequest,
) -> Result<Response, AuthenticatorError> {
    let _ = validate_and_parse_params(authenticator_params)?;

    // TODO HACKATHON: implement removed logic...

    Ok(Response::new().add_attribute("action", "on_authenticator_removed"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::counter::params::EmailAuthParams;
    use crate::msg::EmailAuthDetails;
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env},
        to_json_binary, Addr,
    };

    #[test]
    fn test_on_authenticator_removed() {
        let mut deps = mock_dependencies();

        // remove the spending
        let _key = (&Addr::unchecked("account"), "2");
        //SPENDINGS
        //    .save(deps.as_mut().storage, key, &Spending::default())
        //    .unwrap();
        //assert!(SPENDINGS.has(deps.as_ref().storage, key));

        let msg = OnAuthenticatorRemovedRequest {
            authenticator_id: "2".to_string(),
            account: Addr::unchecked("account"),
            authenticator_params: Some(
                to_json_binary(&EmailAuthParams {
                    limit: 1000u128.into(),
                    auth: EmailAuthDetails::mock(),
                })
                .unwrap(),
            ),
        };

        on_authenticator_removed(deps.as_mut(), mock_env(), msg).unwrap();
        //assert!(!SPENDINGS.has(deps.as_ref().storage, key));
    }
}
