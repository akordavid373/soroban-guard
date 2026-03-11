---
title: Enhance AST parser for better Soroban-specific syntax support
labels: enhancement, parser, soroban, good-first-issue
assignees: []
---

## 🎯 Enhancement Description

The current AST parser provides basic Rust syntax parsing but has limited support for Soroban-specific syntax and patterns. This enhancement would improve vulnerability detection accuracy and reduce false positives by better understanding Soroban contract structure.

## 📁 Files to Modify

### Primary Files
```
📄 src/ast.rs (MAJOR REWRITE)
📄 src/rules.rs (update existing rules)
📄 tests/ast_tests.rs (CREATE NEW)
📄 examples/soroban_patterns.rs (CREATE NEW)
```

### Secondary Files
```
📄 src/lib.rs (update exports)
📄 src/scanner.rs (update to use enhanced parser)
📄 docs/ast-enhancement.md (CREATE NEW)
📄 benches/parser_performance.rs (CREATE NEW)
```

## 🎯 Acceptance Criteria

### ✅ MUST HAVE (High Priority)
- [ ] **src/ast.rs** - Add SorobanContract struct and parsing methods
- [ ] **src/ast.rs** - Implement contract struct recognition (`#[contract]`)
- [ ] **src/ast.rs** - Implement contract impl recognition (`#[contractimpl]`)
- [ ] **src/ast.rs** - Add storage pattern detection
- [ ] **tests/ast_tests.rs** - Comprehensive test coverage

### ✅ SHOULD HAVE (Medium Priority)
- [ ] **src/ast.rs** - Add event pattern detection
- [ ] **src/ast.rs** - Extend type system for Soroban types
- [ ] **src/rules.rs** - Update 3 existing rules to use enhanced parser
- [ ] **examples/soroban_patterns.rs** - Create test examples

### ✅ COULD HAVE (Low Priority)
- [ ] **benches/parser_performance.rs** - Performance benchmarks
- [ ] **docs/ast-enhancement.md** - Documentation for new features
- [ ] **src/lib.rs** - Export new parser features

## 🔧 Implementation Details

### 1. src/ast.rs - MAJOR ENHANCEMENT

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\src\ast.rs`

**Add these new structs after existing ones**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SorobanContract {
    pub name: String,
    pub contract_struct: Option<ContractStruct>,
    pub implementation: Option<ContractImpl>,
    pub storage_keys: Vec<StorageKey>,
    pub events: Vec<EventPattern>,
    pub soroban_types: Vec<SorobanTypeUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractStruct {
    pub name: String,
    pub fields: Vec<ContractField>,
    pub attributes: Vec<Attribute>,
    pub generics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractField {
    pub name: String,
    pub field_type: Type,
    pub visibility: Visibility,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractImpl {
    pub target: String,
    pub methods: Vec<ContractMethod>,
    pub attributes: Vec<Attribute>,
    pub generics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMethod {
    pub name: String,
    pub visibility: Visibility,
    pub is_mut: bool,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Vec<String>,
    pub line_start: usize,
    pub line_end: usize,
    pub attributes: Vec<Attribute>,
    pub is_contract_method: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageKey {
    pub name: String,
    pub key_type: String,
    pub usage_locations: Vec<CodeLocation>,
    pub storage_type: StorageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    Instance,
    Persistent,
    Temporary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPattern {
    pub symbol: String,
    pub data_types: Vec<Type>,
    pub locations: Vec<CodeLocation>,
    pub is_short_symbol: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SorobanTypeUsage {
    pub type_name: String,
    pub usage_locations: Vec<CodeLocation>,
    pub is_generic: bool,
    pub generic_params: Vec<String>,
}
```

**Add these methods to AstParser impl**:
```rust
impl AstParser {
    pub fn parse_soroban_contract(source: &str, file_path: &str) -> anyhow::Result<SorobanContract> {
        let mut contract = SorobanContract {
            name: Self::extract_contract_name(source, file_path)?,
            contract_struct: None,
            implementation: None,
            storage_keys: Vec::new(),
            events: Vec::new(),
            soroban_types: Vec::new(),
        };

        // Parse contract struct
        contract.contract_struct = Self::parse_contract_struct(source)?;

        // Parse contract implementation
        contract.implementation = Self::parse_contract_impl(source)?;

        // Parse storage patterns
        contract.storage_keys = Self::parse_storage_patterns(source)?;

        // Parse event patterns
        contract.events = Self::parse_event_patterns(source)?;

        // Parse Soroban type usage
        contract.soroban_types = Self::parse_soroban_types(source)?;

        Ok(contract)
    }

    fn parse_contract_struct(source: &str) -> anyhow::Result<Option<ContractStruct>> {
        let contract_regex = regex::Regex::new(
            r"#\[contract\]\s*(?P<attributes>#\[.*?\]\s*)*pub\s+struct\s+(?P<name>\w+)\s*(?P<generics<[^>]*>)?\s*\{(?P<body>.*?)\}"
        )?;

        if let Some(captures) = contract_regex.captures(source) {
            let name = captures["name"].to_string();
            let body = &captures["body"];
            let attributes = Self::parse_attributes(&captures["attributes"])?;

            let fields = Self::parse_contract_fields(body)?;
            let generics = Self::extract_generic_params(
                &captures.name("generics").map_or("", |m| m.as_str())
            );

            return Ok(Some(ContractStruct {
                name,
                fields,
                attributes,
                generics,
            }));
        }

        Ok(None)
    }

    fn parse_contract_impl(source: &str) -> anyhow::Result<Option<ContractImpl>> {
        let impl_regex = regex::Regex::new(
            r"#\[contractimpl\]\s*(?P<attributes>#\[.*?\]\s*)*impl\s+(?P<generics<[^>]*>)?\s*(?P<target>\w+)\s*\{(?P<body>.*?)\}"
        )?;

        if let Some(captures) = impl_regex.captures(source) {
            let target = captures["target"].to_string();
            let body = &captures["body"];
            let attributes = Self::parse_attributes(&captures["attributes"])?;

            let methods = Self::parse_contract_methods(body)?;
            let generics = Self::extract_generic_params(
                &captures.name("generics").map_or("", |m| m.as_str())
            );

            return Ok(Some(ContractImpl {
                target,
                methods,
                attributes,
                generics,
            }));
        }

        Ok(None)
    }

    fn parse_contract_fields(body: &str) -> anyhow::Result<Vec<ContractField>> {
        let mut fields = Vec::new();
        let field_regex = regex::Regex::new(
            r"(?P<attributes>#\[.*?\]\s*)*(?P<visibility>pub\s+)?(?P<name>\w+)\s*:\s*(?P<type>[^,]+)"
        )?;

        for caps in field_regex.captures_iter(body) {
            let name = caps["name"].to_string();
            let visibility = if caps["visibility"].starts_with("pub") {
                Visibility::Public
            } else {
                Visibility::Private
            };
            let field_type = Self::parse_type(&caps["type"].trim())?;
            let attributes = Self::parse_attributes(&caps["attributes"])?;

            fields.push(ContractField {
                name,
                field_type,
                visibility,
                attributes,
            });
        }

        Ok(fields)
    }

    fn parse_contract_methods(body: &str) -> anyhow::Result<Vec<ContractMethod>> {
        let mut methods = Vec::new();
        let method_regex = regex::Regex::new(
            r"(?s)(?P<attributes>#\[.*?\]\s*)*(?P<visibility>pub\s+)?fn\s+(?P<name>\w+)\s*\((?P<params>[^)]*)\)\s*(?P<return_type>->\s*[^{]+)?\s*\{(?P<body>.*?)\}"
        )?;

        for (line_num, caps) in method_regex.captures_iter(body).enumerate() {
            let name = caps["name"].to_string();
            let visibility = if caps["visibility"].starts_with("pub") {
                Visibility::Public
            } else {
                Visibility::Private
            };

            let parameters = Self::parse_parameters(&caps["params"])?;
            let return_type = if let Some(ret) = caps.name("return_type") {
                Some(Self::parse_type(&ret.as_str()[2..].trim())?)
            } else {
                None
            };

            let body_lines: Vec<String> = caps["body"]
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect();

            let attributes = Self::parse_attributes(&caps["attributes"])?;

            methods.push(ContractMethod {
                name,
                visibility,
                is_mut: name.contains("mut") || body_lines.iter().any(|line| line.contains("env.")),
                parameters,
                return_type,
                body: body_lines,
                line_start: line_num + 1,
                line_end: line_num + 1,
                attributes,
                is_contract_method: true, // All methods in contractimpl are contract methods
            });
        }

        Ok(methods)
    }

    fn parse_storage_patterns(source: &str) -> anyhow::Result<Vec<StorageKey>> {
        let mut storage_keys = Vec::new();
        
        // Find storage access patterns
        let storage_regex = regex::Regex::new(
            r"env\.storage\(\)\.(?P<storage_type>instance|persistent|temporary)\(\)\.(?P<operation>get|set|remove)\(&(?P<key>[^)]+)\)"
        )?;

        for (line_num, caps) in storage_regex.captures_iter(source).enumerate() {
            let storage_type = match &caps["storage_type"] {
                "instance" => StorageType::Instance,
                "persistent" => StorageType::Persistent,
                "temporary" => StorageType::Temporary,
                _ => continue,
            };

            let key_expr = &caps["key"];
            let key_name = Self::extract_key_name(key_expr);

            let location = CodeLocation {
                file_path: "unknown".to_string(), // Would be passed in real implementation
                line: line_num + 1,
                column: 0,
                function: None,
                contract: None,
            };

            storage_keys.push(StorageKey {
                name: key_name,
                key_type: "unknown".to_string(), // Could be enhanced with type inference
                usage_locations: vec![location],
                storage_type,
            });
        }

        Ok(storage_keys)
    }

    fn parse_event_patterns(source: &str) -> anyhow::Result<Vec<EventPattern>> {
        let mut events = Vec::new();
        
        // Find event emission patterns
        let event_regex = regex::Regex::new(
            r"env\.events\(\)\.publish\(\((?P<content>[^)]+)\)\)"
        )?;

        for (line_num, caps) in event_regex.captures_iter(source).enumerate() {
            let content = &caps["content"];
            let (symbol, data_types) = Self::parse_event_content(content)?;

            let location = CodeLocation {
                file_path: "unknown".to_string(),
                line: line_num + 1,
                column: 0,
                function: None,
                contract: None,
            };

            events.push(EventPattern {
                symbol,
                data_types,
                locations: vec![location],
                is_short_symbol: symbol.contains("short!"),
            });
        }

        Ok(events)
    }

    fn parse_soroban_types(source: &str) -> anyhow::Result<Vec<SorobanTypeUsage>> {
        let mut type_usages = Vec::new();
        
        // Soroban-specific types
        let soroban_types = [
            "Address", "Symbol", "Vec", "Map", "Set", "Bytes", "BytesN", 
            "Duration", "Timepoint", "U256", "I256", "BigUint"
        ];

        for type_name in soroban_types.iter() {
            let type_regex = regex::Regex::new(&format!(r"\b{}\b", type_name))?;
            
            for (line_num, _) in type_regex.find_iter(source).enumerate() {
                let location = CodeLocation {
                    file_path: "unknown".to_string(),
                    line: line_num + 1,
                    column: 0,
                    function: None,
                    contract: None,
                };

                type_usages.push(SorobanTypeUsage {
                    type_name: type_name.to_string(),
                    usage_locations: vec![location],
                    is_generic: vec!["Vec", "Map", "Set", "BytesN"].contains(type_name),
                    generic_params: if vec!["Vec", "Map", "Set", "BytesN"].contains(type_name) {
                        Self::extract_generic_params_from_usage(source, type_name)
                    } else {
                        Vec::new()
                    },
                });
            }
        }

        Ok(type_usages)
    }

    // Helper methods
    fn extract_key_name(key_expr: &str) -> String {
        // Simple extraction - could be enhanced
        if key_expr.contains("DataKey::") {
            key_expr.split("::").last().unwrap_or(key_expr).to_string()
        } else {
            key_expr.to_string()
        }
    }

    fn parse_event_content(content: &str) -> (String, Vec<Type>) {
        let parts: Vec<&str> = content.split(',').collect();
        if parts.is_empty() {
            return (String::new(), Vec::new());
        }

        let symbol_part = parts[0].trim();
        let symbol = if symbol_part.contains("Symbol::new") {
            "Symbol::new".to_string()
        } else if symbol_part.contains("Symbol::short!") {
            "Symbol::short".to_string()
        } else {
            symbol_part.to_string()
        };

        let data_types = parts[1..]
            .iter()
            .map(|part| Type::Simple(part.trim().to_string()))
            .collect();

        (symbol, data_types)
    }

    fn extract_generic_params_from_usage(source: &str, type_name: &str) -> Vec<String> {
        let generic_regex = regex::Regex::new(&format!(r"{}\s*<\s*([^>]+)\s*>", type_name)).unwrap();
        
        if let Some(caps) = generic_regex.captures(source) {
            caps[1]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        } else {
            Vec::new()
        }
    }
}
```

### 2. tests/ast_tests.rs (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\tests\ast_tests.rs`

**Complete Content**:
```rust
use soroban_security_guard::ast::{AstParser, SorobanContract, StorageType};
use std::fs;

#[test]
fn test_parse_contract_struct() {
    let source = r#"
    #[contract]
    pub struct TokenContract {
        #[contractstate]
        admin: Address,
        #[contractstate]
        total_supply: u64,
    }
    "#;
    
    let contract_struct = AstParser::parse_contract_struct(source).unwrap();
    assert!(contract_struct.is_some());
    
    let contract_struct = contract_struct.unwrap();
    assert_eq!(contract_struct.name, "TokenContract");
    assert_eq!(contract_struct.fields.len(), 2);
    assert_eq!(contract_struct.fields[0].name, "admin");
    assert_eq!(contract_struct.fields[1].name, "total_supply");
}

#[test]
fn test_parse_contract_impl() {
    let source = r#"
    #[contractimpl]
    impl TokenContract {
        pub fn initialize(env: &Env, admin: Address) {
            env.storage().instance().set(&DataKey::Admin, &admin);
        }
        
        pub fn transfer(env: &Env, to: Address, amount: u64) -> Result<(), &'static str> {
            // Implementation
            Ok(())
        }
    }
    "#;
    
    let contract_impl = AstParser::parse_contract_impl(source).unwrap();
    assert!(contract_impl.is_some());
    
    let contract_impl = contract_impl.unwrap();
    assert_eq!(contract_impl.target, "TokenContract");
    assert_eq!(contract_impl.methods.len(), 2);
    assert_eq!(contract_impl.methods[0].name, "initialize");
    assert_eq!(contract_impl.methods[1].name, "transfer");
}

#[test]
fn test_parse_storage_patterns() {
    let source = r#"
    env.storage().instance().get(&DataKey::Admin);
    env.storage().persistent().set(&DataKey::Balance, &amount);
    env.storage().temporary().get(&Key::Temporary);
    "#;
    
    let storage_keys = AstParser::parse_storage_patterns(source).unwrap();
    assert_eq!(storage_keys.len(), 3);
    
    assert_eq!(storage_keys[0].storage_type, StorageType::Instance);
    assert_eq!(storage_keys[1].storage_type, StorageType::Persistent);
    assert_eq!(storage_keys[2].storage_type, StorageType::Temporary);
}

#[test]
fn test_parse_event_patterns() {
    let source = r#"
    env.events().publish((Symbol::new(env, "Transfer"), (from, to, amount)));
    env.events().publish((Symbol::short!("MINT"), (recipient, amount)));
    "#;
    
    let events = AstParser::parse_event_patterns(source).unwrap();
    assert_eq!(events.len(), 2);
    
    assert_eq!(events[0].symbol, "Symbol::new");
    assert!(!events[0].is_short_symbol);
    assert_eq!(events[1].symbol, "Symbol::short");
    assert!(events[1].is_short_symbol);
}

#[test]
fn test_parse_soroban_types() {
    let source = r#"
    let admin: Address = Address::zero();
    let symbol: Symbol = Symbol::new(env, "TOKEN");
    let balances: Map<Address, u64> = Map::new(env);
    let data: BytesN<32> = BytesN::from_array(env, &[0u8; 32]);
    "#;
    
    let type_usages = AstParser::parse_soroban_types(source).unwrap();
    assert!(type_usages.len() >= 4);
    
    let address_usage = type_usages.iter().find(|t| t.type_name == "Address").unwrap();
    assert!(!address_usage.usage_locations.is_empty());
    
    let map_usage = type_usages.iter().find(|t| t.type_name == "Map").unwrap();
    assert!(map_usage.is_generic);
}

#[test]
fn test_parse_full_soroban_contract() {
    let source = r#"
    #[contract]
    pub struct TokenContract {
        #[contractstate]
        admin: Address,
        #[contractstate]
        total_supply: u64,
    }

    #[contractimpl]
    impl TokenContract {
        pub fn initialize(env: &Env, admin: Address) {
            env.storage().instance().set(&DataKey::Admin, &admin);
            env.storage().instance().set(&DataKey::TotalSupply, &1000000);
            env.events().publish((Symbol::new(env, "Initialized"), (admin, 1000000)));
        }
        
        pub fn transfer(env: &Env, from: Address, to: Address, amount: u64) -> Result<(), &'static str> {
            let from_balance = env.storage().instance().get(&DataKey::Balance(from));
            let to_balance = env.storage().instance().get(&DataKey::Balance(to));
            
            if from_balance < amount {
                return Err("Insufficient balance");
            }
            
            env.storage().instance().set(&DataKey::Balance(from), &(from_balance - amount));
            env.storage().instance().set(&DataKey::Balance(to), &(to_balance + amount));
            
            env.events().publish((Symbol::new(env, "Transfer"), (from, to, amount)));
            Ok(())
        }
    }
    "#;
    
    let contract = AstParser::parse_soroban_contract(source, "test.rs").unwrap();
    
    assert_eq!(contract.name, "TokenContract");
    assert!(contract.contract_struct.is_some());
    assert!(contract.implementation.is_some());
    assert!(!contract.storage_keys.is_empty());
    assert!(!contract.events.is_empty());
    assert!(!contract.soroban_types.is_empty());
}

#[test]
fn test_real_soroban_contract() {
    // Test with actual example file if it exists
    let contract_path = "examples/vulnerable_contract.rs";
    if std::path::Path::new(contract_path).exists() {
        let source_code = fs::read_to_string(contract_path).unwrap();
        let contract = AstParser::parse_soroban_contract(&source_code, contract_path).unwrap();
        
        // Should parse without panicking
        assert!(!contract.name.is_empty());
    }
}

#[test]
fn test_parser_performance() {
    use std::time::Instant;
    
    let large_contract = generate_large_soroban_contract();
    let start = Instant::now();
    let _contract = AstParser::parse_soroban_contract(&large_contract, "large.rs").unwrap();
    let duration = start.elapsed();
    
    // Should parse in under 1 second for typical contract
    assert!(duration.as_millis() < 1000, "Parser took too long: {:?}", duration);
}

fn generate_large_soroban_contract() -> String {
    let mut contract = String::new();
    
    contract.push_str("#[contract]\npub struct LargeContract {\n");
    for i in 0..20 {
        contract.push_str(&format!("    field_{}: u64,\n", i));
    }
    contract.push_str("}\n\n");
    
    contract.push_str("#[contractimpl]\nimpl LargeContract {\n");
    for i in 0..50 {
        contract.push_str(&format!(
            r#"
    pub fn method_{}(env: &Env, param: u64) -> u64 {{
        let result = param + {};
        env.storage().instance().set(&DataKey::Field{}, &result);
        env.events().publish((Symbol::new(env, "Method{}"), (param, result)));
        result
    }}
"#, i, i, i, i
        ));
    }
    contract.push_str("}\n");
    
    contract
}
```

### 3. examples/soroban_patterns.rs (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\examples\soroban_patterns.rs`

**Complete Content**:
```rust
#![no_std]
#![no_main]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec, Map, BytesN};

#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    Allowance(Address, Address),
    TotalSupply,
    Nonce(u64),
}

#[contract]
pub struct TokenContract {
    #[contractstate]
    admin: Address,
    #[contractstate]
    total_supply: u64,
}

#[contractimpl]
impl TokenContract {
    // Contract initialization
    pub fn initialize(env: &Env, admin: Address, initial_supply: u64) {
        // Storage patterns
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TotalSupply, &initial_supply);
        env.storage().instance().set(&DataKey::Balance(admin), &initial_supply);
        
        // Event emission
        env.events().publish((Symbol::new(env, "Initialized"), (admin, initial_supply)));
    }
    
    // Transfer function with storage and events
    pub fn transfer(env: &Env, from: Address, to: Address, amount: u64) -> Result<(), &'static str> {
        // Instance storage access
        let from_balance = env.storage().instance().get(&DataKey::Balance(from));
        let to_balance = env.storage().instance().get(&DataKey::Balance(to));
        
        if from_balance < amount {
            return Err("Insufficient balance");
        }
        
        // Update storage
        env.storage().instance().set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage().instance().set(&DataKey::Balance(to), &(to_balance + amount));
        
        // Event emission
        env.events().publish((Symbol::new(env, "Transfer"), (from, to, amount)));
        
        Ok(())
    }
    
    // Approval function with persistent storage
    pub fn approve(env: &Env, owner: Address, spender: Address, amount: u64) {
        // Persistent storage access
        env.storage().persistent().set(&DataKey::Allowance(owner, spender), &amount);
        
        // Short symbol event
        env.events().publish((Symbol::short!("APPROVE"), (owner, spender, amount)));
    }
    
    // Function using Soroban types
    pub fn batch_transfer(env: &Env, transfers: Vec<(Address, u64)>) -> Result<(), &'static str> {
        let mut total_transferred: u64 = 0;
        
        for (recipient, amount) in transfers.iter() {
            let current_balance = env.storage().instance().get(&DataKey::Balance(*recipient));
            let new_balance = current_balance.checked_add(*amount)
                .ok_or("Overflow in batch transfer")?;
            
            env.storage().instance().set(&DataKey::Balance(*recipient), &new_balance);
            total_transferred += amount;
            
            // Complex event data
            env.events().publish((
                Symbol::new(env, "BatchTransfer"),
                (recipient, amount, total_transferred)
            ));
        }
        
        Ok(())
    }
    
    // Function with temporary storage
    pub fn calculate_with_cache(env: &Env, value: u64) -> u64 {
        // Temporary storage for caching
        let cache_key = BytesN::from_array(env, &value.to_be_bytes());
        
        if let Some(cached_result) = env.storage().temporary().get(&cache_key) {
            return cached_result;
        }
        
        let result = value * 2; // Simple calculation
        
        // Cache the result
        env.storage().temporary().set(&cache_key, &result);
        
        result
    }
    
    // Function using Map type
    pub fn get_all_balances(env: &Env, addresses: Vec<Address>) -> Map<Address, u64> {
        let mut balances = Map::new(env);
        
        for address in addresses {
            let balance = env.storage().instance().get(&DataKey::Balance(address));
            balances.set(address, balance);
        }
        
        balances
    }
    
    // Function with time-related types
    pub fn time_locked_transfer(env: &Env, to: Address, amount: u64, unlock_time: u64) {
        let current_time = env.ledger().timestamp();
        
        if current_time >= unlock_time {
            // Transfer is unlocked
            Self::transfer(env, env.current_contract_address(), to, amount).unwrap();
            
            env.events().publish((
                Symbol::new(env, "TimeLockUnlocked"),
                (to, amount, unlock_time)
            ));
        } else {
            // Still locked
            env.events().publish((
                Symbol::new(env, "TimeLockActive"),
                (to, amount, unlock_time, current_time)
            ));
        }
    }
}

// Additional contract for testing patterns
#[contract]
pub struct NFTContract {
    #[contractstate]
    owner: Address,
    #[contractstate]
    token_counter: u64,
}

#[contracttype]
pub enum NFTDataKey {
    Owner(u64),
    TokenURI(u64),
    Approved(u64),
}

#[contractimpl]
impl NFTContract {
    pub fn mint(env: &Env, to: Address, token_uri: BytesN<32>) -> u64 {
        let token_id = env.storage().instance().get(&NFTDataKey::TokenCounter) + 1;
        
        // Update token counter
        env.storage().instance().set(&NFTDataKey::TokenCounter, &token_id);
        
        // Set owner
        env.storage().instance().set(&NFTDataKey::Owner(token_id), &to);
        
        // Set URI
        env.storage().persistent().set(&NFTDataKey::TokenURI(token_id), &token_uri);
        
        // Event
        env.events().publish((
            Symbol::new(env, "Mint"),
            (to, token_id, token_uri)
        ));
        
        token_id
    }
    
    pub fn transfer_from(env: &Env, from: Address, to: Address, token_id: u64) {
        let owner = env.storage().instance().get(&NFTDataKey::Owner(token_id));
        
        // Check ownership
        require!(owner == from, "Not token owner");
        
        // Update owner
        env.storage().instance().set(&NFTDataKey::Owner(token_id), &to);
        
        // Clear approval
        env.storage().instance().remove(&NFTDataKey::Approved(token_id));
        
        // Event
        env.events().publish((
            Symbol::new(env, "Transfer"),
            (from, to, token_id)
        ));
    }
}

// Helper macro for require
macro_rules! require {
    ($condition:expr, $message:expr) => {
        if !$condition {
            panic!($message);
        }
    };
}
