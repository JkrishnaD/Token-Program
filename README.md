# Custom Token Program (Solana + Anchor)

## 📌 Overview
This program is a **custom token implementation** on Solana built with [Anchor](https://www.anchor-lang.com/).  
It provides token functionality similar to the SPL Token Program but with **customizable business logic** such as account freezing, custom delegation rules, and controlled burning.

---

## ✨ Features
- **Minting** – Create and distribute tokens from a mint account.  
- **Transfers** – Send tokens between accounts with ownership and freeze checks.  
- **Delegation** – Approve a delegate to spend tokens on your behalf.  
- **Revoke Delegate** – Remove a delegate’s spending authority.  
- **Freezing** – Temporarily lock a token account so no transfers can occur.  
- **Burning** – Permanently destroy tokens, reducing the total supply.  
- **Closing Accounts** – Burn remaining tokens and reclaim rent before closing.

---

## 📂 Account Types

### 1. MintAccount
Stores metadata about the token mint:
| Field | Description |
|-------|-------------|
| `supply` | Total circulating supply |
| `mint_authority` | Entity allowed to mint new tokens |
| `decimals` | Precision of the token |
| `is_initalized` | Flag which shows account is created or not|
| `freeze_authority` | Entity allowed to freeze the minting|
| `bump` | PDA bump seed |


### 2. TokenAccount
Represents a user’s balance of a given token:
| Field | Description |
|-------|-------------|
| `owner` | Wallet controlling this account |
| `mint` | Mint this account is associated with |
| `amount` | Number of tokens held |
| `delegate` | Optional delegate authority |
| `delegate_amount` | Allowance for the delegate |
| `is_initialized` | Whether the account is Initialized |
| `is_frozen` | Whether the account is frozen |
| `bump` | PDA bump seed |

---

## 🔹 Instructions

| Instruction | Purpose | Key Checks |
|-------------|---------|------------|
| **Mint** | Mint tokens to a token account | Mint authority check, account initialized |
| **Transfer** | Transfer tokens between accounts | Ownership, balance, frozen checks |
| **Delegate** | Approve another account to spend tokens | Balance check, frozen check |
| **Revoke Delegate** | Remove delegate | Frozen check |
| **Freeze** | Lock account activity | Ownership check |
| **Burn** | Destroy tokens and reduce supply | Ownership, balance, frozen checks |
| **Close Account** | Burn remaining tokens and reclaim rent | Frozen check, automatic burn if balance > 0 |

---

## ⚙️ Build & Deploy

```bash
# Install dependencies
anchor build

# Deploy to local validator
anchor deploy

# Run tests
anchor test
```