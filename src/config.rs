use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerConfig {
    pub severity_threshold: Severity,
    pub exclude_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
    pub output_format: ReportFormat,
    pub rules: RulesConfig,
    pub max_depth: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesConfig {
    pub access_control: AccessControlConfig,
    pub arithmetic: ArithmeticConfig,
    pub reentrancy: ReentrancyConfig,
    pub token_safety: TokenSafetyConfig,
    pub state_management: StateManagementConfig,
    pub custom_rules: Vec<CustomRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    pub enabled: bool,
    pub strict_mode: bool,
    pub check_admin_functions: bool,
    pub check_owner_functions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArithmeticConfig {
    pub enabled: bool,
    pub check_overflow: bool,
    pub check_underflow: bool,
    pub safe_math_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReentrancyConfig {
    pub enabled: bool,
    pub check_external_calls: bool,
    pub check_state_changes: bool,
    pub require_checks_effect: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSafetyConfig {
    pub enabled: bool,
    pub check_erc20: bool,
    pub check_erc721: bool,
    pub check_approve_patterns: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateManagementConfig {
    pub enabled: bool,
    pub check_uninitialized_state: bool,
    pub check_race_conditions: bool,
    pub check_atomic_operations: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRule {
    pub name: String,
    pub pattern: String,
    pub severity: Severity,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    Console,
    Json,
    Html,
    Sarif,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            severity_threshold: Severity::Medium,
            exclude_patterns: vec![
                "test_*".to_string(),
                "mock_*".to_string(),
                "*_test.rs".to_string(),
            ],
            include_patterns: vec!["*.rs".to_string()],
            output_format: ReportFormat::Console,
            rules: RulesConfig::default(),
            max_depth: Some(10),
        }
    }
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            access_control: AccessControlConfig::default(),
            arithmetic: ArithmeticConfig::default(),
            reentrancy: ReentrancyConfig::default(),
            token_safety: TokenSafetyConfig::default(),
            state_management: StateManagementConfig::default(),
            custom_rules: vec![],
        }
    }
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strict_mode: false,
            check_admin_functions: true,
            check_owner_functions: true,
        }
    }
}

impl Default for ArithmeticConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_overflow: true,
            check_underflow: true,
            safe_math_required: false,
        }
    }
}

impl Default for ReentrancyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_external_calls: true,
            check_state_changes: true,
            require_checks_effect: true,
        }
    }
}

impl Default for TokenSafetyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_erc20: true,
            check_erc721: true,
            check_approve_patterns: true,
        }
    }
}

impl Default for StateManagementConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_uninitialized_state: true,
            check_race_conditions: true,
            check_atomic_operations: true,
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Low => write!(f, "LOW"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::High => write!(f, "HIGH"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl ScannerConfig {
    pub fn from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: ScannerConfig = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse config file: {}", e))?;
        Ok(config)
    }

    pub fn should_include_file(&self, file_path: &str) -> bool {
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Check exclude patterns first
        for pattern in &self.exclude_patterns {
            if self.matches_pattern(file_name, pattern) {
                return false;
            }
        }

        // Then check include patterns
        for pattern in &self.include_patterns {
            if self.matches_pattern(file_name, pattern) {
                return true;
            }
        }

        false
    }

    fn matches_pattern(&self, name: &str, pattern: &str) -> bool {
        if pattern.contains('*') {
            let pattern_regex = pattern
                .replace('*', ".*")
                .replace('?', ".");
            regex::Regex::new(&format!("^{}$", pattern_regex))
                .map(|re| re.is_match(name))
                .unwrap_or(false)
        } else {
            name == pattern
        }
    }
}
