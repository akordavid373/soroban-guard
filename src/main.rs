use anyhow::Result;
use clap::Parser;
use colored::*;
use soroban_security_guard::{cli::Cli, config::{ScannerConfig, Severity, ReportFormat}, SecurityScanner};
use std::fs;
use std::io::{self, Write};

fn main() -> Result<()> {
    let cli = Cli::parse_args();
    
    match cli.command {
        soroban_security_guard::cli::Commands::Scan { 
            path, 
            exclude, 
            include, 
            max_depth 
        } => {
            handle_scan_command(cli, path, exclude, include, max_depth)?;
        }
        soroban_security_guard::cli::Commands::ListRules { 
            severity, 
            show_disabled 
        } => {
            handle_list_rules_command(cli, severity, show_disabled)?;
        }
        soroban_security_guard::cli::Commands::InitConfig { 
            output, 
            strict 
        } => {
            handle_init_config_command(output, strict)?;
        }
        soroban_security_guard::cli::Commands::ValidateConfig { 
            config 
        } => {
            handle_validate_config_command(config)?;
        }
        soroban_security_guard::cli::Commands::Version { 
            detailed 
        } => {
            handle_version_command(detailed)?;
        }
    }

    Ok(())
}

fn handle_scan_command(
    cli: Cli,
    path: std::path::PathBuf,
    exclude: Vec<String>,
    include: Vec<String>,
    max_depth: Option<usize>,
) -> Result<()> {
    if cli.verbose {
        eprintln!("{} Starting security scan...", "🔍".blue());
    }

    // Load configuration
    let mut config = if let Some(config_path) = &cli.config {
        if cli.verbose {
            eprintln!("{} Loading configuration from: {:?}", "📋".blue(), config_path);
        }
        ScannerConfig::from_file(config_path)?
    } else {
        ScannerConfig::default()
    };

    // Override config with CLI arguments
    if !exclude.is_empty() {
        config.exclude_patterns.extend(exclude);
    }
    if !include.is_empty() {
        config.include_patterns.extend(include);
    }
    if let Some(depth) = max_depth {
        config.max_depth = Some(depth);
    }
    if let Ok(severity) = cli.severity.parse::<Severity>() {
        config.severity_threshold = severity;
    }
    if let Ok(format) = cli.output.parse::<ReportFormat>() {
        config.output_format = format;
    }

    // Validate path exists
    if !path.exists() {
        anyhow::bail!("Path does not exist: {:?}", path);
    }

    if cli.verbose {
        eprintln!("{} Scanning: {:?}", "📁".blue(), path);
        eprintln!("{} Severity threshold: {}", "⚠️".yellow(), config.severity_threshold);
        eprintln!("{} Output format: {:?}", "📄".blue(), config.output_format);
    }

    // Run scan
    let scanner = SecurityScanner::new(config);
    let mut report = scanner.scan(path.to_str().unwrap())?;
    report.finalize();

    // Format and output report
    let formatted_report = report.format(&config.output_format)?;
    
    if let Some(output_file) = &cli.output_file {
        fs::write(output_file, formatted_report)?;
        println!("{} Report saved to: {:?}", "✅".green(), output_file);
    } else {
        print!("{}", formatted_report);
    }

    // Exit with error code if critical/high issues found
    let critical_count = report.summary.issues_by_severity.get(&Severity::Critical).unwrap_or(&0);
    let high_count = report.summary.issues_by_severity.get(&Severity::High).unwrap_or(&0);
    
    if *critical_count > 0 || *high_count > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn handle_list_rules_command(
    cli: Cli,
    severity_filter: Option<String>,
    show_disabled: bool,
) -> Result<()> {
    let config = if let Some(config_path) = &cli.config {
        ScannerConfig::from_file(config_path)?
    } else {
        ScannerConfig::default()
    };

    let scanner = SecurityScanner::new(config);
    let rules = scanner.get_rules_summary();

    println!("{}", "🛡️  Available Security Rules".bold().cyan());
    println!("{}", "=".repeat(50).dimmed());

    for rule in rules {
        let should_show = show_disabled || rule.enabled;
        let severity_match = if let Some(filter) = &severity_filter {
            if let Ok(filter_severity) = filter.parse::<Severity>() {
                rule.severity == filter_severity
            } else {
                false
            }
        } else {
            true
        };

        if should_show && severity_match {
            let status = if rule.enabled { "✅" } else { "❌" };
            let severity_color = match rule.severity {
                Severity::Critical => rule.severity.to_string().red().bold(),
                Severity::High => rule.severity.to_string().red(),
                Severity::Medium => rule.severity.to_string().yellow(),
                Severity::Low => rule.severity.to_string().green(),
            };

            println!("\n{} {}", status, rule.name.bold());
            println!("  Severity: {}", severity_color);
            println!("  Description: {}", rule.description);
            println!("  Status: {}", if rule.enabled { "Enabled" } else { "Disabled" });
        }
    }

    Ok(())
}

fn handle_init_config_command(output_path: std::path::PathBuf, strict: bool) -> Result<()> {
    let mut config = if strict {
        ScannerConfig {
            severity_threshold: Severity::Low,
            rules: soroban_security_guard::config::RulesConfig {
                access_control: soroban_security_guard::config::AccessControlConfig {
                    strict_mode: true,
                    ..Default::default()
                },
                arithmetic: soroban_security_guard::config::ArithmeticConfig {
                    safe_math_required: true,
                    ..Default::default()
                },
                reentrancy: soroban_security_guard::config::ReentrancyConfig {
                    require_checks_effect: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    } else {
        ScannerConfig::default()
    };

    // Add some example custom rules
    config.rules.custom_rules.push(soroban_security_guard::config::CustomRule {
        name: "hardcoded_address".to_string(),
        pattern: r#"0x[a-fA-F0-9]{40}"#.to_string(),
        severity: Severity::Medium,
        description: "Potential hardcoded address detected".to_string(),
        enabled: true,
    });

    config.rules.custom_rules.push(soroban_security_guard::config::CustomRule {
        name: "debug_statement".to_string(),
        pattern: r"println!|dbg!|eprintln!".to_string(),
        severity: Severity::Low,
        description: "Debug statement found in production code".to_string(),
        enabled: true,
    });

    let config_toml = toml::to_string_pretty(&config)?;
    
    if output_path.exists() {
        print!("File {:?} already exists. Overwrite? [y/N]: ", output_path);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    fs::write(&output_path, config_toml)?;
    println!("{} Configuration file created: {:?}", "✅".green(), output_path);
    
    if strict {
        println!("{} Strict configuration enabled - all security checks will be most thorough", "⚠️".yellow());
    }

    Ok(())
}

fn handle_validate_config_command(config_path: std::path::PathBuf) -> Result<()> {
    println!("{} Validating configuration file: {:?}", "🔍".blue(), config_path);
    
    if !config_path.exists() {
        anyhow::bail!("Configuration file does not exist: {:?}", config_path);
    }

    match ScannerConfig::from_file(&config_path) {
        Ok(config) => {
            println!("{} Configuration is valid!", "✅".green());
            
            // Show configuration summary
            println!("\n{}", "Configuration Summary:".bold());
            println!("  Severity threshold: {}", config.severity_threshold);
            println!("  Output format: {:?}", config.output_format);
            println!("  Exclude patterns: {}", config.exclude_patterns.join(", "));
            println!("  Include patterns: {}", config.include_patterns.join(", "));
            println!("  Max depth: {:?}", config.max_depth);
            
            println!("\n{}", "Enabled Rule Categories:".bold());
            println!("  Access Control: {}", if config.rules.access_control.enabled { "✅" } else { "❌" });
            println!("  Arithmetic: {}", if config.rules.arithmetic.enabled { "✅" } else { "❌" });
            println!("  Reentrancy: {}", if config.rules.reentrancy.enabled { "✅" } else { "❌" });
            println!("  Token Safety: {}", if config.rules.token_safety.enabled { "✅" } else { "❌" });
            println!("  State Management: {}", if config.rules.state_management.enabled { "✅" } else { "❌" });
            println!("  Custom Rules: {}", config.rules.custom_rules.len());
        }
        Err(e) => {
            println!("{} Configuration validation failed!", "❌".red());
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn handle_version_command(detailed: bool) -> Result<()> {
    println!("soroban-security-guard {}", env!("CARGO_PKG_VERSION"));
    
    if detailed {
        println!("\n{}", "Build Information:".bold());
        println!("  Version: {}", env!("CARGO_PKG_VERSION"));
        println!("  Authors: {}", env!("CARGO_PKG_AUTHORS"));
        println!("  Description: {}", env!("CARGO_PKG_DESCRIPTION"));
        println!("  License: {}", env!("CARGO_PKG_LICENSE"));
        println!("  Repository: {}", env!("CARGO_PKG_REPOSITORY"));
        
        println!("\n{}", "Dependencies:".bold());
        println!("  soroban-sdk: 20.0.0");
        println!("  serde: 1.0");
        println!("  clap: 4.0");
        println!("  tokio: 1.0");
        
        println!("\n{}", "Features:".bold());
        println!("  ✅ Static analysis of Soroban contracts");
        println!("  ✅ Multiple output formats (console, JSON, HTML, SARIF)");
        println!("  ✅ Custom rule support");
        println!("  ✅ Configurable severity thresholds");
        println!("  ✅ Detailed security reports");
    }

    Ok(())
}
