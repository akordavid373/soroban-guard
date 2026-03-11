use crate::ast::ContractAst;
use crate::config::ScannerConfig;
use crate::report::Report;
use crate::rules::{Rule, RuleResult};
use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

pub struct SecurityScanner {
    config: ScannerConfig,
    rules: Vec<Box<dyn Rule>>,
}

impl SecurityScanner {
    pub fn new(config: ScannerConfig) -> Self {
        let mut rules: Vec<Box<dyn Rule>> = Vec::new();

        // Add built-in rules
        rules.push(Box::new(crate::rules::AdminOnlyFunctionRule));
        rules.push(Box::new(crate::rules::OwnerOnlyFunctionRule));
        rules.push(Box::new(crate::rules::PotentialOverflowRule));
        rules.push(Box::new(crate::rules::ReentrancyPatternRule));

        // Add custom rules
        for custom_rule in &config.rules.custom_rules {
            rules.push(Box::new(crate::rules::CustomRuleAdapter::new(
                custom_rule.clone(),
            )));
        }

        Self { config, rules }
    }

    pub fn scan(&self, path: &str) -> Result<Report> {
        let mut report = Report::new();
        let scan_start_time = std::time::Instant::now();

        let path_obj = Path::new(path);
        if path_obj.is_file() {
            self.scan_file(path, &mut report)?;
        } else if path_obj.is_dir() {
            self.scan_directory(path, &mut report)?;
        } else {
            anyhow::bail!("Path is neither a file nor a directory: {}", path);
        }

        report.scan_duration = scan_start_time.elapsed();
        report.total_files_scanned = report.scanned_files.len();
        report.total_issues_found = report.issues.len();

        Ok(report)
    }

    fn scan_directory(&self, dir_path: &str, report: &mut Report) -> Result<()> {
        let walker = WalkDir::new(dir_path)
            .max_depth(self.config.max_depth.unwrap_or(10))
            .into_iter();

        for entry in walker.filter_entry(|e| {
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        }) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(path_str) = path.to_str() {
                    if self.config.should_include_file(path_str) {
                        if let Err(e) = self.scan_file(path_str, report) {
                            eprintln!("Error scanning file {}: {}", path_str, e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn scan_file(&self, file_path: &str, report: &mut Report) -> Result<()> {
        let file_start_time = std::time::Instant::now();
        
        // Parse the file
        let ast = crate::ast::AstParser::parse_file(file_path)?;
        let source_code = std::fs::read_to_string(file_path)?;

        // Create rule context
        let context = crate::rules::RuleContext {
            ast: ast.clone(),
            file_path: file_path.to_string(),
            source_code: source_code.clone(),
            imports: ast.imports.clone(),
            functions: ast.functions.clone(),
            structs: ast.structs.clone(),
            enums: ast.enums.clone(),
        };

        // Run enabled rules
        let mut file_issues = Vec::new();
        for rule in &self.rules {
            if rule.is_enabled(&self.config) {
                let rule_results = rule.check(&context);
                for result in rule_results {
                    if result.severity >= self.config.severity_threshold {
                        file_issues.push(result);
                    }
                }
            }
        }

        // Add to report
        report.scanned_files.push(crate::report::ScannedFile {
            path: file_path.to_string(),
            functions_count: ast.functions.len(),
            structs_count: ast.structs.len(),
            enums_count: ast.enums.len(),
            scan_duration: file_start_time.elapsed(),
            issues_count: file_issues.len(),
        });

        report.issues.extend(file_issues);

        Ok(())
    }

    pub fn get_rules_summary(&self) -> Vec<crate::report::RuleSummary> {
        self.rules
            .iter()
            .map(|rule| crate::report::RuleSummary {
                name: rule.name().to_string(),
                description: rule.description().to_string(),
                severity: rule.severity(),
                enabled: true, // We only store enabled rules
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Severity, ScannerConfig};

    #[test]
    fn test_scanner_creation() {
        let config = ScannerConfig::default();
        let scanner = SecurityScanner::new(config);
        assert!(!scanner.rules.is_empty());
    }

    #[test]
    fn test_scan_nonexistent_file() {
        let config = ScannerConfig::default();
        let scanner = SecurityScanner::new(config);
        let result = scanner.scan("nonexistent.rs");
        assert!(result.is_err());
    }
}
