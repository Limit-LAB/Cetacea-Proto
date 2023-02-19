# Cetacea Protocol
> TL;DR Cetacea protocol is **[Matrix](https://spec.matrix.org)** version of **[Nostr](https://github.com/nostr-protocol/nostr)** protocol


## What is Cetacea Protocol?
cetacea protocol is a protocol for decentralized communication. 
it uses **[Orkas](https://github.com/limit-lab/orkas)** to provide a decentralized distributed network for federated communication.

## What problem does Cetacea Protocol solve?
We have added these solvable things on top of all the problems solved by Nostr, such as

- Junk management
- Permissions Management
- Message Backup
- Real-time guaranteed
- Message reliability guaranteed
- Sequence guaranteed
- Message traceability guaranteed

# How does Cetacea Protocol solve the problem?

relay protocol is a stateless forwarding protocol, want to do the above points will need to sink some logic in the client, such as message backup, message traceability guarantee, etc., but it can not avoid a problem is the network-wide bearing strength, now Demus above a large number of messages are spam, these messages will be mindlessly forwarded to the user's device, a waste of forwarding nodes and user's device a lot of bandwidth.

We solve this problem by introducing a minimum state, such as message backup, which we can solve by introducing a state of message backup that can be controlled by the user themselves, for example, the user (hoser) can choose not to backup, or can choose to backup to a third party service (such as AWS S3), thus avoiding a network-wide bearer effort.

We also do identity verification by introducing the blockchain mechanism, so that the privacy of user information can be avoided and user information can also be prevented from being tampered with.



## How to use Cetacea Protocol?

this repository contains the specification of the Cetacea protocol.

and we are working on the standard implementation of cetacea protocol on **[Cetacea](https://github.com/limit-lab/cetacea)**

## How to contribute?

Please read the contribution guidelines

## Specification
this repository contains the specification of the Cetacea protocol.

it is written in Rust programming language, due to which we can simply see the specification in the code and produce the specification document from the code.

alternatively, this repository could be used as a library for cetacea protocol.
