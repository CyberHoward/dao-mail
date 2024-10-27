# Authenticator Starter Kit

## Overview

This project contains a starter kit for authenticators, it exposes all the sudo functions so the contract can be uploaded to a node.


### Beware of dragons

- sorry this project is a mess, feel free to create a PR into this to improve <3

## Development

### Pre-requisites

- [Rust](https://www.rust-lang.org/)
- [Go](https://golang.org/) (for running integration tests & localosmosis)
- [CosmWasm Setup](https://book.cosmwasm.com/setting-up-env.html)
- [Beaker](https://github.com/osmosis-labs/beaker)
- [Docker](https://www.docker.com/)

## Errors

```solidity
  error occurred: Command "sccache" "clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fno-exceptions" "--target=wasm32-unknown-unknown" "-I" "include" "-I" "/Users/adair/Development/Hackathons/Hackmos2024/dkim-auth/target/wasm32-unknown-unknown/release/build/ring-e931193a3221ee99/out" "-Wall" "-Wextra" "-fvisibility=hidden" "-std=c1x" "-Wall" "-Wbad-function-cast" "-Wcast-align" "-Wcast-qual" "-Wconversion" "-Wmissing-field-initializers" "-Wmissing-include-dirs" "-Wnested-externs" "-Wredundant-decls" "-Wshadow" "-Wsign-compare" "-Wsign-conversion" "-Wstrict-prototypes" "-Wundef" "-Wuninitialized" "-g3" "-nostdlibinc" "-DNDEBUG" "-DRING_CORE_NOSTDLIBINC=1" "-o" "/Users/adair/Development/Hackathons/Hackmos2024/dkim-auth/target/wasm32-unknown-unknown/release/build/ring-e931193a3221ee99/out/fad98b632b8ce3cc-curve25519.o" "-c" "crypto/curve25519/curve25519.c" with args clang did not execute successfully (status code exit status: 1).

```
See https://github.com/briansmith/ring/issues/1824#issuecomment-2059955073