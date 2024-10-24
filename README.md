# Authenticator Starter Kit

## Overview


- For this authenticator, it is always `Authenticated` in `Authenticate` hook, since it will check the spend limit in `ConfirmExecution` hook.
- It checks by using `Track` hook to get the pre execution balances of the account.
- then, `ConfirmExecution` hook get the post execution balances of the account. The difference between the pre and post execution balances is the amount spent in the transaction.
- If last spending update was within the past set period, it resets the spending to 0.
- The amount spent are then converted into quoted denom using TWAP price.
- If the amount spent is greater than the spend limit, the transaction will be rejected. If not, it will be accepted and the spending will be accumulated.

## Development

### Pre-requisites

- [Rust](https://www.rust-lang.org/)
- [Go](https://golang.org/) (for running integration tests & localosmosis)
- [CosmWasm Setup](https://book.cosmwasm.com/setting-up-env.html)
- [Beaker](https://github.com/osmosis-labs/beaker)
- [Docker](https://www.docker.com/)
