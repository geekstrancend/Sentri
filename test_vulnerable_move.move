module 0xCAFE::vulnerable_token {
    use std::signer;
    use std::error;

    const EINSUFFICIENT_BALANCE: u64 = 0;

    struct Token has key {
        amount: u64,
    }

    // VULNERABILITY 1: Resource leak - move_from without binding
    public fun destroy_account(addr: address) acquires Token {
        let token = move_from<Token>(addr);
        // Resource dropped without handling - leaked!
        _ = token;
    }

    // VULNERABILITY 2: Missing ability checks on move
    public fun transfer(from: &signer, to: address, amount: u64) acquires Token {
        let from_addr = signer::address_of(from);
        let token = borrow_global_mut<Token>(from_addr);
        
        // VULNERABILITY 4: Unchecked arithmetic - potential underflow
        token.amount = token.amount - amount;
        
        // VULNERABILITY 3: Move to without signer verification of recipient
        let new_token = Token { amount };
        move_to(from, new_token); // Should require recipient signer
    }

    // VULNERABILITY 5: Unguarded state mutation
    public fun unchecked_modify(addr: address) acquires Token {
        let token = borrow_global_mut<Token>(addr);
        token.amount = token.amount * 2; // No validation!
    }

    // VULNERABILITY 6: Privilege escalation
    public fun extract_address(signer_ref: &signer): address {
        let addr = signer::address_of(signer_ref);
        // Then use this address without proper verification
        addr
    }

    // VULNERABILITY 7: Unsafe abort
    public fun validate_amount(amount: u64) {
        if (amount == 0) {
            abort 100; // Error code without reason
        }
    }

    public fun initialize(account: &signer) {
        let token_store = Token { amount: 0 };
        move_to(account, token_store);
    }
}
