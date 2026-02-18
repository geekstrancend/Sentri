# Writing Invariants

## DSL Syntax Reference

### Basic Structure

```invar
invariant: identifier
description: "Human-readable description"
[category: category_name]
[tags: [tag1, tag2]]

[context { ... }]     // Optional context declaration

[forall <var> in <collection>:  // Optional loop constraint
    <condition>
]

[global:               // Optional global constraint (at least one required)
    <condition>
]
```

### Identifiers

Invariant names must be:
- Valid Rust identifier characters: `[a-zA-Z_][a-zA-Z0-9_]*`
- Unique within your project
- Descriptive

**Examples:**
```invar
invariant: vault_conservation
invariant: token_mint_safety
invariant: governance_quorum
```

### Expressions

Supported operators:
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `&&`, `||`, `!`
- Aggregation: `sum()`, `count()`, `max()`, `min()`, `avg()`

**Examples:**
```invar
// Arithmetic
balance + interest > 0

// Aggregation
sum(account_balances) == total_supply

// Complex
(balance > 0 && verified) || admin
```

### Types

All values must have compatible types:
- `u8`, `u16`, `u32`, `u64` - Unsigned integers
- `i8`, `i16`, `i32`, `i64` - Signed integers
- `String` - Text
- `bool` - Boolean
- `Address` - Chain-specific address
- `Bytes` - Raw bytes
- Custom types from your contract

**Type annotations (optional):**
```invar
forall amount: u64 in deposits:
    amount > 0

global:
    (total: u128) == sum(balances)
```

## Common Patterns

### 1. Conservation Law

Check that total is preserved:

```invar
invariant: balance_conservation
description: "Total balance across accounts must equal pool total"

global:
    sum(accounts[*].balance) == pool.total_balance
```

### 2. Monotonic Property

Check that values only increase/decrease:

```invar
invariant: vault_monotonic_increase
description: "Vault balance never decreases"

forall withdrawal in recent_withdrawals:
    withdrawal.before_balance >= withdrawal.after_balance
```

### 3. Bounds Checking

Check values stay within bounds:

```invar
invariant: bounded_percentage
description: "APY percentage must be (0, 100%)"

global:
    apy >= 0 && apy <= 100
```

### 4. Uniqueness

Check for no duplicates:

```invar
invariant: unique_accounts
description: "No duplicate account registrations"

global:
    count(unique_owners()) == count(all_accounts)
```

### 5. Dependency

Check relationships between values:

```invar
invariant: deposit_covers_withdrawal
description: "Can't withdraw more than deposited"

forall withdrawal in pending_withdrawals:
    withdrawal.amount <= account.total_deposits
```

## Advanced Features

### Context Declaration

Specify what state you're analyzing:

```invar
invariant: solana_specific
description: "Only for Solana"

context {
    state: ProgramState,
    chain: Solana,
    accounts: [user, vault, mint],
    epochs: 100
}

global:
    state.lamports >= 1000000  // 0.001 SOL
```

### Multiple Conditions

Combine with logical operators:

```invar
invariant: complex_rules
description: "Multiple rules must all hold"

global:
    (balance > 0 && verified) ||
    (balance > 1000 && pending_verification) &&
    !admin_disabled
```

### Collection Access

Access specific elements:

```invar
invariant: vault_accounts
description: "Check all vault accounts"

forall account in vault.accounts:
    account.balance >= MIN_BALANCE &&
    account.owner != address(0)
```

## Anti-Patterns

### Anti-Pattern: Too Broad

```invar
invariant: vague_rule
global:
    value > 0
```

**Problem:** What is `value`? Where does it come from? Too vague.

**Fix:**
```invar
invariant: positive_balance
description: "Account balance must be positive"
global:
    account.balance > 0
```

### Anti-Pattern: Missing Context

```invar
invariant: state_assumption
global:
    data[0] == expected  // What is data? What is expected?
```

**Problem:** Unclear where data comes from.

**Fix:**
```invar
invariant: initial_state_correct
description: "First item in vault matches initial deposit"

forall deposit in vault.deposits:
    if deposit.id == 0:
        deposit.amount == initial_deposit_amount
```

### Anti-Pattern: Contradictory Rules

```invar
invariant: impossible
global:
    balance == 100 && balance > 200  // Can never be true!
```

**Problem:** Invariant is unsatisfiable.

**Fix:** Review your logic and reason about what should actually be true.

### Anti-Pattern: State-Dependent

```invar
invariant: bad_assumption
global:
    latest_block.timestamp == last_update_time  // Depends on execution order
```

**Problem:** May not hold depending on block timing.

**Fix:** Use deterministic relationships.

## Testing Your Invariants

### Inline Testing

```invar
invariant: math_basic
description: "Basic arithmetic is correct"

global:
    2 + 2 == 4 &&      // Always true
    100 > 50 &&        // Always true
    "a" == "a"         // Always true
```

### With Real State

```bash
invar check vault.invar --state vault_state.json
```

### Checking for Violations

```invar
invariant: vault_conservation
description: "Should catch imbalance"

global:
    sum(deposits) == vault_balance  // Should fail if we inject bad state
```

## Style Guide

### Naming

- **Invariants**: `snake_case`
- **Variables in forall**: Single lowercase letter preferred `x`, `item`
- **Categories**: Domain-specific `finance`, `security`, `governance`

**Good:**
```invar
invariant: vault_conservation
forall deposit in deposits:
    deposit.amount > 0
```

**Bad:**
```invar
invariant: VaultConservation
forall MyDeposit in Deposits:
    MyDeposit.Amount > 0
```

### Organization

Group related invariants:

```invar
// ============ CONSERVATION LAWS ============

invariant: balance_conservation
// ...

invariant: supply_conservation
// ...

// ============ SECURITY PROPERTIES ============

invariant: access_control
// ...

invariant: mint_authorization
// ...
```

### Documentation

Always include descriptions:

```invar
invariant: vault_monotonic_increase
description: "Vault balance can only increase (deposits) or decrease (withdrawals), never jump"
category: security

global:
    // Each state transition follows valid rules
    all_transitions_valid()
```

## Common Mistakes

### Mistake 1: Forgetting forall iterator

```invar
BAD: forall deposit:  // What collection?
    deposit.amount > 0

GOOD: forall deposit in deposits:
    deposit.amount > 0
```

### Mistake 2: Type mismatch

```invar
BAD: global:
    "string" > 100  // Can't compare string to integer

GOOD: global:
    "100".to_number() > 50
```

### Mistake 3: Undefined variables

```invar
BAD: invariant: bad_var
global:
    undefined_variable > 0  // Where is undefined_variable?

GOOD: invariant: good_var
global:
    account.balance > 0  // Clear source
```

## Troubleshooting

### "Invariant never fails"

Your invariant might be:
- Too weak (always true)
- Logically impossible
- Missing important constraints

### "Invariant always fails"

Your invariant might be:
- Too strong
- Counting on state that doesn't exist
- Misunderstanding the data model

### "Type error in invariant"

Check:
- Variable types match expressions
- Operators work with your types
- String/number confusion

## Next Steps

- [Get Started](getting-started.md) - Set up your project
- [Example Invariants](example-invariants.md) - Real-world patterns
- [Architecture Overview](architecture-overview.md) - Understand how Invar works
