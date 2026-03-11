#![no_std]
#![no_main]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
pub enum DataKey {
    Admin,
    Counter,
    Balances(Address),
}

#[contract]
pub struct VulnerableContract {
    // Missing access control on admin
    admin: Address,
}

#[contractimpl]
impl VulnerableContract {
    pub fn __init(env: &Env, admin: Address) {
        // No access control - anyone can set admin
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    // VULNERABILITY: Missing access control
    // This function should be admin-only but has no protection
    pub fn set_admin(env: &Env, new_admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }

    // VULNERABILITY: Missing access control
    // Owner function without access check
    pub fn transfer_ownership(env: &Env, new_owner: Address) {
        let current_admin = Self::get_admin(env);
        env.storage().instance().set(&DataKey::Admin, &new_owner);
    }

    // VULNERABILITY: Potential integer overflow
    pub unsafe fn increment_counter(env: &Env, amount: u64) -> u64 {
        let mut counter: u64 = env.storage().instance().get(&DataKey::Counter).unwrap_or(0);
        counter += amount; // No overflow check
        env.storage().instance().set(&DataKey::Counter, &counter);
        counter
    }

    // VULNERABILITY: Potential reentrancy
    pub fn withdraw(env: &Env, amount: u64, recipient: Address) {
        let caller = env.current_contract_address();
        let balance = Self::get_balance(env, caller);
        
        require!(balance >= amount, "Insufficient balance");
        
        // State change should happen before external call
        env.storage().instance().set(&DataKey::Balances(caller), &(balance - amount));
        
        // External call after state change - potential reentrancy
        Self._transfer(env, recipient, amount);
        
        // State change after external call - VULNERABLE
        env.storage().instance().set(&DataKey::Counter, &(amount + 1));
    }

    // VULNERABILITY: Unsafe approve pattern
    pub fn approve(env: &Env, spender: Address, amount: u64) {
        let caller = env.current_contract_address();
        // No check for existing allowance - can lead to race conditions
        env.storage().instance().set(&DataKey::Balances(spender), &amount);
    }

    // VULNERABILITY: Hardcoded address
    pub fn emergency_withdraw(env: &Env) {
        let hardcoded_address = Address::from_string(&"GD5JDQ..."); // Hardcoded!
        let amount = Self::get_balance(env, hardcoded_address);
        Self::_transfer(env, hardcoded_address, amount);
    }

    // VULNERABILITY: Debug statement in production
    pub fn debug_function(env: &Env, data: &str) {
        println!("Debug data: {}", data); // Debug statement
        env.storage().instance().set(&Symbol::new(env, "debug"), data);
    }

    // Helper functions
    fn get_admin(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    fn get_balance(env: &Env, addr: Address) -> u64 {
        env.storage().instance().get(&DataKey::Balances(addr)).unwrap_or(0)
    }

    fn _transfer(env: &Env, recipient: Address, amount: u64) {
        // This would normally be an external call
        // For demonstration, we'll just update storage
        let current_balance = Self::get_balance(env, recipient);
        env.storage().instance().set(&DataKey::Balances(recipient), &(current_balance + amount));
    }
}

// Helper macro (missing proper implementation)
macro_rules! require {
    ($condition:expr, $message:expr) => {
        if !$condition {
            panic!($message);
        }
    };
}
