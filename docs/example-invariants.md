# Example Invariants

Real-world invariant patterns for smart contracts.

## Solana Examples

### Vault Conservation

A SPL token vault that maintains balance invariant:

```invar
invariant: spl_vault_conservation
description: "Total vault balance equals sum of deposits"
category: finance
chain: Solana

context {
    state: VaultState,
    protocol: SPL,
    accounts: [vault, user_accounts, mint]
}

// Conservation law
global:
    sum(user_accounts[*].balance) == vault.authority.balance

// Individual constraints
forall account in user_accounts:
    account.balance >= 0 &&
    account.owner != address(0) &&
    account.delegated <= account.balance
```

### Multisig Authorization

Ensure multisig transactions are authorized:

```invar
invariant: multisig_required_signers
description: "Transactions require minimum signatures"
category: security

forall transaction in pending_transactions:
    count_valid_signatures(transaction) >= MIN_REQUIRED &&
    transaction.signers_all_distinct() &&
    all_signers_are_authorized()
```

## EVM Examples

### Token Mint Safety

ERC-20 token minting maintains supply invariant:

```invar
invariant: erc20_total_supply_conservation
description: "Sum of balances equals total supply"
category: token
chain: EVM
contract: ERC20

global:
    sum(balances[*]) == totalSupply &&
    totalSupply <= MAX_UINT256 &&
    totalSupply >= 0

forall mint_event in MintEvents:
    mint_event._value > 0 &&
    mint_event._to != address(0) &&
    balances[mint_event._to] >= 0
```

### Liquidity Pool AMM

Constant product formula invariant:

```invar
invariant: constant_product_formula
description: "k = x*y remains constant after swaps"
category: defi
contract: UniswapV2Pair

context {
    state: PoolState,
    tokens: [token0, token1],
    fee: 0.003
}

global:
    // After any swap, k should be maintained or increased
    reserve0 * reserve1 >= last_reserve0 * last_reserve1
    
forall swap in recent_swaps:
    swap.amount_in > 0 &&
    swap.amount_out > 0 &&
    swap.from != swap.to
```

### Governance Proposal

Voting and execution constraints:

```invar
invariant: governance_proposal_validity
description: "Proposals meet voting requirements"
category: governance

forall proposal in state.proposals:
    proposal.votes_for + proposal.votes_against <= total_voting_power &&
    proposal.start_block < current_block &&
    if proposal.status == EXECUTED:
        proposal.votes_for > proposal.votes_against &&
        proposal.end_block < current_block
```

## Move Examples

### NFT Collection Tracking

Ensure NFT collection integrity:

```invar
invariant: nft_collection_unique_owners
description: "Each NFT has exactly one owner"
category: nft
chain: Move

global:
    count(unique_owners()) == count(all_nfts) &&
    sum(nft_counts_per_owner[*]) == total_nfts

forall nft in collection.nfts:
    nft.owner != address(0) &&
    nft.minted_at <= current_time &&
    nft.royalty_percentage >= 0 &&
    nft.royalty_percentage <= 100
```

## Cross-Chain Examples

### Token Bridge Conservation

Bridged tokens:

```invar
invariant: bridge_token_conservation
description: "Tokens bridged from chain A to B maintain quantity"
category: security

global:
    tokens_locked_on_chain_a == tokens_issued_on_chain_b

forall bridge_event in BridgeEvents:
    bridge_event.source_amount > 0 &&
    bridge_event.destination_amount == bridge_event.source_amount &&
    bridge_event.timestamp_valid() &&
    bridge_event.receiver != address(0)
```

## Advanced Patterns

### State Machine Validation

Ensure valid state transitions:

```invar
invariant: valid_state_machine
description: "State transitions follow rules"
category: security

forall state_change in state_changes:
    is_valid_transition(state_change.from_state, state_change.to_state) &&
    state_change.timestamp >= last_change_timestamp &&
    state_change.validator != address(0)
```

### Time-Locked Operations

Enforce time constraints:

```invar
invariant: time_lock_enforcement
description: "Time-locked operations respect minimum duration"
category: security

forall operation in pending_operations:
    if operation.is_time_locked:
        operation.execution_time >= operation.creation_time + MIN_LOCK_DURATION &&
        operation.execution_time <= operation.creation_time + MAX_LOCK_DURATION
```

### Escrow Guarantee

Held funds are protected:

```invar
invariant: escrow_funds_protected
description: "Escrowed funds cannot be removed until conditions met"
category: finance

global:
    escrowed_balance == sum(escrow_accounts[*].balance)

forall escrow in escrow_accounts:
    escrow.balance > 0 &&
    !conditions_met(escrow) ||
    can_be_released(escrow)
```

## Testing Invariants

### Simple Test Case

```invar
invariant: basic_arithmetic
description: "Test basic properties"

global:
    1 + 1 == 2 &&
    10 > 5 &&
    "test" != "fail"
```

### State Transition Test

```invar
invariant: deposit_withdrawal_balance
description: "Deposit then withdraw returns to original"

global:
    initial_balance + deposit_amount - withdrawal_amount == final_balance
```

### Edge Case Test

```invar
invariant: zero_amount_handling
description: "Zero amounts handled correctly"

forall transaction in transactions:
    if transaction.amount == 0:
        transaction.state == REJECTED ||
        transaction.state == SKIPPED
```

## Integrating with Your Project

### Solana Program

```rust
// in lib.rs
#[program]
pub mod my_vault {
    // Your program code
    
    // Run invariant check in tests
    #[test]
    fn test_conservation_holds() {
        // Setup program state
        // Run instruction
        // Run: invar check invariants/vault.invar
    }
}
```

### EVM Contract

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract MyToken {
    // Your contract code
    
    // Invariants defined in: invariants/token.invar
}

// Run in test suite:
// invar check invariants/ --format json
```

### CI/CD Integration

```yaml
# .github/workflows/invariants.yml
name: Invariant Checks

on: [push, pull_request]

jobs:
  invariants:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install invar
      - run: invar check invariants/
```

## Validation Checklist

After writing invariants, verify:

- [ ] **Descriptive name** - What is being checked?
- [ ] **Clear description** - Why does this matter?
- [ ] **Correct syntax** - Does it parse?
- [ ] **Type safe** - Are all types compatible?
- [ ] **Not contradictory** - Can invariant be satisfied?
- [ ] **Testable** - Can you verify it with data?
- [ ] **Not too weak** - Does it catch actual bugs?
- [ ] **Not too strong** - Won't it be violated legitimately?

## Common Pitfalls

1. **Invariants that are always true** - No value as a check
2. **Invariants that are never true** - Indicates misunderstanding of data
3. **Missing state variables** - Can't reference what doesn't exist
4. **Confusing logic** - `&&` vs `||`, `>` vs `>=`
5. **Timelock inconsistencies** - Forgetting blockchain doesn't have continuous time

## More Examples

For additional examples by domain:
- **DeFi**: See Uniswap, Aave, Curve patterns
- **NFTs**: See OpenZeppelin, Magic Eden patterns
- **Governance**: See Aragon, Snapshot patterns
- **Bridges**: See Wormhole, Multichain patterns

See [Example Invariants](https://github.com/invar/invar/tree/main/examples) in repository.

## Next Steps

- [Writing Invariants Guide](writing-invariants.md) - DSL reference
- [Getting Started](getting-started.md) - Set up your project
- [CI/CD Integration](ci-integration.md) - Automated checking
