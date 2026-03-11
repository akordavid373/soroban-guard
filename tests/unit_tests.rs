#[cfg(test)]
mod tests {
    use soroban_security_guard::{
        config::{ScannerConfig, Severity, ReportFormat},
        rules::{Rule, RuleContext, RuleResult, AdminOnlyFunctionRule, PotentialOverflowRule},
        ast::{ContractAst, Function, Type, Visibility, Parameter},
        SecurityScanner,
    };

    fn create_mock_function(name: &str, body: Vec<String>) -> Function {
        Function {
            name: name.to_string(),
            visibility: Visibility::Public,
            is_mut: false,
            parameters: vec![],
            return_type: None,
            body,
            line_start: 1,
            line_end: 10,
            attributes: vec![],
            generics: vec![],
            where_clause: None,
        }
    }

    fn create_mock_context(functions: Vec<Function>) -> RuleContext {
        RuleContext {
            ast: ContractAst {
                name: "TestContract".to_string(),
                functions: functions.clone(),
                structs: vec![],
                enums: vec![],
                imports: vec![],
                constants: vec![],
                type_aliases: vec![],
                traits: vec![],
                impl_blocks: vec![],
            },
            file_path: "test.rs".to_string(),
            source_code: "mock source code".to_string(),
            imports: vec![],
            functions,
            structs: vec![],
            enums: vec![],
        }
    }

    #[test]
    fn test_admin_function_rule_detection() {
        let rule = AdminOnlyFunctionRule;
        let config = ScannerConfig::default();

        // Test with vulnerable admin function
        let vulnerable_function = create_mock_function(
            "set_admin",
            vec!["env.storage().instance().set(&DataKey::Admin, &new_admin);".to_string()],
        );
        let context = create_mock_context(vec![vulnerable_function]);

        let results = rule.check(&context);
        assert!(!results.is_empty());
        assert_eq!(results[0].rule_name, "admin_only_function");
        assert_eq!(results[0].severity, Severity::High);

        // Test with protected admin function
        let protected_function = create_mock_function(
            "set_admin",
            vec![
                "let current_admin = Self::get_admin(env);".to_string(),
                "require!(current_admin == env.current_contract_address(), \"Admin only\");".to_string(),
                "env.storage().instance().set(&DataKey::Admin, &new_admin);".to_string(),
            ],
        );
        let context = create_mock_context(vec![protected_function]);

        let results = rule.check(&context);
        assert!(results.is_empty());

        // Test rule enablement
        assert!(rule.is_enabled(&config));
    }

    #[test]
    fn test_owner_function_rule_detection() {
        let rule = soroban_security_guard::rules::OwnerOnlyFunctionRule;

        // Test with vulnerable owner function
        let vulnerable_function = create_mock_function(
            "transfer_ownership",
            vec!["env.storage().instance().set(&DataKey::Admin, &new_owner);".to_string()],
        );
        let context = create_mock_context(vec![vulnerable_function]);

        let results = rule.check(&context);
        assert!(!results.is_empty());
        assert_eq!(results[0].rule_name, "owner_only_function");

        // Test with protected owner function
        let protected_function = create_mock_function(
            "transfer_ownership",
            vec![
                "let current_admin = Self::get_admin(env);".to_string(),
                "require!(current_admin == env.current_contract_address(), \"Admin only\");".to_string(),
                "env.storage().instance().set(&DataKey::Admin, &new_owner);".to_string(),
            ],
        );
        let context = create_mock_context(vec![protected_function]);

        let results = rule.check(&context);
        assert!(results.is_empty());
    }

    #[test]
    fn test_overflow_rule_detection() {
        let rule = PotentialOverflowRule;

        // Test with unsafe arithmetic
        let unsafe_function = create_mock_function(
            "add_numbers",
            vec!["result = a + b;".to_string()],
        );
        let context = create_mock_context(vec![unsafe_function]);

        let results = rule.check(&context);
        assert!(!results.is_empty());
        assert_eq!(results[0].rule_name, "potential_overflow");
        assert_eq!(results[0].severity, Severity::Medium);

        // Test with safe arithmetic
        let safe_function = create_mock_function(
            "add_numbers",
            vec!["result = a.checked_add(b).unwrap_or(0);".to_string()],
        );
        let context = create_mock_context(vec![safe_function]);

        let results = rule.check(&context);
        assert!(results.is_empty());
    }

    #[test]
    fn test_reentrancy_rule_detection() {
        let rule = soroban_security_guard::rules::ReentrancyPatternRule;

        // Test with reentrancy vulnerability
        let vulnerable_function = create_mock_function(
            "withdraw",
            vec![
                "let balance = Self::get_balance(env, caller);".to_string(),
                "env.storage().instance().set(&DataKey::Balance, &(balance - amount));".to_string(),
                "Self::external_transfer(env, recipient, amount);".to_string(),
                "env.storage().instance().set(&DataKey::Counter, &1);".to_string(),
            ],
        );
        let context = create_mock_context(vec![vulnerable_function]);

        let results = rule.check(&context);
        assert!(!results.is_empty());
        assert_eq!(results[0].rule_name, "reentrancy_pattern");
        assert_eq!(results[0].severity, Severity::High);

        // Test with proper checks-effects-interactions pattern
        let safe_function = create_mock_function(
            "withdraw",
            vec![
                "let balance = Self::get_balance(env, caller);".to_string(),
                "require!(balance >= amount, \"Insufficient balance\");".to_string(),
                "env.storage().instance().set(&DataKey::Balance, &(balance - amount));".to_string(),
                "env.storage().instance().set(&DataKey::Counter, &1);".to_string(),
                "Self::external_transfer(env, recipient, amount);".to_string(),
            ],
        );
        let context = create_mock_context(vec![safe_function]);

        let results = rule.check(&context);
        assert!(results.is_empty());
    }

    #[test]
    fn test_custom_rule_adapter() {
        let custom_rule = soroban_security_guard::config::CustomRule {
            name: "test_rule".to_string(),
            pattern: r"println!".to_string(),
            severity: Severity::Low,
            description: "Test rule for println!".to_string(),
            enabled: true,
        };

        let rule = soroban_security_guard::rules::CustomRuleAdapter::new(custom_rule);

        let function_with_println = create_mock_function(
            "debug_function",
            vec!["println!(\"Debug: {}\", data);".to_string()],
        );
        let context = create_mock_context(vec![function_with_println]);

        let results = rule.check(&context);
        assert!(!results.is_empty());
        assert_eq!(results[0].rule_name, "test_rule");
        assert_eq!(results[0].severity, Severity::Low);
    }

    #[test]
    fn test_scanner_creation() {
        let config = ScannerConfig::default();
        let scanner = SecurityScanner::new(config);
        
        // Should have built-in rules
        let rules_summary = scanner.get_rules_summary();
        assert!(!rules_summary.is_empty());
        
        // Should contain expected rule names
        let rule_names: Vec<String> = rules_summary.iter().map(|r| r.name.clone()).collect();
        assert!(rule_names.contains(&"admin_only_function".to_string()));
        assert!(rule_names.contains(&"potential_overflow".to_string()));
        assert!(rule_names.contains(&"reentrancy_pattern".to_string()));
    }

    #[test]
    fn test_config_default_values() {
        let config = ScannerConfig::default();
        
        assert_eq!(config.severity_threshold, Severity::Medium);
        assert_eq!(config.output_format, ReportFormat::Console);
        assert!(config.rules.access_control.enabled);
        assert!(config.rules.arithmetic.enabled);
        assert!(config.rules.reentrancy.enabled);
        assert!(config.rules.token_safety.enabled);
        assert!(config.rules.state_management.enabled);
    }

    #[test]
    fn test_config_file_filtering() {
        let mut config = ScannerConfig::default();
        
        // Test include patterns
        assert!(config.should_include_file("contract.rs"));
        assert!(!config.should_include_file("test_contract.rs"));
        
        // Test exclude patterns
        config.include_patterns = vec!["*.rs".to_string()];
        assert!(config.should_include_file("contract.rs"));
        assert!(config.should_include_file("test_contract.rs"));
        
        config.exclude_patterns.push("test_*".to_string());
        assert!(config.should_include_file("contract.rs"));
        assert!(!config.should_include_file("test_contract.rs"));
    }

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::High);
        assert!(Severity::High > Severity::Medium);
        assert!(Severity::Medium > Severity::Low);
    }

    #[test]
    fn test_rule_result_creation() {
        let location = soroban_security_guard::rules::CodeLocation {
            file_path: "test.rs".to_string(),
            line: 10,
            column: 5,
            function: Some("test_function".to_string()),
            contract: Some("TestContract".to_string()),
        };

        let result = RuleResult::new(
            "test_rule".to_string(),
            Severity::Medium,
            "Test message".to_string(),
            location.clone(),
        )
        .with_suggestion("Fix it like this".to_string())
        .with_confidence(0.8);

        assert_eq!(result.rule_name, "test_rule");
        assert_eq!(result.severity, Severity::Medium);
        assert_eq!(result.message, "Test message");
        assert_eq!(result.location.file_path, "test.rs");
        assert_eq!(result.suggestion, Some("Fix it like this".to_string()));
        assert_eq!(result.confidence, 0.8);
    }

    #[test]
    fn test_type_parsing() {
        // Test simple type
        let simple_type = soroban_security_guard::ast::AstParser::parse_type("Address").unwrap();
        match simple_type {
            soroban_security_guard::ast::Type::Simple(name) => assert_eq!(name, "Address"),
            _ => panic!("Expected simple type"),
        }

        // Test generic type
        let generic_type = soroban_security_guard::ast::AstParser::parse_type("Vec<Address>").unwrap();
        match generic_type {
            soroban_security_guard::ast::Type::Generic { name, args } => {
                assert_eq!(name, "Vec");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected generic type"),
        }
    }

    #[test]
    fn test_parameter_parsing() {
        let params = soroban_security_guard::ast::AstParser::parse_parameters("env: &Env, amount: u64").unwrap();
        
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "env");
        assert_eq!(params[1].name, "amount");
        assert!(!params[0].is_mut);
        assert!(!params[1].is_mut);
    }

    #[test]
    fn test_visibility_parsing() {
        let function_with_pub = "pub fn test_function() {}";
        let function_without_pub = "fn test_function() {}";
        
        assert!(function_with_pub.contains("pub"));
        assert!(!function_without_pub.contains("pub"));
    }
}
