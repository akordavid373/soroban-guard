use crate::config::{ReportFormat, Severity};
use crate::rules::RuleResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub scan_metadata: ScanMetadata,
    pub scanned_files: Vec<ScannedFile>,
    pub issues: Vec<RuleResult>,
    pub summary: ReportSummary,
    pub scan_duration: Duration,
    pub total_files_scanned: usize,
    pub total_issues_found: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanMetadata {
    pub scanner_version: String,
    pub scan_timestamp: String,
    pub config_used: String,
    pub rules_enabled: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedFile {
    pub path: String,
    pub functions_count: usize,
    pub structs_count: usize,
    pub enums_count: usize,
    pub scan_duration: Duration,
    pub issues_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub issues_by_severity: HashMap<Severity, usize>,
    pub issues_by_file: HashMap<String, usize>,
    pub issues_by_rule: HashMap<String, usize>,
    pub most_vulnerable_files: Vec<String>,
    pub most_common_issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSummary {
    pub name: String,
    pub description: String,
    pub severity: Severity,
    pub enabled: bool,
}

impl Report {
    pub fn new() -> Self {
        Self {
            scan_metadata: ScanMetadata {
                scanner_version: env!("CARGO_PKG_VERSION").to_string(),
                scan_timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .to_string(),
                config_used: "default".to_string(),
                rules_enabled: Vec::new(),
            },
            scanned_files: Vec::new(),
            issues: Vec::new(),
            summary: ReportSummary {
                issues_by_severity: HashMap::new(),
                issues_by_file: HashMap::new(),
                issues_by_rule: HashMap::new(),
                most_vulnerable_files: Vec::new(),
                most_common_issues: Vec::new(),
            },
            scan_duration: Duration::from_secs(0),
            total_files_scanned: 0,
            total_issues_found: 0,
        }
    }

    pub fn finalize(&mut self) {
        self.calculate_summary();
    }

    fn calculate_summary(&mut self) {
        // Clear existing summary
        self.summary.issues_by_severity.clear();
        self.summary.issues_by_file.clear();
        self.summary.issues_by_rule.clear();

        // Calculate statistics
        for issue in &self.issues {
            // Count by severity
            *self.summary.issues_by_severity.entry(issue.severity).or_insert(0) += 1;

            // Count by file
            *self.summary.issues_by_file.entry(issue.location.file_path.clone()).or_insert(0) += 1;

            // Count by rule
            *self.summary.issues_by_rule.entry(issue.rule_name.clone()).or_insert(0) += 1;
        }

        // Find most vulnerable files
        let mut file_issues: Vec<(String, usize)> = self.summary.issues_by_file
            .iter()
            .map(|(file, count)| (file.clone(), *count))
            .collect();
        file_issues.sort_by(|a, b| b.1.cmp(&a.1));
        self.summary.most_vulnerable_files = file_issues
            .into_iter()
            .take(10)
            .map(|(file, _)| file)
            .collect();

        // Find most common issues
        let mut rule_issues: Vec<(String, usize)> = self.summary.issues_by_rule
            .iter()
            .map(|(rule, count)| (rule.clone(), *count))
            .collect();
        rule_issues.sort_by(|a, b| b.1.cmp(&a.1));
        self.summary.most_common_issues = rule_issues
            .into_iter()
            .take(10)
            .map(|(rule, _)| rule)
            .collect();
    }

    pub fn format(&self, format: &ReportFormat) -> Result<String, Box<dyn std::error::Error>> {
        match format {
            ReportFormat::Console => self.format_console(),
            ReportFormat::Json => self.format_json(),
            ReportFormat::Html => self.format_html(),
            ReportFormat::Sarif => self.format_sarif(),
        }
    }

    fn format_console(&self) -> Result<String, Box<dyn std::error::Error>> {
        use colored::*;
        
        let mut output = String::new();

        // Header
        output.push_str(&format!(
            "{}\n",
            "🛡️  Soroban Security Guard Report".bold().cyan()
        ));
        output.push_str(&format!("{}\n", "=".repeat(60).dimmed()));
        output.push_str(&format!("Scanner Version: {}\n", self.scan_metadata.scanner_version));
        output.push_str(&format!("Scan Timestamp: {}\n", self.scan_metadata.scan_timestamp));
        output.push_str(&format!("Scan Duration: {:.2}s\n", self.scan_duration.as_secs_f64()));
        output.push_str(&format!("Files Scanned: {}\n", self.total_files_scanned));
        output.push_str(&format!("Issues Found: {}\n\n", self.total_issues_found));

        // Summary by severity
        if !self.summary.issues_by_severity.is_empty() {
            output.push_str(&format!("{}\n", "Issues by Severity:".bold()));
            for severity in [Severity::Critical, Severity::High, Severity::Medium, Severity::Low] {
                if let Some(count) = self.summary.issues_by_severity.get(&severity) {
                    let colored_severity = match severity {
                        Severity::Critical => severity.to_string().red().bold(),
                        Severity::High => severity.to_string().red(),
                        Severity::Medium => severity.to_string().yellow(),
                        Severity::Low => severity.to_string().green(),
                    };
                    output.push_str(&format!("  {}: {}\n", colored_severity, count));
                }
            }
            output.push('\n');
        }

        // Issues list
        if !self.issues.is_empty() {
            output.push_str(&format!("{}\n", "Security Issues:".bold().red()));
            output.push_str(&format!("{}\n", "-".repeat(60).dimmed()));

            for (i, issue) in self.issues.iter().enumerate() {
                let severity_color = match issue.severity {
                    Severity::Critical => issue.severity.to_string().red().bold(),
                    Severity::High => issue.severity.to_string().red(),
                    Severity::Medium => issue.severity.to_string().yellow(),
                    Severity::Low => issue.severity.to_string().green(),
                };

                output.push_str(&format!(
                    "\n{}. [{}] {}\n",
                    i + 1,
                    severity_color,
                    issue.rule_name.bold()
                ));
                output.push_str(&format!("   Severity: {}\n", issue.severity));
                output.push_str(&format!("   Location: {}:{}:{}\n", 
                    issue.location.file_path, 
                    issue.location.line, 
                    issue.location.column
                ));
                output.push_str(&format!("   Message: {}\n", issue.message));
                
                if let Some(function) = &issue.location.function {
                    output.push_str(&format!("   Function: {}\n", function));
                }

                if let Some(suggestion) = &issue.suggestion {
                    output.push_str(&format!("   Suggestion: {}\n", suggestion.green()));
                }

                output.push_str(&format!("   Confidence: {:.1}%\n", issue.confidence * 100.0));
            }
        } else {
            output.push_str(&format!("{}\n", "🎉 No security issues found!".green().bold()));
        }

        // Most vulnerable files
        if !self.summary.most_vulnerable_files.is_empty() {
            output.push_str(&format!("\n{}\n", "Most Vulnerable Files:".bold().yellow()));
            for (i, file) in self.summary.most_vulnerable_files.iter().take(5).enumerate() {
                let count = self.summary.issues_by_file.get(file).unwrap_or(&0);
                output.push_str(&format!("  {}. {} ({} issues)\n", i + 1, file, count));
            }
        }

        Ok(output)
    }

    fn format_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    fn format_html(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut html = String::new();

        // HTML header
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<title>Soroban Security Guard Report</title>\n");
        html.push_str("<style>\n");
        html.push_str(include_str!("../templates/report.css"));
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");

        // Report header
        html.push_str("<div class=\"header\">\n");
        html.push_str("<h1>🛡️ Soroban Security Guard Report</h1>\n");
        html.push_str(&format!("<p>Scanner Version: {}</p>\n", self.scan_metadata.scanner_version));
        html.push_str(&format!("<p>Scan Timestamp: {}</p>\n", self.scan_metadata.scan_timestamp));
        html.push_str(&format!("<p>Scan Duration: {:.2}s</p>\n", self.scan_duration.as_secs_f64()));
        html.push_str(&format!("<p>Files Scanned: {}</p>\n", self.total_files_scanned));
        html.push_str(&format!("<p>Issues Found: {}</p>\n", self.total_issues_found));
        html.push_str("</div>\n");

        // Summary section
        html.push_str("<div class=\"summary\">\n");
        html.push_str("<h2>Summary</h2>\n");

        if !self.summary.issues_by_severity.is_empty() {
            html.push_str("<h3>Issues by Severity</h3>\n");
            html.push_str("<ul>\n");
            for (severity, count) in &self.summary.issues_by_severity {
                let class = match severity {
                    Severity::Critical => "critical",
                    Severity::High => "high",
                    Severity::Medium => "medium",
                    Severity::Low => "low",
                };
                html.push_str(&format!(
                    "<li class=\"{}\">{}: {}</li>\n",
                    class, severity, count
                ));
            }
            html.push_str("</ul>\n");
        }

        html.push_str("</div>\n");

        // Issues section
        if !self.issues.is_empty() {
            html.push_str("<div class=\"issues\">\n");
            html.push_str("<h2>Security Issues</h2>\n");

            for (i, issue) in self.issues.iter().enumerate() {
                let severity_class = match issue.severity {
                    Severity::Critical => "critical",
                    Severity::High => "high",
                    Severity::Medium => "medium",
                    Severity::Low => "low",
                };

                html.push_str(&format!(
                    "<div class=\"issue {}\">\n",
                    severity_class
                ));
                html.push_str(&format!("<h3>{}. {}</h3>\n", i + 1, issue.rule_name));
                html.push_str(&format!("<p><strong>Severity:</strong> {}</p>\n", issue.severity));
                html.push_str(&format!(
                    "<p><strong>Location:</strong> {}:{}:{}</p>\n",
                    issue.location.file_path, issue.location.line, issue.location.column
                ));
                html.push_str(&format!("<p><strong>Message:</strong> {}</p>\n", issue.message));

                if let Some(function) = &issue.location.function {
                    html.push_str(&format!("<p><strong>Function:</strong> {}</p>\n", function));
                }

                if let Some(suggestion) = &issue.suggestion {
                    html.push_str(&format!(
                        "<p><strong>Suggestion:</strong> {}</p>\n",
                        suggestion
                    ));
                }

                html.push_str(&format!(
                    "<p><strong>Confidence:</strong> {:.1}%</p>\n",
                    issue.confidence * 100.0
                ));

                html.push_str("</div>\n");
            }

            html.push_str("</div>\n");
        } else {
            html.push_str("<div class=\"no-issues\">\n");
            html.push_str("<h2>🎉 No security issues found!</h2>\n");
            html.push_str("</div>\n");
        }

        // HTML footer
        html.push_str("</body>\n</html>");

        Ok(html)
    }

    fn format_sarif(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sarif = serde_json::Map::new();

        // SARIF version and schema
        sarif.insert("$schema".to_string(), 
            serde_json::Value::String("https://json.schemastore.org/sarif-2.1.0".to_string()));
        sarif.insert("version".to_string(), serde_json::Value::String("2.1.0".to_string()));

        // Runs array
        let mut runs = Vec::new();
        let mut run = serde_json::Map::new();

        // Tool information
        let mut tool = serde_json::Map::new();
        let mut driver = serde_json::Map::new();
        driver.insert("name".to_string(), serde_json::Value::String("soroban-security-guard".to_string()));
        driver.insert("version".to_string(), serde_json::Value::String(self.scan_metadata.scanner_version.clone()));
        driver.insert("informationUri".to_string(), 
            serde_json::Value::String("https://github.com/soroban-security-guard/soroban-security-guard".to_string()));
        tool.insert("driver".to_string(), serde_json::Value::Object(driver));
        run.insert("tool".to_string(), serde_json::Value::Object(tool));

        // Results
        let mut results = Vec::new();
        for issue in &self.issues {
            let mut result = serde_json::Map::new();
            
            // Rule ID
            result.insert("ruleId".to_string(), serde_json::Value::String(issue.rule_name.clone()));
            
            // Level
            let level = match issue.severity {
                Severity::Critical => "error",
                Severity::High => "error", 
                Severity::Medium => "warning",
                Severity::Low => "note",
            };
            result.insert("level".to_string(), serde_json::Value::String(level.to_string()));
            
            // Message
            let mut message = serde_json::Map::new();
            message.insert("text".to_string(), serde_json::Value::String(issue.message.clone()));
            result.insert("message".to_string(), serde_json::Value::Object(message));
            
            // Locations
            let mut locations = Vec::new();
            let mut location = serde_json::Map::new();
            let mut physical_location = serde_json::Map::new();
            let mut artifact_location = serde_json::Map::new();
            artifact_location.insert("uri".to_string(), 
                serde_json::Value::String(issue.location.file_path.clone()));
            physical_location.insert("artifactLocation".to_string(), 
                serde_json::Value::Object(artifact_location));
            
            let mut region = serde_json::Map::new();
            region.insert("startLine".to_string(), 
                serde_json::Value::Number(serde_json::Number::from(issue.location.line as u64)));
            region.insert("startColumn".to_string(), 
                serde_json::Value::Number(serde_json::Number::from(issue.location.column as u64)));
            physical_location.insert("region".to_string(), serde_json::Value::Object(region));
            
            location.insert("physicalLocation".to_string(), 
                serde_json::Value::Object(physical_location));
            locations.push(serde_json::Value::Object(location));
            result.insert("locations".to_string(), serde_json::Value::Array(locations));
            
            results.push(serde_json::Value::Object(result));
        }
        
        run.insert("results".to_string(), serde_json::Value::Array(results));
        runs.push(serde_json::Value::Object(run));
        sarif.insert("runs".to_string(), serde_json::Value::Array(runs));

        Ok(serde_json::to_string_pretty(&serde_json::Value::Object(sarif))?)
    }
}

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}
