use criterion::{black_box, criterion_group, criterion_main, Criterion};
use soroban_security_guard::{scan_contract, ScannerConfig};
use std::fs;

fn bench_scan_vulnerable_contract(c: &mut Criterion) {
    let vulnerable_code = r#"
#[contract]
pub struct VulnerableContract {
    admin: Address,
}

#[contractimpl]
impl VulnerableContract {
    pub fn set_admin(env: &Env, new_admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }
    
    pub unsafe fn add_numbers(a: u64, b: u64) -> u64 {
        a + b
    }
    
    pub fn withdraw(env: &Env, amount: u64, recipient: Address) {
        let balance = Self::get_balance(env, caller);
        env.storage().instance().set(&DataKey::Balance, &(balance - amount));
        Self::external_transfer(env, recipient, amount);
        env.storage().instance().set(&DataKey::Counter, &1);
    }
}
"#;

    c.bench_function("scan_vulnerable_contract", |b| {
        b.iter(|| {
            let config = ScannerConfig::default();
            let _report = scan_contract(black_box("vulnerable_contract.rs"), black_box(config)).unwrap();
        })
    });
}

fn bench_scan_safe_contract(c: &mut Criterion) {
    let safe_code = r#"
#[contract]
pub struct SafeContract {
    admin: Address,
}

#[contractimpl]
impl SafeContract {
    pub fn set_admin(env: &Env, new_admin: Address) {
        let current_admin = Self::get_admin(env);
        require!(current_admin == env.current_contract_address(), "Admin only");
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }
    
    pub fn add_numbers(a: u64, b: u64) -> Result<u64, &'static str> {
        a.checked_add(b).ok_or("Overflow detected")
    }
    
    pub fn withdraw(env: &Env, amount: u64, recipient: Address) -> Result<(), &'static str> {
        let balance = Self::get_balance(env, caller);
        require!(balance >= amount, "Insufficient balance");
        env.storage().instance().set(&DataKey::Balance, &(balance - amount));
        env.storage().instance().set(&DataKey::Counter, &1);
        Self::external_transfer(env, recipient, amount)?;
        Ok(())
    }
}
"#;

    c.bench_function("scan_safe_contract", |b| {
        b.iter(|| {
            let config = ScannerConfig::default();
            let _report = scan_contract(black_box("safe_contract.rs"), black_box(config)).unwrap();
        })
    });
}

fn bench_large_contract(c: &mut Criterion) {
    let mut large_contract = String::new();
    
    // Generate a large contract with many functions
    large_contract.push_str("#[contract]\npub struct LargeContract {}\n\n#[contractimpl]\nimpl LargeContract {\n");
    
    for i in 0..100 {
        large_contract.push_str(&format!(
            r#"
    pub fn function_{}(env: &Env, param: u64) -> u64 {{
        let result = param + {};
        env.storage().instance().set(&DataKey::Counter, &result);
        result
    }}
"#, i, i
        ));
    }
    
    large_contract.push_str("}\n");

    c.bench_function("scan_large_contract", |b| {
        b.iter(|| {
            let config = ScannerConfig::default();
            let _report = scan_contract(black_box("large_contract.rs"), black_box(config)).unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_scan_vulnerable_contract,
    bench_scan_safe_contract,
    bench_large_contract
);
criterion_main!(benches);
