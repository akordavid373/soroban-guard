---
title: Create VS Code extension for real-time security analysis
labels: enhancement, vscode, ide, integration
assignees: []
---

## 💻 Enhancement Description

Develop a Visual Studio Code extension that provides real-time security analysis and feedback for Soroban smart contracts during development. This will integrate security scanning directly into the development workflow.

## 📁 Files to Modify

### Primary Files
```
📄 vscode/ (CREATE NEW DIRECTORY)
├── 📄 vscode/src/extension.ts (CREATE NEW)
├── 📄 vscode/src/commands.ts (CREATE NEW)
├── 📄 vscode/src/diagnostics.ts (CREATE NEW)
├── 📄 vscode/src/statusBar.ts (CREATE NEW)
├── 📄 vscode/src/config.ts (CREATE NEW)
├── 📄 vscode/src/scanner.ts (CREATE NEW)
├── 📄 vscode/package.json (CREATE NEW)
├── 📄 vscode/tsconfig.json (CREATE NEW)
├── 📄 vscode/webpack.config.js (CREATE NEW)
└── 📄 vscode/README.md (CREATE NEW)
```

### Secondary Files
```
📄 src/vscode.rs (CREATE NEW - VS Code specific interface)
📄 Cargo.toml (add VS Code dependencies)
📄 scripts/build-vscode.sh (CREATE NEW)
📄 scripts/package-vscode.sh (CREATE NEW)
```

## 🎯 Acceptance Criteria

### ✅ MUST HAVE (High Priority)
- [ ] **vscode/package.json** - Extension manifest and configuration
- [ ] **vscode/src/extension.ts** - Main extension entry point
- [ ] **vscode/src/scanner.ts** - Core scanning integration
- [ ] **vscode/src/diagnostics.ts** - Real-time vulnerability highlighting
- [ ] **vscode/src/commands.ts** - VS Code command registration
- [ ] **src/vscode.rs** - Rust interface for VS Code integration

### ✅ SHOULD HAVE (Medium Priority)
- [ ] **vscode/src/statusBar.ts** - Status bar integration
- [ ] **vscode/src/config.ts** - Extension configuration management
- [ ] **vscode/webpack.config.js** - Build configuration
- [ ] **scripts/build-vscode.sh** - Build automation script

### ✅ COULD HAVE (Low Priority)
- [ ] **vscode/README.md** - Extension documentation
- [ ] **scripts/package-vscode.sh** - Packaging script
- [ ] **vscode/src/webview/** - Optional webview interface

## 🔧 Implementation Details

### 1. vscode/package.json (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\vscode\package.json`

**Complete Content**:
```json
{
  "name": "soroban-security-guard",
  "displayName": "Soroban Security Guard",
  "description": "Real-time security analysis for Soroban smart contracts",
  "version": "0.1.0",
  "publisher": "soroban-security-guard",
  "engines": {
    "vscode": "^1.82.0"
  },
  "categories": [
    "Linters",
    "Programming Languages",
    "Security"
  ],
  "keywords": [
    "soroban",
    "stellar",
    "blockchain",
    "security",
    "smart contracts",
    "rust"
  ],
  "activationEvents": [
    "onLanguage:rust",
    "onCommand:soroban-security-guard.scanFile",
    "onCommand:soroban-security-guard.scanWorkspace",
    "onCommand:soroban-security-guard.configure"
  ],
  "main": "./dist/extension.js",
  "contributes": {
    "commands": [
      {
        "command": "soroban-security-guard.scanFile",
        "title": "Scan Current File",
        "category": "Soroban Security Guard"
      },
      {
        "command": "soroban-security-guard.scanWorkspace",
        "title": "Scan Workspace",
        "category": "Soroban Security Guard"
      },
      {
        "command": "soroban-security-guard.configure",
        "title": "Configure",
        "category": "Soroban Security Guard"
      },
      {
        "command": "soroban-security-guard.clearDiagnostics",
        "title": "Clear Diagnostics",
        "category": "Soroban Security Guard"
      },
      {
        "command": "soroban-security-guard.showReport",
        "title": "Show Security Report",
        "category": "Soroban Security Guard"
      }
    ],
    "configuration": {
      "title": "Soroban Security Guard",
      "properties": {
        "soroban-security-guard.enabled": {
          "type": "boolean",
          "default": true,
          "description": "Enable real-time security analysis"
        },
        "soroban-security-guard.severityThreshold": {
          "type": "string",
          "enum": ["low", "medium", "high", "critical"],
          "default": "low",
          "description": "Minimum severity level to report"
        },
        "soroban-security-guard.autoScan": {
          "type": "boolean",
          "default": true,
          "description": "Automatically scan files on save"
        },
        "soroban-security-guard.enabledRules": {
          "type": "array",
          "default": [],
          "description": "List of enabled rule IDs (empty = all enabled)"
        },
        "soroban-security-guard.customRules": {
          "type": "array",
          "default": [],
          "description": "Custom security rules"
        }
      }
    },
    "languages": [
      {
        "id": "rust",
        "aliases": ["rust", "rs"],
        "extensions": [".rs"],
        "configuration": "./language-configuration.json"
      }
    ],
    "grammars": [
      {
        "language": "rust",
        "scopeName": "source.rust",
        "path": "./syntaxes/soroban.tmLanguage.json"
      }
    ]
  },
  "scripts": {
    "vscode:prepublish": "npm run package",
    "vscode:package": "webpack --mode production --devtool hidden-source-map",
    "vscode:compile": "webpack --mode development",
    "vscode:watch": "webpack --mode development --watch",
    "pretest": "npm run compile && npm run lint",
    "lint": "eslint src --ext .ts",
    "test": "node ./out/test/runTest.js"
  },
  "devDependencies": {
    "@types/vscode": "^1.82.0",
    "@types/node": "18.18.5",
    "eslint": "^8.51.0",
    "typescript": "^5.2.2",
    "webpack": "^5.89.0",
    "ts-loader": "^9.5.0",
    "webpack-cli": "^5.1.4",
    "@vscode/test-electron": "^2.3.6"
  },
  "dependencies": {
    "vscode-languageclient": "^8.1.0"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/akordavid373/soroban-guard.git"
  },
  "license": "MIT",
  "icon": "icon.png"
}
```

### 2. vscode/src/extension.ts (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\vscode\src\extension.ts`

**Complete Content**:
```typescript
import * as vscode from 'vscode';
import { ScannerService } from './scanner';
import { registerCommands } from './commands';
import { setupDiagnostics } from './diagnostics';
import { setupStatusBar } from './statusBar';
import { ExtensionConfig } from './config';

let scannerService: ScannerService;
let config: ExtensionConfig;

export function activate(context: vscode.ExtensionContext) {
    console.log('Soroban Security Guard extension is now active!');

    // Initialize configuration
    config = new ExtensionConfig(context);

    // Initialize scanner service
    scannerService = new ScannerService(config);

    // Register commands
    registerCommands(context, scannerService);

    // Setup diagnostics
    setupDiagnostics(context, scannerService);

    // Setup status bar
    setupStatusBar(context, scannerService);

    // Setup file watchers
    setupFileWatchers(context, scannerService);

    // Setup language features
    setupLanguageFeatures(context, scannerService);

    // Show welcome message on first activation
    showWelcomeMessage(context);
}

function setupFileWatchers(context: vscode.ExtensionContext, scanner: ScannerService) {
    if (!config.getAutoScan()) {
        return;
    }

    // Watch for file changes
    const fileWatcher = vscode.workspace.createFileSystemWatcher(
        '**/*.rs',
        (uri) => {
            if (uri.scheme === 'file') {
                scanFile(uri);
            }
        }
    );

    // Watch for file saves
    const saveWatcher = vscode.workspace.onDidSaveTextDocument((document) => {
        if (document.languageId === 'rust' && config.getAutoScan()) {
            scanner.scanDocument(document);
        }
    });

    context.subscriptions.push(fileWatcher, saveWatcher);
}

function setupLanguageFeatures(context: vscode.ExtensionContext, scanner: ScannerService) {
    // Register completion provider for security-related keywords
    const completionProvider = vscode.languages.registerCompletionItemProvider(
        'rust',
        new SecurityCompletionProvider(scanner)
    );

    // Register hover provider for vulnerability information
    const hoverProvider = vscode.languages.registerHoverProvider(
        'rust',
        new SecurityHoverProvider(scanner)
    );

    // Register code actions for quick fixes
    const codeActionProvider = vscode.languages.registerCodeActionsProvider(
        'rust',
        new SecurityCodeActionProvider(scanner)
    );

    context.subscriptions.push(completionProvider, hoverProvider, codeActionProvider);
}

async function scanFile(uri: vscode.Uri) {
    try {
        const document = await vscode.workspace.openTextDocument(uri);
        await scanner.scanDocument(document);
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to scan file: ${error}`);
    }
}

function showWelcomeMessage(context: vscode.ExtensionContext) {
    const shown = context.globalState.get('welcomeShown');
    if (!shown) {
        vscode.window.showInformationMessage(
            'Welcome to Soroban Security Guard! 🛡️',
            'Get Started',
            'Learn More'
        ).then(selection => {
            if (selection === 'Get Started') {
                vscode.commands.executeCommand('soroban-security-guard.scanWorkspace');
            } else if (selection === 'Learn More') {
                vscode.env.openExternal(
                    vscode.Uri.parse('https://github.com/akordavid373/soroban-guard')
                );
            }
        });
        context.globalState.update('welcomeShown', true);
    }
}

export function deactivate() {
    console.log('Soroban Security Guard extension deactivated');
}

// Language feature providers
class SecurityCompletionProvider implements vscode.CompletionItemProvider {
    constructor(private scanner: ScannerService) {}

    provideCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position
    ): vscode.CompletionItem[] {
        const securityKeywords = [
            'require', 'assert_eq', 'panic', 'unwrap', 'expect',
            'env.storage()', 'env.events()', 'env.invoke_contract()'
        ];

        const linePrefix = document.getText(
            new vscode.Range(
                new vscode.Position(position.line, 0),
                position
            )
        );

        return securityKeywords
            .filter(keyword => keyword.startsWith(linePrefix))
            .map(keyword => {
                const item = new vscode.CompletionItem(
                    keyword,
                    vscode.CompletionItemKind.Keyword
                );
                item.documentation = new vscode.MarkdownString(
                    `**${keyword}**\n\nSecurity consideration for ${keyword} usage.`
                );
                return item;
            });
    }
}

class SecurityHoverProvider implements vscode.HoverProvider {
    constructor(private scanner: ScannerService) {}

    provideHover(
        document: vscode.TextDocument,
        position: vscode.Position
    ): vscode.Hover | undefined {
        const range = document.getWordRangeAtPosition(position);
        if (!range) {
            return undefined;
        }

        const word = document.getText(range);
        const diagnostics = scanner.getDiagnosticsForDocument(document.uri);
        
        const relevantDiagnostic = diagnostics.find(d => 
            d.message.includes(word) || 
            d.range.contains(range)
        );

        if (relevantDiagnostic) {
            const content = new vscode.MarkdownString();
            content.appendMarkdown(`**Security Issue: ${relevantDiagnostic.message}**\n\n`);
            content.appendMarkdown(`**Severity:** ${relevantDiagnostic.severity}\n\n`);
            content.appendMarkdown(`**Recommendation:** ${relevantDiagnostic.recommendation}`);
            
            return new vscode.Hover(content, range);
        }

        return undefined;
    }
}

class SecurityCodeActionProvider implements vscode.CodeCodeActionProvider {
    constructor(private scanner: ScannerService) {}

    provideCodeActions(
        document: vscode.TextDocument,
        range: vscode.Range
    ): vscode.CodeAction[] {
        const actions: vscode.CodeAction[] = [];
        const diagnostics = scanner.getDiagnosticsForDocument(document.uri);
        
        const relevantDiagnostics = diagnostics.filter(d => 
            d.range.contains(range) || range.contains(d.range)
        );

        for (const diagnostic of relevantDiagnostics) {
            const action = new vscode.CodeAction(
                `Fix: ${diagnostic.message}`,
                vscode.CodeActionKind.QuickFix
            );
            
            action.diagnostics = [diagnostic];
            action.edit = new vscode.WorkspaceEdit();
            
            // Apply quick fix based on diagnostic type
            if (diagnostic.code.includes('missing_access_control')) {
                action.edit.replace(
                    document.uri,
                    range,
                    `require!(env.current_contract_address() == admin, "Unauthorized");\n    `
                );
            } else if (diagnostic.code.includes('integer_overflow')) {
                action.edit.replace(
                    document.uri,
                    range,
                    range.getText().replace('+', '.checked_add(')
                );
            }
            
            actions.push(action);
        }

        return actions;
    }
}
```

### 3. vscode/src/scanner.ts (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\vscode\src\scanner.ts`

**Complete Content**:
```typescript
import * as vscode from 'vscode';
import { ExtensionConfig } from './config';
import { SecurityDiagnostic } from './diagnostics';

export class ScannerService {
    private diagnostics: Map<string, SecurityDiagnostic[]> = new Map();
    private config: ExtensionConfig;
    private outputChannel: vscode.OutputChannel;

    constructor(config: ExtensionConfig) {
        this.config = config;
        this.outputChannel = vscode.window.createOutputChannel('Soroban Security Guard');
    }

    async scanDocument(document: vscode.TextDocument): Promise<void> {
        if (!this.config.isEnabled()) {
            return;
        }

        try {
            this.outputChannel.appendLine(`Scanning file: ${document.uri.fsPath}`);
            
            // Call the Rust scanner (this would need to be implemented)
            const scanResults = await this.callRustScanner(document.getText());
            
            // Convert results to VS Code diagnostics
            const vsDiagnostics = scanResults.map(result => this.convertToDiagnostic(result, document));
            
            // Store diagnostics for this document
            this.diagnostics.set(document.uri.toString(), scanResults);
            
            // Publish diagnostics
            const diagnosticCollection = vscode.languages.createDiagnosticCollection('soroban-security');
            diagnosticCollection.set(document.uri, vsDiagnostics);
            
            this.outputChannel.appendLine(`Found ${scanResults.length} security issues`);
            
        } catch (error) {
            this.outputChannel.appendLine(`Scan failed: ${error}`);
            vscode.window.showErrorMessage(`Failed to scan ${document.fileName}: ${error}`);
        }
    }

    async scanWorkspace(): Promise<void> {
        if (!this.config.isEnabled()) {
            vscode.window.showWarningMessage('Soroban Security Guard is disabled');
            return;
        }

        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            vscode.window.showInformationMessage('No workspace folder found');
            return;
        }

        const totalIssues: SecurityDiagnostic[] = [];
        const diagnosticCollection = vscode.languages.createDiagnosticCollection('soroban-security');

        for (const folder of workspaceFolders) {
            vscode.window.showInformationMessage(`Scanning workspace: ${folder.name}`);
            
            try {
                // Find all Rust files in the workspace
                const rustFiles = await vscode.workspace.findFiles(
                    '**/*.rs',
                    '**/node_modules/**',
                    { maxResults: 1000 }
                );

                this.outputChannel.appendLine(`Found ${rustFiles.length} Rust files to scan`);

                for (const file of rustFiles) {
                    try {
                        const document = await vscode.workspace.openTextDocument(file);
                        await this.scanDocument(document);
                        
                        const fileDiagnostics = this.getDiagnosticsForDocument(file);
                        totalIssues.push(...fileDiagnostics);
                        
                    } catch (error) {
                        this.outputChannel.appendLine(`Failed to scan ${file.fsPath}: ${error}`);
                    }
                }

            } catch (error) {
                this.outputChannel.appendLine(`Failed to scan workspace ${folder.name}: ${error}`);
            }
        }

        // Show summary
        this.showScanSummary(totalIssues);
    }

    private async callRustScanner(code: string): Promise<any[]> {
        // This would integrate with the Rust scanner
        // For now, return mock data
        return this.mockScanResults(code);
    }

    private mockScanResults(code: string): any[] {
        const results = [];
        
        // Simple pattern matching for demonstration
        if (code.includes('env.storage().instance().set(')) {
            results.push({
                id: 'storage-write-1',
                rule_name: 'Unvalidated Storage Write',
                severity: 'high',
                description: 'Storage write without proper validation',
                recommendation: 'Add input validation before writing to storage',
                location: { line: this.findLineNumber(code, 'env.storage().instance().set') },
                code_snippet: this.extractCodeSnippet(code, 'env.storage().instance().set'),
                rule_id: 'STORAGE_WRITE_VALIDATION',
                confidence: 0.8
            });
        }

        if (code.includes('unwrap()')) {
            results.push({
                id: 'unwrap-usage-1',
                rule_name: 'Unsafe Unwrap Usage',
                severity: 'medium',
                description: 'Using unwrap() can cause panics',
                recommendation: 'Use proper error handling or expect() with message',
                location: { line: this.findLineNumber(code, 'unwrap()') },
                code_snippet: this.extractCodeSnippet(code, 'unwrap()'),
                rule_id: 'UNSAFE_UNWRAP',
                confidence: 0.9
            });
        }

        if (code.includes('panic!')) {
            results.push({
                id: 'panic-usage-1',
                rule_name: 'Panic Usage in Production',
                severity: 'high',
                description: 'Using panic! in production code',
                recommendation: 'Replace with proper error handling',
                location: { line: this.findLineNumber(code, 'panic!') },
                code_snippet: this.extractCodeSnippet(code, 'panic!'),
                rule_id: 'PANIC_USAGE',
                confidence: 0.95
            });
        }

        return results;
    }

    private findLineNumber(code: string, pattern: string): number {
        const lines = code.split('\n');
        for (let i = 0; i < lines.length; i++) {
            if (lines[i].includes(pattern)) {
                return i + 1;
            }
        }
        return 1;
    }

    private extractCodeSnippet(code: string, pattern: string): string {
        const lines = code.split('\n');
        const targetLine = lines.find(line => line.includes(pattern));
        if (targetLine) {
            return targetLine.trim();
        }
        return '';
    }

    private convertToDiagnostic(result: any, document: vscode.TextDocument): vscode.Diagnostic {
        const line = Math.max(0, (result.location?.line || 1) - 1);
        const lineText = document.lineAt(line);
        
        const range = new vscode.Range(
            line,
            lineText.indexOf(result.code_snippet) || 0,
            line,
            (lineText.indexOf(result.code_snippet) || 0) + result.code_snippet.length
        );

        return {
            code: result.rule_id,
            message: result.rule_name,
            severity: this.convertSeverity(result.severity),
            range,
            source: 'Soroban Security Guard',
            diagnosticInformation: {
                description: result.description,
                recommendation: result.recommendation,
                confidence: result.confidence
            }
        };
    }

    private convertSeverity(severity: string): vscode.DiagnosticSeverity {
        switch (severity) {
            case 'critical': return vscode.DiagnosticSeverity.Error;
            case 'high': return vscode.DiagnosticSeverity.Error;
            case 'medium': return vscode.DiagnosticSeverity.Warning;
            case 'low': return vscode.DiagnosticSeverity.Information;
            default: return vscode.DiagnosticSeverity.Warning;
        }
    }

    private showScanSummary(issues: SecurityDiagnostic[]): void {
        const critical = issues.filter(i => i.severity === 'critical').length;
        const high = issues.filter(i => i.severity === 'high').length;
        const medium = issues.filter(i => i.severity === 'medium').length;
        const low = issues.filter(i => i.severity === 'low').length;

        const message = `Scan completed! Found ${issues.length} issues: ${critical} critical, ${high} high, ${medium} medium, ${low} low`;
        
        vscode.window.showInformationMessage(message, 'View Report').then(selection => {
            if (selection === 'View Report') {
                vscode.commands.executeCommand('soroban-security-guard.showReport');
            }
        });
    }

    getDiagnosticsForDocument(uri: vscode.Uri): SecurityDiagnostic[] {
        return this.diagnostics.get(uri.toString()) || [];
    }

    getDiagnostics(): Map<string, SecurityDiagnostic[]> {
        return this.diagnostics;
    }

    clearDiagnostics(): void {
        this.diagnostics.clear();
        const diagnosticCollection = vscode.languages.createDiagnosticCollection('soroban-security');
        diagnosticCollection.clear();
        this.outputChannel.appendLine('Cleared all diagnostics');
    }
}
```

### 4. src/vscode.rs (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\src\vscode.rs`

**Complete Content**:
```rust
use wasm_bindgen::prelude::*;
use crate::SecurityScanner;
use crate::config::ScannerConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VSCodeScanRequest {
    pub code: String,
    pub file_path: String,
    pub config: Option<VSCodeConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct VSCodeConfig {
    pub severity_threshold: String,
    pub enabled_rules: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct VSCodeScanResult {
    pub id: String,
    pub rule_name: String,
    pub severity: String,
    pub description: String,
    pub recommendation: String,
    pub location: VSCodeLocation,
    pub code_snippet: String,
    pub rule_id: String,
    pub confidence: f64,
}

#[derive(Serialize, Deserialize)]
pub struct VSCodeLocation {
    pub line: usize,
    pub column: Option<usize>,
    pub file_path: String,
}

#[wasm_bindgen]
pub struct VSCodeScanner {
    scanner: SecurityScanner,
}

#[wasm_bindgen]
impl VSCodeScanner {
    #[wasm_bindgen(constructor)]
    pub fn new(config_json: &str) -> Result<VSCodeScanner, JsValue> {
        let config: ScannerConfig = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid config: {}", e)))?;
        
        let scanner = SecurityScanner::new(config)
            .map_err(|e| JsValue::from_str(&format!("Failed to create scanner: {}", e)))?;
        
        Ok(VSCodeScanner { scanner })
    }

    #[wasm_bindgen]
    pub fn scan_code(&mut self, request_json: &str) -> Result<JsValue, JsValue> {
        let request: VSCodeScanRequest = serde_json::from_str(request_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid request: {}", e)))?;

        // Update config if provided
        if let Some(vscode_config) = request.config {
            self.update_config(vscode_config)?;
        }

        // Perform scan
        let results = self.scanner.scan_contract_code(&request.code)
            .map_err(|e| JsValue::from_str(&format!("Scan failed: {}", e)))?;

        // Convert to VS Code format
        let vscode_results: Vec<VSCodeScanResult> = results.into_iter()
            .map(|r| VSCodeScanResult {
                id: r.id,
                rule_name: r.rule_name,
                severity: format!("{:?}", r.severity).to_lowercase(),
                description: r.description,
                recommendation: r.recommendation,
                location: VSCodeLocation {
                    line: r.location.line,
                    column: Some(r.location.column),
                    file_path: request.file_path.clone(),
                },
                code_snippet: r.code_snippet.unwrap_or_default(),
                rule_id: r.rule_id,
                confidence: r.confidence,
            })
            .collect();

        let json = serde_json::to_string(&vscode_results)
            .map_err(|e| JsValue::from_str(&format!("JSON serialization failed: {}", e)))?;
        
        Ok(JsValue::from_str(&json))
    }

    #[wasm_bindgen]
    pub fn get_available_rules(&self) -> Result<JsValue, JsValue> {
        let rules = self.scanner.get_available_rules();
        let json = serde_json::to_string(&rules)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize rules: {}", e)))?;
        Ok(JsValue::from_str(&json))
    }

    #[wasm_bindgen]
    pub fn validate_rule(&self, rule_json: &str) -> Result<JsValue, JsValue> {
        // Validate custom rule
        let validation_result = serde_json::json!({
            "valid": true,
            "errors": []
        });

        let json = serde_json::to_string(&validation_result)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize validation: {}", e)))?;
        
        Ok(JsValue::from_str(&json))
    }

    fn update_config(&mut self, config: VSCodeConfig) -> Result<(), JsValue> {
        let mut scanner_config = self.scanner.config().clone();

        // Update severity threshold
        scanner_config.severity_threshold = match config.severity_threshold.as_str() {
            "low" => crate::config::Severity::Low,
            "medium" => crate::config::Severity::Medium,
            "high" => crate::config::Severity::High,
            "critical" => crate::config::Severity::Critical,
            _ => return Err(JsValue::from_str("Invalid severity threshold")),
        };

        // Update enabled rules
        scanner_config.enabled_rules = config.enabled_rules;

        // Apply new configuration
        self.scanner.update_config(scanner_config)
            .map_err(|e| JsValue::from_str(&format!("Failed to update config: {}", e)))?;

        Ok(())
    }
}

// Utility functions for VS Code integration
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn get_supported_file_types() -> JsValue {
    let file_types = vec!["rust", "rs"];
    JsValue::from_serde(&file_types).unwrap()
}
```

### 5. Cargo.toml Updates

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\Cargo.toml`

**Add to dependencies**:
```toml
[dependencies]
# ... existing dependencies ...
wasm-bindgen = { version = "0.2", features = ["serde-serialize"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "wasm-bindgen"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
```

**Add new binary target**:
```toml
[[bin]]
name = "soroban-security-guard-vscode"
path = "src/main.rs"

[[bin]]
name = "soroban-security-guard"
path = "src/main.rs"
```

### 6. scripts/build-vscode.sh (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\scripts\build-vscode.sh`

**Complete Content**:
```bash
#!/bin/bash

# Build script for VS Code extension

set -e

echo "🔧 Building Soroban Security Guard VS Code extension..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the project root directory"
    exit 1
fi

# Install wasm target if not already installed
echo "📦 Installing wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

# Build the WASM library for VS Code
echo "🔨 Building WASM library for VS Code..."
cargo build --target wasm32-unknown-unknown --release

# Generate JS bindings for VS Code
echo "🔗 Generating JavaScript bindings for VS Code..."
wasm-bindgen \
    --target web \
    --out-dir vscode/dist \
    --out-name soroban_security_guard_vscode \
    --no-typescript \
    target/wasm32-unknown-unknown/release/libsoroban_security_guard_vscode.a

# Build the VS Code extension
echo "📦 Building VS Code extension..."
cd vscode
npm install
npm run compile
cd ..

echo "✅ VS Code extension built successfully!"
echo ""
echo "📁 Build artifacts:"
echo "  - vscode/dist/ - Extension files"
echo "  - vscode/dist/soroban_security_guard_vscode.wasm - WASM module"
echo "  - vscode/dist/soroban_security_guard_vscode.js - JavaScript bindings"
echo ""
echo "🚀 To test locally:"
echo "  cd vscode && npm run watch"
echo ""
echo "📦 To package:"
echo "  cd vscode && npm run vscode:package"
echo ""
echo "🏗 To install in development:"
echo "  cd vscode && npm run vscode:install"
```

## 📁 Folder Structure After Implementation

```
soroban-guard/
├── 📄 src/
│   ├── 📄 vscode.rs (new)
│   ├── 📄 lib.rs (updated)
│   └── 📄 ... (existing files)
├── 📄 vscode/ (new directory)
│   ├── 📄 package.json (new)
│   ├── 📄 webpack.config.js (new)
│   ├── 📄 tsconfig.json (new)
│   ├── 📄 icon.png (new)
│   ├── 📄 src/
│   │   ├── 📄 extension.ts (new)
│   │   ├── 📄 commands.ts (new)
│   │   ├── 📄 diagnostics.ts (new)
│   │   ├── 📄 statusBar.ts (new)
│   │   ├── 📄 config.ts (new)
│   │   └── 📄 scanner.ts (new)
│   └── 📄 dist/ (build output)
├── 📄 scripts/
│   └── 📄 build-vscode.sh (new)
└── 📄 Cargo.toml (updated)
```

## 🚀 Implementation Steps

1. **Create VS Code extension structure** - Set up TypeScript project
2. **Implement core extension** - Main activation and lifecycle
3. **Create scanner integration** - Bridge between Rust and VS Code
4. **Add diagnostics system** - Real-time vulnerability highlighting
5. **Implement commands** - Scan file, workspace, configure
6. **Add status bar integration** - Show scan status and quick actions
7. **Create language features** - Completion, hover, code actions
8. **Set up build system** - Webpack configuration and scripts
9. **Test integration** - End-to-end testing with VS Code
10. **Package for marketplace** - Prepare for VS Code marketplace

## ✅ Success Metrics

- [ ] Extension loads in <2 seconds
- [ ] Real-time scanning completes in <5 seconds for typical files
- [ ] All vulnerability types detected and highlighted correctly
- [ ] Code actions provide working quick fixes
- [ ] Extension works across all platforms (Windows, macOS, Linux)
- [ ] Extension size <5MB when packaged

## 🎯 Definition of Done

This issue is **COMPLETE** when:
1. All VS Code extension components are implemented and functional
2. Real-time scanning works seamlessly during development
3. Diagnostics and code actions provide helpful feedback
4. Extension configuration is intuitive and persistent
5. Build system works and produces distributable package
6. Extension passes VS Code marketplace validation

## 📋 Additional Notes

### Publishing to VS Code Marketplace

When ready to publish:

1. **Create publisher account** at https://marketplace.visualstudio.com/manage
2. **Package extension**: `cd vscode && npm run vscode:package`
3. **Validate extension**: `vsce verify --packagePath soroban-security-guard-*.vsix`
4. **Publish**: `vsce publish --packagePath soroban-security-guard-*.vsix`

### Extension Features

- **Real-time scanning**: Automatic scanning on file changes
- **Diagnostic highlighting**: Color-coded severity levels
- **Quick fixes**: Automated code suggestions
- **Code completion**: Security-aware suggestions
- **Hover information**: Detailed vulnerability descriptions
- **Workspace scanning**: Bulk analysis of entire projects
- **Configuration**: Customizable rules and thresholds
- **Status bar**: Quick access to common actions
- **Report generation**: Export scan results in multiple formats
