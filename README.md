# Stellar Live Auction

## Project Description

Stellar Live Auction is a real-time auction mini-dApp built on Stellar Testnet using Soroban smart contracts.

The project allows a seller to create an auction and allows users to place bids on an item. When a valid bid is placed, the smart contract updates the highest bid, stores the highest bidder, and emits an event that can be tracked through the contract history.

This project was built for Level 3 of the Stellar builder challenge, focusing on smart contract interaction, event updates, transaction status, testing, and complete documentation.

---

## Project Vision

The vision of Stellar Live Auction is to create a transparent and verifiable auction system where bidding activity can be tracked on-chain.

In a real-world version, this project can be used for:

- NFT auctions
- Digital collectible sales
- Community fundraising auctions
- Event item auctions
- Student builder demo projects
- Transparent bidding systems for Web3 communities

By using Stellar and Soroban, auction data such as the current highest bid and highest bidder can be stored and verified through smart contract interactions.

---

## Key Features

- Create a new auction
- Store auction item name and description
- Set a starting price
- Place a bid
- Reject bids that are lower than or equal to the current highest bid
- Track the current highest bid
- Track the current highest bidder
- Read full auction information from the contract
- Close the auction
- Emit contract events for auction creation, bidding, and closing
- View transaction history and event updates on Stellar Testnet
- Includes 4 passing smart contract tests

---
## Deployed Contract Details

Network: Stellar Testnet

Contract ID: CAPLREGDXMFVN4HBFB6TZQUGRZCQPPVH46MORKRN326ORLZKW35AGPCG

WASM Hash: b260620a...919c6ced

Created At: 2026-04-26 14:53:48 UTC

Transaction History:

- create_auction was called successfully
- place_bid was called successfully with amount 150
- get_highest_bid returned 150
- get_highest_bidder returned the bidder address
##Screenshot:
![screenshot](explorer.png)
![screenshot](test3.png)


## Future Scope

In the future, this project can be improved with:

- Frontend live auction page
- Wallet connection through Freighter
- Real-time event listener for new bids
- Countdown timer for auction deadline
- Multiple auction support
- NFT auction integration
- Real XLM or token transfer for bids
- Refund logic for outbid users
- Seller settlement after auction closes
- Bid history table
- Mobile responsive interface
- Live demo deployment on Vercel or Netlify

## Contract Functions

### `create_auction(seller, item_name, description, starting_price)`

Creates a new auction.

Parameters:

- `seller`: address of the auction creator
- `item_name`: name of the auction item
- `description`: description of the auction item
- `starting_price`: starting bid amount

Example:

```text
seller: GCSGI4ZRPWFZV3DZMHNRELFJXJ6YELBV3LDADK7AWUHSELBLPAU4UT42
item_name: Stellar Builder Badge
description: A live auction demo item on Stellar Testnet
starting_price: 100