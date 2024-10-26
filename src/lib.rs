use anyhow::Result;
use cosmwasm_std::{coin, coins, Addr, BankMsg, Empty};
use cw_orch_daemon::{networks::OSMO_5, DaemonAsync, DaemonAsyncBase};
use serde::{Deserialize, Serialize};

const CONTRACT_ADDRESS: &str = "0x1234567890abcdef1234567890abcdef12345678";

#[derive(Serialize, Deserialize)]
pub struct Payment {
    recipient: String,
    amount: u64,
    denomination: String,
}

pub async fn propose(title: String, description: String, payment: Payment) -> Result<()> {
    let daemon = DaemonAsync::builder(OSMO_5).build().await?;

    let payment_msg = BankMsg::Send {
        to_address: payment.recipient,
        amount: coins(payment.amount as u128, payment.denomination),
    };

    daemon
        .execute(
            &cw3::Cw3ExecuteMsg::<Empty>::Propose {
                title,
                description,
                msgs: vec![payment_msg.into()],
                earliest: None,
                latest: None,
            },
            &[],
            &Addr::unchecked(CONTRACT_ADDRESS),
        )
        .await?;

    Ok(())
}
