# Warden Poller

This contract is meant to serve as an outpost on Warden chain to request DNS TXT records for specific domains so that we can retrieve the DKIM public key.

It is meant to call the x/futures module described [here](https://docs.google.com/presentation/d/1b8zLjXBoF-Du29WFf7FXd-MqleeBpYrB60nFeASZX0Q/edit#slide=id.g30e3b5aee7e_0_72), though the calls are stubbed for now.

## Links
- [Build a warden app](https://docs.wardenprotocol.org/build-an-app/introduction)
- [x/act docs](https://docs.wardenprotocol.org/learn/warden-protocol-modules/x-act)