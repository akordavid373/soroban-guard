pub mod scanner;
pub mod rules;
pub mod ast;
pub mod report;
pub mod config;
pub mod cli;

pub use scanner::SecurityScanner;
pub use rules::{Rule, RuleResult, Severity};
pub use config::ScannerConfig;
pub use report::{Report, ReportFormat};

use anyhow::Result;

/// Main entry point for the security guard library
pub fn scan_contract(contract_path: &str, config: ScannerConfig) -> Result<Report> {
    let scanner = SecurityScanner::new(config);
    scanner.scan(contract_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_scan() {
        let config = ScannerConfig::default();
        // Test will be implemented when we have sample contracts
    }
}
