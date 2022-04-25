# Casper-NFT-Marketplace


## Browse
![](screenshots/v0.0.2.gif)


## Description

Built for the Ready Player Casper Hackathon. Currently, work in progress, an NFT Store to participate in the Casper Network.

**Please note this has not been audited or peer reviewed, and is not ready for production.**

### Design Summary

Using Next.js for marketplace users to interact with. A rust Market contract to interact with the Casper CEP47 protocol.

## Implemented

- Browse page with static data
- Connect to wallet
- Mint NFT
- View owned NFTs
- View all NFTs
- View NFT Detail including meta, image and price
- Partial market contract implementation - see below for remaining functionality required


## TODO

### Server - Nextjs

~~- Implement minting page~~
~~- Implement detail for buyer~~
~~- Implement detail for seller~~
~~- Implement list all nfts~~
~~- Implement list owned nfts~~
~~- Link front end with cep47 contract - Retrieve NFT info~~
~~- Link front end with cep47 contract - Mint NFT~~
- Link front end with cep47 contract - Burn NFT
- Link front end with market contract - List NFT for sale
- Link front end with market contract - Sell NFT
- Link front end with market contract - Buy NFT
- Link front end with market contract - Withdraw funds

### Market contract - CEP47 compatible

~~- Functionality - Changing ownership of nft~~
~~- Functionality - Creating market item for sale~~
~~- Functionality - Processing sale of market item~~
~~- Functionality - Funds from sale of market item transferred to seller~~
~~- Functionality - Adding payments for buy/sell~~
- Functionality - Cancel sale of market item
- Security - Ensuring no loopholes in logic
- Error Handling - Provide correct errors within Contract
- Tests around securing transactions

### Wishlist

- Create collections
- Market commission
- Seller commission
- Upload meta to online buckets instead of asking for url in mint page
- Transaction history on NFT detail page
- Functionality - Add quantity to nft
- Code improvement - Change market item to struct


## Instructions

### Deploying Contracts

```
cd casper-contracts-js-client
cp ..env.cep47.example .env.cep47
npm i

# Take note of Contract Hash and Contract Package Hash for adding to server .env.local
npm run e2e:cep47:install

Optional - for some mock nfts:
e2e:cep47:fixture
```


### Launching Server
```
cd server
cp .env.template .env.local
npm i
npm start dev
```

## Screenshots

## Detail Buy
![](screenshots/v0.0.2-detail-buy.png)

## Detail Sell
![](screenshots/v0.0.2-detail-sell.png)

## My NFTs
![](screenshots/v0.0.2-my-nfts.png)

## Create NFT
![](screenshots/v0.0.2-create.png)
