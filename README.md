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

## How does Cetacea Protocol solve the problem?

relay protocol is a stateless forwarding protocol, which requires clients to implement some of the features, such as message backup, message traceability guarantee, etc. But one problem is the network-wide bearing strength. Now a large amount of messages on Demus are spam, which are forwarded to users' devices and discarded, or even worse, read by users.

We resolve this problem by introducing a minimal state controlled by the users themselves. For example, users can choose not to backup messages, or save to a thirdparty service (such as AWS S3), thus avoiding a network-wide bearer effort. What's more, we also implement identity verification with blockchain mechanism, so that the confidentiality and integrity of user information and privacy are guaranteed.

## How to use Cetacea Protocol?

this repository contains the specification of the Cetacea protocol. We are working on the standard implementation of cetacea protocol on **[Cetacea](https://github.com/limit-lab/cetacea)**

## How to contribute?

Please read the contribution guidelines

## Specification

This repository contains the specification of the Cetacea protocol. It is written in Rust programming language, due to which we can simply see the specification in the code and produce the document with rustdoc. Additionally, this repository can also be used as a library for cetacea protocol models.
