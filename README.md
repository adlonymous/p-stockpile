# P-Stockpile (Stockpile v3)


## Overview

Stockpile is a decentralized funding engine for the open internet. Leveraging the speed of Solana, it
facilitates the creation of quadratic funding rounds at the speed of light. This particular version of
the program is a re-write of the [Stockpile v2 program](https://github.com/StockpileProtocol/stockpile-v2) created by Joey Meere, and is written in Native 
Rust using the Pinocchio library. 

## Getting Started

- Ensure you have the Solana CLI installed and configured.

    `solana --version`

- Clone this repository.

    `git clone https://github.com/adlonymous/p-stockpile.git`

- Build the program.

    `cargo build-bpf`

## Navigating the Codebase

To understand the codebase, check out documentation on: 

- [Solana](https://solana.com/docs)
- [Pinocchio](https://docs.rs/pinocchio)


## Progress

Just started working on this, you can follow along with the progress through this codebase or through
my [Twitter](https://twitter.com/adlonymous).

### To Do

- [x] Create Pool
- [x] Create Vault
- [ ] Join Pool
- [ ] Refresh
- [ ] Contribute with vote
- [ ] Accept Participant
