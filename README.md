# Hecate: Abuse Reporting in Secure Messengers with Sealed Sender

**Hecate** is a Rust library that implements the work in [[eprint/2021/1686]](https://eprint.iacr.org/2021/1686) on abuse reporting in end to end messaging systems (EEMS) with sealed sender and anonymous networks.

## Structure

**Hecate** considers 5 differents types of parties:
<ol>
  <li>The <em>Moderator</em> who creates tokens for users a-priori and handles user reports. The Moderatoris assumed to be seperate from the Platform. </li>
  <li>The <em>Sender</em> of a message who wishes to send a message via the EEMS extended with abuse reporting to their desired message recipient. Prior to sending the message, the Sender partakes in an offline pre-processing phase where they receive tokens from the Moderator that they later use when sending the message.</li>
  <li>The <em>Receiver</em> of a message who first verifies the well-formedness of the message and then either: (1) reads the message, (2) reports it to the Moderator, or (3) forwards it by acting as a Forwarder.</li>
  <li>The <em>Forwarder</em> of a message who just forwards the message along and is assumed to have already received and apropriately verified the message.</li>
  <li>The <em>Platform</em> (e.g. Signal Server) who relays encrypted messages to their appropriate recipients and timestamps them along the way.</li>
</ol>


Each party can be invoked seperately after they are provided with the material they need (e.g. the sender should be first given tokens before being able to send message). Note that parties write their intended output to files but may easily be changed to send their outputs over TCP channels (the code for doing so is provided is certain cases).

You may refer to the script <code>scripts/run_pipeline.sh</code> for the intended pipeline of **Hecate** (please run <code>scripts/setup_parties.sh</code> first).

### Moderator

The moderator has 3 possible functionalities:
<ol>
  <li><em>Setup</em>. Which creates the long terms key material of the moderator and creates ids for users (currently just a single sender). The setup phase can be run via:
  <code>cargo run --release --bin moderator_setup</code> </li>
  <li><em>Pre-processing</em>. Which creates tokens for a user (currently specified in src/bin/moderator/generate/main.rs). The moderator can optionally send a batch of tokens to the user. The pre-processing phase can be run via:
  <code>cargo run --release --bin moderator_generate</code> </li>
  <li><em>Inspect</em>. Which inspects and traces a message reports. The inspection can be run via:
  <code>cargo run --release --bin moderator_inspect</code> </li>
</ol>

### Sender
The sender has 2 possible functionalities:
<ol>
  <li><em>Fetching Tokens</em>. Which receives tokens from the moderator. This is optional especially in the case where parties are run locally and outputs are written to files. This phase can be run via:
  <code>cargo run --release --bin sender_fetch</code> </li>
  <li><em>Sending.</em>. Which creates the necessary user reporting material for a specific message (the message can be specified in src/bin/sender/send/main.rs). The sending phase can be run via:
  <code>cargo run --release --bin sender_send</code> </li>
</ol>

### Receiver
The receiver has a single functionality:
<ol>
  <li><em>Receiving</em>. Which verifies a received message and generates the report if the receiver decides to report the message. This phase can be run via:
  <code>cargo run --release --bin receive</code> </li>
</ol>

### Forwarder

The forwarder has a single functionality:
<ol>
  <li><em>Forwarding</em>. Which forwards along the message with the correct message body and message envelope. The forwarder is assumed to have already received the message and handled it via the receiver's functionalities. This phase can be run via:
  <code>cargo run --release --bin forward</code> </li>
</ol>

### Platform

The platform has two functionality:
<ol>
  <li><em>Setup</em>. Which creates the long terms key material of the platform. The setup phase can be run via:
  <code>cargo run --release --bin platform_setup</code></li>
  <li><em>Timestamp</em>. Which timestamp and signs an encrypted message envelope as it's being relayed to it's appropriate recipient:
  <code>cargo run --release --bin platform_timestamp</code></li>
</ol>

## Benchmarks

**Hecate** uses Criterion as its testing suite. Each functionality, previously described, is benchmarked in isolation with variying message sizes except for pre-processing. Pre-processing is benchmarked with varying token batches sizes. Depending on your machine, you may need to increase the maximum measurement_time in benches/criterion.rs. You can run all the benchmarks via:

<code>cargo bench</code>

A detailed output can be founded in target/criterion

## Integration with signal-cli

**Hecate** can be integrated into the signal-cli command line Signal client in order to send and receive signal messages extended with user reports.

### Pre-requisites

You will need to clone the following two libraries (modified for use with Hecate):

<ol>
  <li>[libgsignal-client](https://github.com/Ra1issa/libsignal-client)</li>
  <li> [signal-cli](https://github.com/Ra1issa/signal-cli)</li>
</ol>

Make sure that they are in the same directory as Hecate, otherwise you will need to modify each repos CArgo.toml and the makefile in signal-cli accordingly.

## Disclaimer

You will also need at least one phone number to register and use with signal-cli. It's highly recommended that you DO NOT use your own phone number. You can try to use a phone number provided by Google voice instead.

## Installation

In [**signal-cli**](https://github.com/Ra1issa/signal-cli) you will find a makefile that will allow you to clean, build and run the application.
<ol>
  <li> You will need to install the dependencies of each as specified in their original repositories [AsamK/signal-cli](https://github.com/AsamK/signal-cli) and [signalapp/libsignal-client](https://github.com/signalapp/libsignal-client) respectively.</li>
  <li> Clean up the repository first via <code>make clean</code></li>
  <li> Build all the repositories via <code>make build_all</code>. You can alternatively build each of Hecate, libgsignal-client or signal-cli individually.</li>
  <li> Modify the makefile with your desired phone numbers and messages of choice</li>
  <li> Register your desired phone number (or numbers if you would like try out both a sender and a receiver) in signal-cli by following the steps in [AsamK/signal-cli](https://github.com/AsamK/signal-cli). You will most likely need to have a captcha.</li>
  <li> (Optional) If you are running the receiver using signal-cli: Run the receiver daemon using <code> make run_receiver_daemon</code> (after changing the makefile with your desired phone numbers)</li>
  <li>Send a message using the sender via <code> make run_send</code>. Note that you can alternatively run a daemon for the sender and use the dbus interface to send messages (see [AsamK/signal-cli/wiki](https://github.com/AsamK/signal-cli/wiki/DBus-service))</li>
</ol>

## Important Notes
<ol>
  <li> In order to enable Signal's sealed sender, the sender and the receiver will have to send a 2-3 messages to one another in order to trust one another's accounts.</li>
  <li> If the sender is not run via the daemon (especially in the case where the receiver is run locally via the daemon), you will need to regularly manually poll for message receipts by running receive on the sender's phone number. This is <em>extremely important</em>, do NOT skip this step. Without it, the Signal protocol and signal-cli will not work properly and you may end up having to wait a very long time for every single message receipt for every message ever sent when you eventually do.</li>
</ol>

## A Note on Security

This work is only a prototype and is not production level ready. Do not deploy it or trust it with sensitive data.

## Contact

If you have any questions, feel free to start a discussion on this repo and I will get back to you asap.
