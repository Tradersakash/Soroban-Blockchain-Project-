# Fractional NFT Contract

## Overview

This contract implements functionality for managing fractional ownership of non-fungible tokens (NFTs). It allows users to buy and sell fractions of an NFT.

## Structures

### Fraction

- `owner`: Address of the owner of the fraction.
- `amount`: Amount of the fraction.

### NFT

- `id`: Symbol representing the NFT.
- `name`: Name of the NFT.
- `description`: Description of the NFT.

## FractionalNFTContract

This contract provides methods for initializing the contract, buying and selling fractions of an NFT, and getting details of the NFT.

### Methods

- `initialize(&self, e: Env, nft: NFT, total_fractions: u32)`: Initializes the contract with the given NFT and total number of fractions.
- `buy_fraction(&self, e: Env, buyer: Address, amount: u32) -> Result<(), String>`: Allows a user to buy fractions of the NFT.
- `sell_fraction(&self, e: Env, seller: Address, amount: u32) -> Result<(), String>`: Allows a user to sell fractions of the NFT.
- `get_nft(&self, e: Env) -> NFT`: Retrieves details of the NFT.

## Usage

To use this contract, you can deploy it to your smart contract platform and interact with it using the provided methods.

\
contract.sell_fraction(env, seller, amount_to_sell);

// Get details of the NFT
let nft_details = contract.get_nft(env);
