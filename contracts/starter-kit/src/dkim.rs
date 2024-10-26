use crate::state::DOMAIN_PKS;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{StdResult, Storage};

#[cw_serde]
pub struct DomainAuthConfig {
    pub domain: String,
    pub dkim_pk: String,
}

impl DomainAuthConfig {
    pub fn verify(&self) -> StdResult<()> {
        // TODO: verify the public key
        Ok(())
    }

    pub fn save(self, storage: &mut dyn Storage) -> StdResult<()> {
        DOMAIN_PKS.save(storage, self.domain, &self.dkim_pk)
    }
}
