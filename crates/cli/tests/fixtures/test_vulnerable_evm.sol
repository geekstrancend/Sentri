// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// VULNERABILITY: Reentrancy
contract Vulnerable {
    mapping(address => uint256) public balances;
    
    function withdraw(uint256 amount) public {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        
        // VULNERABILITY 1: Reentrancy - state change after external call
        (bool success, ) = msg.sender.call{value: amount}("");
        require(success, "Transfer failed");
        
        // State changed AFTER external call - reentrancy vulnerability
        balances[msg.sender] -= amount;
    }
    
    // VULNERABILITY 2: Unchecked call return value
    function unsafeTransfer(address recipient) public {
        recipient.call{value: 1 ether}("");
        // No require - return value not checked!
    }
    
    // VULNERABILITY 3: Integer overflow (no SafeMath)
    function add(uint256 a, uint256 b) public pure returns (uint256) {
        return a + b; // Can overflow in pragma <0.8, but still risky
    }
    
    // VULNERABILITY 4: Block.timestamp dependency
    function mint(address to, uint256 amount) public {
        require(block.timestamp > 0, "Invalid time");
        balances[to] += amount;
    }
    
    // VULNERABILITY 5: Missing access control
    function transfer(address from, address to, uint256 amount) public {
        // No check if msg.sender is authorized
        balances[from] -= amount;
        balances[to] += amount;
    }
    
    // VULNERABILITY 6: Delegatecall without proper validation
    function delegateCall(address target, bytes memory data) public {
        target.delegatecall(data); // No validation of target
    }
}
