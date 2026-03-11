use crate::config::{Severity, CustomRule};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleResult {
    pub rule_name: String,
    pub severity: Severity,
    pub message: String,
    pub location: CodeLocation,
    pub suggestion: Option<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLocation {
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    pub function: Option<String>,
    pub contract: Option<String>,
}

impl RuleResult {
    pub fn new(
        rule_name: String,
        severity: Severity,
        message: String,
        location: CodeLocation,
    ) -> Self {
        Self {
            rule_name,
            severity,
            message,
            location,
            suggestion: None,
            confidence: 1.0,
        }
    }

    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence;
        self
    }
}

impl fmt::Display for RuleResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} at {}:{}:{} - {}",
            self.severity,
            self.rule_name,
            self.location.file_path,
            self.location.line,
            self.location.column,
            self.message
        )
    }
}

pub trait Rule {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn severity(&self) -> Severity;
    fn check(&self, context: &RuleContext) -> Vec<RuleResult>;
    fn is_enabled(&self, config: &crate::config::ScannerConfig) -> bool;
}

#[derive(Debug)]
pub struct RuleContext {
    pub ast: crate::ast::ContractAst,
    pub file_path: String,
    pub source_code: String,
    pub imports: Vec<String>,
    pub functions: Vec<crate::ast::Function>,
    pub structs: Vec<crate::ast::Struct>,
    pub enums: Vec<crate::ast::Enum>,
}

// Access Control Rules
pub struct AdminOnlyFunctionRule;
pub struct OwnerOnlyFunctionRule;
pub struct MissingAccessControlRule;

// Arithmetic Rules
pub struct PotentialOverflowRule;
pub struct PotentialUnderflowRule;
pub struct UnsafeMathRule;

// Reentrancy Rules
pub struct ReentrancyPatternRule;
pub struct StateChangeAfterExternalCallRule;

// Token Safety Rules
pub struct UnsafeApproveRule;
pub struct UnsafeTransferRule;
pub struct MissingBalanceCheckRule;

// State Management Rules
pub struct UninitializedStateRule;
pub struct RaceConditionRule;
pub struct NonAtomicOperationRule;

impl Rule for AdminOnlyFunctionRule {
    fn name(&self) -> &str {
        "admin_only_function"
    }

    fn description(&self) -> &str {
        "Functions that should be admin-only but lack access control"
    }

    fn severity(&self) -> Severity {
        Severity::High
    }

    fn check(&self, context: &RuleContext) -> Vec<RuleResult> {
        let mut results = Vec::new();
        
        for function in &context.functions {
            if is_admin_function(&function.name) && !has_access_control(&function) {
                let location = CodeLocation {
                    file_path: context.file_path.clone(),
                    line: function.line_start,
                    column: 0,
                    function: Some(function.name.clone()),
                    contract: None,
                };

                results.push(RuleResult::new(
                    self.name().to_string(),
                    self.severity(),
                    format!(
                        "Function '{}' appears to be admin-only but lacks access control",
                        function.name
                    ),
                    location,
                ).with_suggestion(
                    "Add access control check at the beginning of the function".to_string()
                ));
            }
        }

        results
    }

    fn is_enabled(&self, config: &crate::config::ScannerConfig) -> bool {
        config.rules.access_control.enabled && config.rules.access_control.check_admin_functions
    }
}

impl Rule for OwnerOnlyFunctionRule {
    fn name(&self) -> &str {
        "owner_only_function"
    }

    fn description(&self) -> &str {
        "Functions that should be owner-only but lack access control"
    }

    fn severity(&self) -> Severity {
        Severity::High
    }

    fn check(&self, context: &RuleContext) -> Vec<RuleResult> {
        let mut results = Vec::new();
        
        for function in &context.functions {
            if is_owner_function(&function.name) && !has_access_control(&function) {
                let location = CodeLocation {
                    file_path: context.file_path.clone(),
                    line: function.line_start,
                    column: 0,
                    function: Some(function.name.clone()),
                    contract: None,
                };

                results.push(RuleResult::new(
                    self.name().to_string(),
                    self.severity(),
                    format!(
                        "Function '{}' appears to be owner-only but lacks access control",
                        function.name
                    ),
                    location,
                ).with_suggestion(
                    "Add owner access control check at the beginning of the function".to_string()
                ));
            }
        }

        results
    }

    fn is_enabled(&self, config: &crate::config::ScannerConfig) -> bool {
        config.rules.access_control.enabled && config.rules.access_control.check_owner_functions
    }
}

impl Rule for PotentialOverflowRule {
    fn name(&self) -> &str {
        "potential_overflow"
    }

    fn description(&self) -> &str {
        "Potential integer overflow in arithmetic operations"
    }

    fn severity(&self) -> Severity {
        Severity::Medium
    }

    fn check(&self, context: &RuleContext) -> Vec<RuleResult> {
        let mut results = Vec::new();
        
        for function in &context.functions {
            for statement in &function.body {
                if let Some(arith_op) = detect_arithmetic_operation(statement) {
                    if !uses_safe_math(&function.body) {
                        let location = CodeLocation {
                            file_path: context.file_path.clone(),
                            line: function.line_start,
                            column: 0,
                            function: Some(function.name.clone()),
                            contract: None,
                        };

                        results.push(RuleResult::new(
                            self.name().to_string(),
                            self.severity(),
                            format!(
                                "Potential overflow in arithmetic operation: {}",
                                arith_op
                            ),
                            location,
                        ).with_suggestion(
                            "Use safe arithmetic functions or add overflow checks".to_string()
                        ));
                    }
                }
            }
        }

        results
    }

    fn is_enabled(&self, config: &crate::config::ScannerConfig) -> bool {
        config.rules.arithmetic.enabled && config.rules.arithmetic.check_overflow
    }
}

impl Rule for ReentrancyPatternRule {
    fn name(&self) -> &str {
        "reentrancy_pattern"
    }

    fn description(&self) -> &str {
        "Potential reentrancy vulnerability detected"
    }

    fn severity(&self) -> Severity {
        Severity::High
    }

    fn check(&self, context: &RuleContext) -> Vec<RuleResult> {
        let mut results = Vec::new();
        
        for function in &context.functions {
            let external_calls = find_external_calls(&function.body);
            let state_changes = find_state_changes(&function.body);
            
            if !external_calls.is_empty() && state_changes.len() > external_calls.len() {
                // State changes after external calls
                let location = CodeLocation {
                    file_path: context.file_path.clone(),
                    line: function.line_start,
                    column: 0,
                    function: Some(function.name.clone()),
                    contract: None,
                };

                results.push(RuleResult::new(
                    self.name().to_string(),
                    self.severity(),
                    format!(
                        "Function '{}' has state changes after external calls, potential reentrancy",
                        function.name
                    ),
                    location,
                ).with_suggestion(
                    "Apply checks-effects-interactions pattern: move state changes before external calls".to_string()
                ));
            }
        }

        results
    }

    fn is_enabled(&self, config: &crate::config::ScannerConfig) -> bool {
        config.rules.reentrancy.enabled
    }
}

// Helper functions
fn is_admin_function(function_name: &str) -> bool {
    let admin_patterns = [
        "admin_", "set_admin", "change_admin", "upgrade", "migrate",
        "emergency_", "pause", "unpause", "set_config", "update_config"
    ];
    
    admin_patterns.iter().any(|pattern| function_name.contains(pattern))
}

fn is_owner_function(function_name: &str) -> bool {
    let owner_patterns = [
        "owner_", "transfer_ownership", "renounce_ownership", "claim_ownership",
        "set_owner", "change_owner", "only_owner"
    ];
    
    owner_patterns.iter().any(|pattern| function_name.contains(pattern))
}

fn has_access_control(function: &crate::ast::Function) -> bool {
    // Check for common access control patterns
    function.body.iter().any(|stmt| {
        stmt.contains("require") || 
        stmt.contains("assert") || 
        stmt.contains("panic!") ||
        stmt.contains("only_admin") ||
        stmt.contains("only_owner") ||
        stmt.contains("has_auth")
    })
}

fn detect_arithmetic_operation(statement: &str) -> Option<String> {
    if statement.contains('+') || statement.contains('-') || 
       statement.contains('*') || statement.contains('/') {
        Some(statement.to_string())
    } else {
        None
    }
}

fn uses_safe_math(body: &[String]) -> bool {
    body.iter().any(|stmt| {
        stmt.contains("checked_add") || 
        stmt.contains("checked_sub") || 
        stmt.contains("checked_mul") ||
        stmt.contains("safe_add") ||
        stmt.contains("safe_sub") ||
        stmt.contains("safe_mul")
    })
}

fn find_external_calls(body: &[String]) -> Vec<usize> {
    body.iter()
        .enumerate()
        .filter(|(_, stmt)| {
            stmt.contains("invoke_contract") ||
            stmt.contains("call") ||
            stmt.contains("transfer") ||
            stmt.contains("send")
        })
        .map(|(i, _)| i)
        .collect()
}

fn find_state_changes(body: &[String]) -> Vec<usize> {
    body.iter()
        .enumerate()
        .filter(|(_, stmt)| {
            stmt.contains("=") && 
            (stmt.contains("env.") || stmt.contains("self."))
        })
        .map(|(i, _)| i)
        .collect()
}

pub struct CustomRuleAdapter {
    rule: CustomRule,
}

impl CustomRuleAdapter {
    pub fn new(rule: CustomRule) -> Self {
        Self { rule }
    }
}

impl Rule for CustomRuleAdapter {
    fn name(&self) -> &str {
        &self.rule.name
    }

    fn description(&self) -> &str {
        &self.rule.description
    }

    fn severity(&self) -> Severity {
        self.rule.severity
    }

    fn check(&self, context: &RuleContext) -> Vec<RuleResult> {
        let mut results = Vec::new();
        
        if let Ok(regex) = regex::Regex::new(&self.rule.pattern) {
            for (line_num, line) in context.source_code.lines().enumerate() {
                if regex.is_match(line) {
                    let location = CodeLocation {
                        file_path: context.file_path.clone(),
                        line: line_num + 1,
                        column: 0,
                        function: None,
                        contract: None,
                    };

                    results.push(RuleResult::new(
                        self.rule.name.clone(),
                        self.rule.severity,
                        self.rule.description.clone(),
                        location,
                    ).with_confidence(0.8));
                }
            }
        }

        results
    }

    fn is_enabled(&self, _config: &crate::config::ScannerConfig) -> bool {
        self.rule.enabled
    }
}
