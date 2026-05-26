# MedDisburse Macabebe

Bypassing banking bureaucracy to deliver instant emergency medical stipends to frontline rescue volunteers during severe floods.

## Problem
Local barangay health workers in Macabebe endure critical delays procuring life-saving medical supplies because traditional municipal medical fund reimbursement audits take weeks to clear centralized banking channels.

## Solution
Local health boards secure emergency funds inside a Soroban smart contract. Authorized medical dispatchers trigger equal, low-cost USDC emergency stipends directly to the wallets of pre-verified healthcare volunteers, empowering them to buy immediate first-aid assets.

## Timeline
* **Sprint 1:** Smart Contract logic & unit testing
* **Sprint 2:** Vite/React frontend integration and Freighter wallet connection
* **Sprint 3:** Testnet deployment & end-to-end demo prep

## Stellar Features Used
* **Soroban Smart Contracts:** Secure, immutable execution of disbursement rules.
* **USDC on Stellar:** Stable, low-cost value transfer ensuring the $50 stipend holds its purchasing power.

## Vision and Purpose
To create a scalable, composable disaster-relief template that municipalities across the Philippines and SEA can fork to ensure their emergency responders are never handicapped by banking red tape when lives are on the line.

## Prerequisites
* Rust toolchain (`rustup target add wasm32-unknown-unknown`)
* Soroban CLI (`cargo install --locked soroban-cli`)

## How to Build
Compile the smart contract to WebAssembly:
```bash
soroban contract build

## Stellar Expert Link
https://stellar.expert/explorer/testnet/contract/CA4AH4KHUJXWZBBKWDDYXNYCGSQXGWN4GPQCL2JWHWOUJZQQ57S4W24M

## Contact ID
CA4AH4KHUJXWZBBKWDDYXNYCGSQXGWN4GPQCL2JWHWOUJZQQ57S4W24M

## Screenshot
![alt text](<Screenshot 2026-05-26 162738.png>)