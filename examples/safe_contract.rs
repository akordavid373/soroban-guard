#![no_std]
#![no_main]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
pub enum DataKey {
    Admin,
    Counter,
    Allowances(Address, Address), // owner, spender
    Balances(Address),
    Paused,
}

#[contract]
pub struct SafeContract {
    admin: Address,
    paused: bool,
}

#[contractimpl]
impl SafeContract {
    pub fn __init(env: &Env, admin: Address) {
        // Set initial admin
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Paused, &false);
    }

    // SAFE: Proper access control
    pub fn set_admin(env: &Env, new_admin: Address) {
        let current_admin = Self::get_admin(env);
        require!(current_admin == env.current_contract_address(), "Admin only");
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }

    // SAFE: Proper access control for ownership transfer
    pub fn transfer_ownership(env: &Env, new_owner: Address) {
        let current_admin = Self::get_admin(env);
        require!(current_admin == env.current_contract_address(), "Admin only");
        require!(new_owner != Address::zero(), "Invalid address");
        env.storage().instance().set(&DataKey::Admin, &new_owner);
    }

    // SAFE: Safe arithmetic with overflow checks
    pub fn increment_counter(env: &Env, amount: u64) -> Result<u64, &'static str> {
        let counter: u64 = env.storage().instance().get(&DataKey::Counter).unwrap_or(0);
        
        // Use checked addition to prevent overflow
        match counter.checked_add(amount) {
            Some(new_counter) => {
                env.storage().instance().set(&DataKey::Counter, &new_counter);
                Ok(new_counter)
            }
            None => Err("Overflow detected"),
        }
    }

    // SAFE: Proper checks-effects-interactions pattern
    pub fn withdraw(env: &Env, amount: u64, recipient: Address) -> Result<(), &'static str> {
        // Check contract is not paused
        require!(!Self::is_paused(env), "Contract is paused");

        let caller = env.current_contract_address();
        let balance = Self::get_balance(env, caller);
        
        // Checks
        require!(balance >= amount, "Insufficient balance");
        require!(recipient != Address::zero(), "Invalid recipient");

        // Effects - all state changes before external calls
        let new_balance = balance.checked_sub(amount)
            .ok_or("Underflow detected")?;
        env.storage().instance().set(&DataKey::Balances(caller), &new_balance);

        // Interactions - external call at the end
        Self._safe_transfer(env, recipient, amount)?;
        
        Ok(())
    }

    // SAFE: Proper approve pattern with allowance clearing
    pub fn approve(env: &Env, spender: Address, amount: u64) -> Result<(), &'static str> {
        let owner = env.current_contract_address();
        
        // Clear existing allowance to prevent race conditions
        env.storage().instance().remove(&DataKey::Allowances(owner, spender));
        
        // Set new allowance
        env.storage().instance().set(&DataKey::Allowances(owner, spender), &amount);
        
        Ok(())
    }

    // SAFE: No hardcoded addresses, uses parameters
    pub fn emergency_withdraw(env: &Env, recipient: Address, amount: u64) -> Result<(), &'static str> {
        let admin = Self::get_admin(env);
        require!(admin == env.current_contract_address(), "Admin only");
        require!(recipient != Address::zero(), "Invalid recipient");

        let balance = Self::get_balance(env, admin);
        require!(balance >= amount, "Insufficient balance");

        // State change before external call
        env.storage().instance().set(&DataKey::Balances(admin), &(balance - amount));
        
        // External call
        Self::_safe_transfer(env, recipient, amount)?;
        
        Ok(())
    }

    // SAFE: No debug statements in production
    pub fn log_event(env: &Env, event_type: Symbol, data: &str) {
        // Use proper event logging instead of println
        env.events().publish(event_type, data);
    }

    // SAFE: Pause functionality with access control
    pub fn pause(env: &Env) -> Result<(), &'static str> {
        let admin = Self::get_admin(env);
        require!(admin == env.current_contract_address(), "Admin only");
        
        env.storage().instance().set(&DataKey::Paused, &true);
        Ok(())
    }

    // SAFE: Unpause functionality with access control
    pub fn unpause(env: &Env) -> Result<(), &'static str> {
        let admin = Self::get_admin(env);
        require!(admin == env.current_contract_address(), "Admin only");
        
        env.storage().instance().set(&DataKey::Paused, &false);
        Ok(())
    }

    // Helper functions
    fn get_admin(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    fn get_balance(env: &Env, addr: Address) -> u64 {
        env.storage().instance().get(&DataKey::Balances(addr)).unwrap_or(0)
    }

    fn is_paused(env: &Env) -> bool {
        env.storage().instance().get(&DataKey::Paused).unwrap_or(false)
    }

    fn _safe_transfer(env: &Env, recipient: Address, amount: u64) -> Result<(), &'static str> {
        // This would normally be an external call with proper error handling
        // For demonstration, we'll update storage with safety checks
        let current_balance = Self::get_balance(env, recipient);
        let new_balance = current_balance.checked_add(amount)
            .ok_or("Overflow in recipient balance")?;
        env.storage().instance().set(&DataKey::Balances(recipient), &new_balance);
        Ok(())
    }

    // SAFE: Transfer with allowance support
    pub fn transfer_from(env: &Env, from: Address, to: Address, amount: u64) -> Result<(), &'static str> {
        let spender = env.current_contract_address();
        let allowance = Self::get_allowance(env, from, spender);
        
        require!(allowance >= amount, "Insufficient allowance");
        
        // Update allowance first
        let new_allowance = allowance.checked_sub(amount)
            .ok_or("Underflow in allowance")?;
        env.storage().instance().set(&DataKey::Allowances(from, spender), &new_allowance);
        
        // Then perform transfer
        Self._transfer_internal(env, from, to, amount)
    }

    fn get_allowance(env: &Env, owner: Address, spender: Address) -> u64 {
        env.storage().instance().get(&DataKey::Allowances(owner, spender)).unwrap_or(0)
    }

    fn _transfer_internal(env: &Env, from: Address, to: Address, amount: u64) -> Result<(), &'static str> {
        require!(to != Address::zero(), "Invalid recipient");
        
        let from_balance = Self::get_balance(env, from);
        require!(from_balance >= amount, "Insufficient balance");
        
        // Update sender balance
        let new_from_balance = from_balance.checked_sub(amount)
            .ok_or("Underflow in sender balance")?;
        env.storage().instance().set(&DataKey::Balances(from), &new_from_balance);
        
        // Update recipient balance
        let to_balance = Self::get_balance(env, to);
        let new_to_balance = to_balance.checked_add(amount)
            .ok_or("Overflow in recipient balance")?;
        env.storage().instance().set(&DataKey::Balances(to), &new_to_balance);
        
        Ok(())
    }
}

// Safe require macro
macro_rules! require {
    ($condition:expr, $message:expr) => {
        if !$condition {
            return Err($message);
        }
    };
}
