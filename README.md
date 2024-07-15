# NFT Minting and Swapping Program with Vault System

This project implements a comprehensive program using Anchor for minting a collection of NFTs, creating a vault system to lock NFTs with rental fees returned to the protocol, and a swap program for exchanging $SOL for NFTs. The program includes storage and retrieval of images with appropriate metadata.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
  - [Minting NFTs](#minting-nfts)
  - [Vault System](#vault-system)
  - [Image Storage and Retrieval](#image-storage-and-retrieval)
  - [Swap Program](#swap-program)
- [Additional Resources](#additional-resources)

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- Node.js (v14 or higher)
- NPM or Yarn

### Steps

1. **Clone the repository:**

   ```bash
   git clone https://github.com/Dunsin-cyber/nft_minting.git
   cd nft_minting
   ```

2. **Install dependencies:**

   ```
   npm install # or yarn install

   ```

3. **Build the Anchor program:**

   ```
   anchor build

   ```

4. **Deploy the Anchor program:**

   ```
   anchor deploy

   ```

5. **Start the local Solana cluster:**

   ```
   solana-test-validator

   ```

6. **Run the client script:**

   ```
    npm run start

   ```
