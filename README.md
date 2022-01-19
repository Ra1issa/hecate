# Hecate: Abuse Reporting in Secure Messengers with Sealed Sender

## Generating Tokens

At the start of the protocol the moderator generates tokens and sends them via:

`cargo run --release --bin moderator_generate`

The sender fetches them via:

`cargo run --release --bin sender_fetch`


## Franking Messages

The sender can 
