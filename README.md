# 🎓 Campus Reward Marketplace

## Project Title

Campus Reward Marketplace

---

## A Quick Note

This is a beginner's blockchain project created as part of my learning journey with Stellar Soroban.

I hope the reviewers will be a little gentle with this junior developer. 😄

Thank you for providing this opportunity to learn blockchain development and smart contract programming through this workshop.

---

## Project Description

Campus Reward Marketplace is a decentralized reward management platform built on the Stellar blockchain using Soroban smart contracts.

The platform allows students to earn points by participating in campus activities, workshops, volunteer programs, and events. These points can later be redeemed for rewards such as vouchers, merchandise, certificates, or event tickets.

Unlike traditional reward systems that rely on centralized databases, all student balances, rewards, and redemption actions are managed on-chain, ensuring transparency, security, and accountability.

---
## Project Preview

<img width="1906" height="910" alt="Screenshot 2026-06-17 091829" src="https://github.com/user-attachments/assets/7b8d3d84-3534-4708-bd59-f060c9b1e0ee" />

---

## Project Vision

The vision of Campus Reward Marketplace is to provide educational institutions and student organizations with a transparent and trustworthy reward ecosystem powered by blockchain technology.

By leveraging Stellar Soroban smart contracts, the platform enables secure point management, verifiable reward redemption, and decentralized record keeping without relying on centralized intermediaries.

---

## Why Blockchain?

Traditional reward systems often rely on centralized databases where administrators can modify records without transparency.

This project uses blockchain to provide:

* Transparent reward management
* Tamper-resistant records
* Wallet-based student identity
* Verifiable reward redemption
* Decentralized storage of reward data

Student actions and balances are secured through Stellar wallet authentication and Soroban smart contract authorization.

---

## Key Features

### Student Registration

* Register students using Stellar wallet addresses
* Store student information on-chain
* Create decentralized student identities

### Point Management

* Award participation points
* View point balances
* Maintain immutable reward records

### Reward Marketplace

* Create reward items
* Store reward catalog on-chain
* Define redemption costs

### Reward Redemption

* Redeem rewards using accumulated points
* Automatic point deduction
* Validation of sufficient balances

### Authorization System

* Admin-only reward creation
* Admin-only point distribution
* Student-authorized reward redemption
* Wallet-based authentication using Soroban Address authorization

---

## Smart Contract Functions

### initialize

Initializes the smart contract and assigns an administrator.

### register_student

Allows students to register themselves using their wallet address.

### get_student

Returns student information.

### get_points

Returns the current point balance.

### add_points

Allows the administrator to award points to students.

### create_reward

Allows the administrator to create reward offerings.

### get_reward

Returns reward information.

### redeem_reward

Allows students to redeem rewards using their points.

---

## Example Workflow

### Step 1

Administrator initializes the contract.

↓

### Step 2

Student registers with their Stellar wallet.

↓

### Step 3

Administrator awards participation points.

Example:

100 Points

↓

### Step 4

Administrator creates a reward.

Example:

Coffee Voucher

Cost: 50 Points

↓

### Step 5

Student redeems the reward.

↓

### Step 6

Point balance is automatically updated on-chain.

---

## Technology Stack

* Rust
* Soroban SDK
* Stellar Blockchain
* Soroban Smart Contracts
* Persistent On-Chain Storage
* Wallet-Based Authentication

---

## Future Scope

* NFT achievement badges
* Event attendance verification
* QR-code reward redemption
* Reward redemption history
* Multi-club support
* Student leaderboards
* Mobile application integration
* Tokenized reward points
* University-wide reward ecosystem

---

## Project Screenshot

Insert your deployment or contract screenshot here.

![Project Screenshot](images/project-preview.png)

---

## Contract Detail

Contract ID:

CDFN65REUPGJDO5QIIADAUCI6L6BTBNG2DD4E7XN2B3K6BPP5HB4TGAR

Network:

Stellar Testnet

---

## Educational Purpose

This project was developed for educational purposes as part of a Stellar Soroban workshop.

The goal is to demonstrate practical applications of blockchain technology in student engagement and reward management while exploring smart contract development using Rust and Soroban.

---

## License

This project is licensed under the MIT License.
