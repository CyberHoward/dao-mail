# InMail Treasury

InMail Treasury brings secure, AI-powered treasury management to non-crypto-native organizations using a familiar, email-driven interface. We offer organizations a way to control their crypto treasury using emails written in natural language. This both onboards traditional organizations onchain (secured by email DKIM signatures) and allows their employees to control crypto treasuries.

## Overview

InMail is built on an Osmosis Smart Account, with a custom DKIM signature and header authenticator. The Smart Account also contains multisig functionality to enable creation of and voting on proposals. Warden protocol would be used for email polling and DKIM TXT record synchronization.

## Tech

- CosmWasm
- Osmosis Smart Accounts for DKIM authentication
- Warden x/act and (when supported) x/future module

## How it works

1. **Mail Clients**
- Employees send treasury-related emails (proposals, votes) to the organization’s designated email (treasury@abstract.money).
1. **Mail Server**
- Acts as the DAO’s inbox, receiving requests and forwarding them for interpretation and processing.

3. **Inbox Listener**

- A Rust server listens for incoming emails, parsing each one to extract the action type (proposal or vote).
- The action is sent to an LLM to transform the message into a CosmosMsg.
- The CosmosMsg is submitted
- https://github.com/CyberHoward/dao-mail/blob/main/src/bin/main.rs

**4. Osmosis Smart-Account**

- Authenticators: Checks DKIM signatures against the domain’s public key to confirm sender authenticity.
- Execution: Once a proposal reaches quorum, the transaction is executed on-chain, all managed within Osmosis.
- https://github.com/CyberHoward/dao-mail/tree/main/inmail-treasury/contracts/starter-kit

## Generate oauth

`rm oauth/token.json && cd oauth && node .`

Click the link, sign in with **dao@abstract.money** and copy the token in the URL of the localhost that opens.

should look something like `4/0AVG7fiS....yog`

Paste that token in the terminal and it will update the `oauth/token.json` file.

# Links
- [DoraHacks Submission](https://dorahacks.io/buidl/18108/)