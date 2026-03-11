use soroban_security_guard::{scan_contract, ScannerConfig, Severity};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_scan_vulnerable_contract() {
    let vulnerable_code = r#"
#[contract]
pub struct VulnerableContract {
    admin: Address,
}

#[contractimpl]
impl VulnerableContract {
    pub fn set_admin(env: &Env, new_admin: Address) {
        // No access control - vulnerable!
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }
}
"#;

    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("vulnerable.rs");
    fs::write(&file_path, vulnerable_code).unwrap();

    let config = ScannerConfig::default();
    let report = scan_contract(file_path.to_str().unwrap(), config).unwrap();

    assert!(!report.issues.is_empty());
    
    // Should detect missing access control
    let admin_issues: Vec<_> = report.issues.iter()
        .filter(|issue| issue.rule_name.contains("admin"))
        .collect();
    assert!(!admin_issues.is_empty());
}

#[test]
fn test_scan_safe_contract() {
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
}
"#;

    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("safe.rs");
    fs::write(&file_path, safe_code).unwrap();

    let config = ScannerConfig::default();
    let report = scan_contract(file_path.to_str().unwrap(), config).unwrap();

    // Should have fewer or no issues compared to vulnerable contract
    let admin_issues: Vec<_> = report.issues.iter()
        .filter(|issue| issue.rule_name.contains("admin"))
        .collect();
    assert_eq!(admin_issues.len(), 0);
}

#[test]
fn test_custom_rules() {
    let code_with_hardcoded_address = r#"
pub fn transfer(env: &Env) {
    let hardcoded = Address::from_string(&"GD5JDQ...");
    env.storage().instance().set(&Key::Admin, &hardcoded);
}
"#;

    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("hardcoded.rs");
    fs::write(&file_path, code_with_hardcoded_address).unwrap();

    let mut config = ScannerConfig::default();
    config.rules.custom_rules.push(soroban_security_guard::config::CustomRule {
        name: "hardcoded_address".to_string(),
        pattern: r#"0x[a-fA-F0-9]{40}"#.to_string(),
        severity: Severity::Medium,
        description: "Potential hardcoded address detected".to_string(),
        enabled: true,
    });

    let report = scan_contract(file_path.to_str().unwrap(), config).unwrap();
    
    let custom_rule_issues: Vec<_> = report.issues.iter()
        .filter(|issue| issue.rule_name == "hardcoded_address")
        .collect();
    assert!(!custom_rule_issues.is_empty());
}

#[test]
fn test_severity_filtering() {
    let code_with_multiple_issues = r#"
#[contract]
pub struct Contract {
    admin: Address,
}

#[contractimpl]
impl Contract {
    pub fn set_admin(env: &Env, new_admin: Address) {
        // High severity: missing access control
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }
    
    pub fn debug_function(env: &Env, data: &str) {
        // Low severity: debug statement
        println!("Debug: {}", data);
    }
}
"#;

    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("multi_issues.rs");
    fs::write(&file_path, code_with_multiple_issues).unwrap();

    // Test with high severity threshold
    let mut config = ScannerConfig::default();
    config.severity_threshold = Severity::High;
    
    let report = scan_contract(file_path.to_str().unwrap(), config).unwrap();
    
    // Should only include high severity issues
    let low_severity_issues: Vec<_> = report.issues.iter()
        .filter(|issue| issue.severity == Severity::Low)
        .collect();
    assert_eq!(low_severity_issues.len(), 0);
}

#[test]
fn test_file_filtering() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a contract file
    let contract_code = r#"
#[contract]
pub struct TestContract {}
"#;
    let contract_file = temp_dir.path().join("contract.rs");
    fs::write(&contract_file, contract_code).unwrap();

    // Create a test file
    let test_code = r#"
#[test]
fn test_something() {
    assert_eq!(1, 1);
}
"#;
    let test_file = temp_dir.path().join("test_contract.rs");
    fs::write(&test_file, test_code).unwrap();

    let mut config = ScannerConfig::default();
    config.exclude_patterns.push("test_*.rs".to_string());

    let report = scan_contract(temp_dir.path().to_str().unwrap(), config).unwrap();
    
    // Should only scan the contract file, not the test file
    assert_eq!(report.scanned_files.len(), 1);
    assert!(report.scanned_files[0].path.contains("contract.rs"));
}

#[test]
fn test_arithmetic_overflow_detection() {
    let overflow_code = r#"
#[contractimpl]
impl Contract {
    pub unsafe fn add_numbers(a: u64, b: u64) -> u64 {
        a + b // No overflow check
    }
}
"#;

    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("overflow.rs");
    fs::write(&file_path, overflow_code).unwrap();

    let config = ScannerConfig::default();
    let report = scan_contract(file_path.to_str().unwrap(), config).unwrap();

    let overflow_issues: Vec<_> = report.issues.iter()
        .filter(|issue| issue.rule_name.contains("overflow"))
        .collect();
    assert!(!overflow_issues.is_empty());
}

#[test]
fn test_reentrancy_detection() {
    let reentrancy_code = r#"
#[contractimpl]
impl Contract {
    pub fn withdraw(env: &Env, amount: u64, recipient: Address) {
        let balance = Self::get_balance(env, env.current_contract_address());
        env.storage().instance().set(&DataKey::Balance, &(balance - amount));
        
        // External call
        Self::external_transfer(env, recipient, amount);
        
        // State change after external call - vulnerable!
        env.storage().instance().set(&DataKey::Counter, &1);
    }
}
"#;

    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("reentrancy.rs");
    fs::write(&file_path, reentrancy_code).unwrap();

    let config = ScannerConfig::default();
    let report = scan_contract(file_path.to_str().unwrap(), config).unwrap();

    let reentrancy_issues: Vec<_> = report.issues.iter()
        .filter(|issue| issue.rule_name.contains("reentrancy"))
        .collect();
    assert!(!reentrancy_issues.is_empty());
}
