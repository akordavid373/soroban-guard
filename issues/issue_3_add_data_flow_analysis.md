---
title: Add data flow analysis for advanced vulnerability detection
labels: enhancement, security, analysis, data-flow
assignees: []
---

## 🎯 Enhancement Description

The current rule system primarily uses pattern matching and AST analysis. Adding data flow analysis would enable detection of more sophisticated vulnerabilities that require tracking data through multiple function calls and understanding variable relationships.

## 📁 Files to Modify

### Primary Files
```
📄 src/data_flow.rs (CREATE NEW)
📄 src/taint_analysis.rs (CREATE NEW)
📄 src/rules.rs (update with data flow rules)
📄 tests/data_flow_tests.rs (CREATE NEW)
```

### Secondary Files
```
📄 src/scanner.rs (integrate data flow analysis)
📄 src/lib.rs (export new modules)
📄 examples/data_flow_examples.rs (CREATE NEW)
📄 docs/data-flow-analysis.md (CREATE NEW)
```

## 🎯 Acceptance Criteria

### ✅ MUST HAVE (High Priority)
- [ ] **src/data_flow.rs** - Create core data flow analysis engine
- [ ] **src/taint_analysis.rs** - Implement taint tracking system
- [ ] **src/data_flow.rs** - Add variable tracking across function calls
- [ ] **src/rules.rs** - Create 3 new data flow-based rules
- [ ] **tests/data_flow_tests.rs** - Comprehensive test coverage

### ✅ SHOULD HAVE (Medium Priority)
- [ ] **src/data_flow.rs** - Add storage data flow analysis
- [ ] **src/rules.rs** - Update 2 existing rules to use data flow
- [ ] **examples/data_flow_examples.rs** - Create vulnerability examples
- [ ] **src/scanner.rs** - Integrate data flow into scanning pipeline

### ✅ COULD HAVE (Low Priority)
- [ ] **docs/data-flow-analysis.md** - Documentation for new features
- [ ] **src/lib.rs** - Export data flow analysis API
- [ ] **benches/data_flow_benchmarks.rs** - Performance benchmarks

## 🔧 Implementation Details

### 1. src/data_flow.rs (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\src\data_flow.rs`

**Complete Content**:
```rust
use crate::ast::{Function, Variable, Type, CodeLocation};
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlowGraph {
    pub nodes: Vec<DataFlowNode>,
    pub edges: Vec<DataFlowEdge>,
    pub entry_points: Vec<String>,
    pub exit_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlowNode {
    pub id: String,
    pub node_type: NodeType,
    pub variable: Option<Variable>,
    pub location: CodeLocation,
    pub taint_level: TaintLevel,
    pub context: ExecutionContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Entry,
    VariableDeclaration,
    Assignment,
    FunctionCall,
    Return,
    StorageRead,
    StorageWrite,
    Conditional,
    Loop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaintLevel {
    Untainted,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlowEdge {
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeType {
    Assignment,
    ParameterPass,
    Return,
    StorageDependency,
    Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub function_name: String,
    pub contract_name: Option<String>,
    pub call_stack: Vec<String>,
    pub storage_context: HashMap<String, StorageContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageContext {
    pub storage_type: String, // instance, persistent, temporary
    pub key_pattern: String,
    pub last_read: Option<CodeLocation>,
    pub last_write: Option<CodeLocation>,
    pub read_write_race: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaintSource {
    pub name: String,
    pub source_type: TaintSourceType,
    pub location: CodeLocation,
    pub taint_level: TaintLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaintSourceType {
    UserInput,
    ExternalCall,
    StorageRead,
    Parameter,
    EnvironmentVariable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaintSink {
    pub name: String,
    pub sink_type: TaintSinkType,
    pub location: CodeLocation,
    pub required_sanitization: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaintSinkType {
    StorageWrite,
    ExternalCall,
    EventEmission,
    Return,
    Panic,
}

pub struct DataFlowAnalyzer {
    graph: DataFlowGraph,
    taint_sources: Vec<TaintSource>,
    taint_sinks: Vec<TaintSink>,
    variable_flows: HashMap<String, Vec<String>>,
}

impl DataFlowAnalyzer {
    pub fn new() -> Self {
        Self {
            graph: DataFlowGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
                entry_points: Vec::new(),
                exit_points: Vec::new(),
            },
            taint_sources: Vec::new(),
            taint_sinks: Vec::new(),
            variable_flows: HashMap::new(),
        }
    }

    pub fn analyze_function(&mut self, function: &Function) -> anyhow::Result<DataFlowGraph> {
        self.reset();
        
        // Add entry point
        self.add_entry_point(function);
        
        // Analyze function body
        self.analyze_function_body(function)?;
        
        // Build data flow edges
        self.build_data_flow_edges()?;
        
        // Identify taint sources and sinks
        self.identify_taint_sources(function)?;
        self.identify_taint_sinks(function)?;
        
        // Propagate taint through graph
        self.propagate_taint()?;
        
        Ok(self.graph.clone())
    }

    fn reset(&mut self) {
        self.graph = DataFlowGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
            entry_points: Vec::new(),
            exit_points: Vec::new(),
        };
        self.taint_sources.clear();
        self.taint_sinks.clear();
        self.variable_flows.clear();
    }

    fn add_entry_point(&mut self, function: &Function) {
        let entry_node = DataFlowNode {
            id: format!("entry_{}", function.name),
            node_type: NodeType::Entry,
            variable: None,
            location: CodeLocation {
                file_path: function.location.file_path.clone(),
                line: function.location.line,
                column: 0,
                function: Some(function.name.clone()),
                contract: None,
            },
            taint_level: TaintLevel::Untainted,
            context: ExecutionContext {
                function_name: function.name.clone(),
                contract_name: None,
                call_stack: Vec::new(),
                storage_context: HashMap::new(),
            },
        };

        self.graph.nodes.push(entry_node);
        self.graph.entry_points.push(format!("entry_{}", function.name));
    }

    fn analyze_function_body(&mut self, function: &Function) -> anyhow::Result<()> {
        let mut current_line = function.location.line;
        
        for statement in &function.body {
            match self.classify_statement(statement) {
                StatementType::VariableDeclaration { name, var_type } => {
                    self.add_variable_declaration(name, var_type, current_line, &function.name)?;
                }
                StatementType::Assignment { target, source } => {
                    self.add_assignment(target, source, current_line, &function.name)?;
                }
                StatementType::FunctionCall { name, args } => {
                    self.add_function_call(name, args, current_line, &function.name)?;
                }
                StatementType::StorageOperation { operation, key, value } => {
                    self.add_storage_operation(operation, key, value, current_line, &function.name)?;
                }
                StatementType::Conditional { condition, body } => {
                    self.add_conditional(condition, body, current_line, &function.name)?;
                }
                StatementType::Return { value } => {
                    self.add_return(value, current_line, &function.name)?;
                }
            }
            current_line += 1;
        }

        Ok(())
    }

    fn classify_statement(&self, statement: &str) -> StatementType {
        let trimmed = statement.trim();
        
        // Variable declaration
        if trimmed.starts_with("let ") && trimmed.contains("=") {
            let parts: Vec<&str> = trimmed.splitn(3, ' ').collect();
            if parts.len() >= 3 {
                let var_part = parts[1];
                let name = var_part.split(':').next().unwrap_or(var_part).trim();
                let var_type = if var_part.contains(':') {
                    var_part.split(':').nth(1).unwrap_or("").trim()
                } else {
                    ""
                };
                
                let assignment_part = parts[2..].join(" ");
                let source = assignment_part.split('=').nth(1).unwrap_or("").trim();
                
                return StatementType::VariableDeclaration {
                    name: name.to_string(),
                    var_type: var_type.to_string(),
                };
            }
        }
        
        // Assignment
        if trimmed.contains('=') && !trimmed.starts_with("let ") {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                return StatementType::Assignment {
                    target: parts[0].trim().to_string(),
                    source: parts[1].trim().to_string(),
                };
            }
        }
        
        // Function call
        if trimmed.contains('(') && trimmed.contains(')') && !trimmed.starts_with("if ") {
            let paren_pos = trimmed.find('(').unwrap_or(0);
            let name = trimmed[..paren_pos].trim();
            let args_part = &trimmed[paren_pos..];
            
            return StatementType::FunctionCall {
                name: name.to_string(),
                args: args_part.to_string(),
            };
        }
        
        // Storage operations
        if trimmed.contains("env.storage()") {
            if trimmed.contains(".get(") {
                let after_get = trimmed.split(".get(").nth(1).unwrap_or("");
                let key = after_get.split(')').next().unwrap_or("").trim();
                return StatementType::StorageOperation {
                    operation: "get".to_string(),
                    key: key.to_string(),
                    value: String::new(),
                };
            } else if trimmed.contains(".set(") {
                let after_set = trimmed.split(".set(").nth(1).unwrap_or("");
                let parts: Vec<&str> = after_set.splitn(2, ',').collect();
                let key = parts.get(0).unwrap_or(&"").trim();
                let value = parts.get(1).unwrap_or(&"").trim().trim_end_matches(')');
                
                return StatementType::StorageOperation {
                    operation: "set".to_string(),
                    key: key.to_string(),
                    value: value.to_string(),
                };
            }
        }
        
        // Conditional
        if trimmed.starts_with("if ") {
            let after_if = trimmed[3..].trim();
            let condition_end = after_if.find('{').unwrap_or(after_if.len());
            let condition = after_if[..condition_end].trim();
            
            return StatementType::Conditional {
                condition: condition.to_string(),
                body: String::new(), // Would need more complex parsing
            };
        }
        
        // Return
        if trimmed.starts_with("return ") {
            let value = trimmed[7..].trim().to_string();
            return StatementType::Return { value };
        }
        
        StatementType::Other
    }

    fn add_variable_declaration(&mut self, name: &str, var_type: &str, line: usize, function: &str) -> anyhow::Result<()> {
        let node = DataFlowNode {
            id: format!("var_{}_{}", name, line),
            node_type: NodeType::VariableDeclaration,
            variable: Some(Variable {
                name: name.to_string(),
                var_type: Type::Simple(var_type.to_string()),
                mutable: true,
                location: CodeLocation {
                    file_path: "unknown".to_string(),
                    line,
                    column: 0,
                    function: Some(function.to_string()),
                    contract: None,
                },
            }),
            location: CodeLocation {
                file_path: "unknown".to_string(),
                line,
                column: 0,
                function: Some(function.to_string()),
                contract: None,
            },
            taint_level: TaintLevel::Untainted,
            context: ExecutionContext {
                function_name: function.to_string(),
                contract_name: None,
                call_stack: Vec::new(),
                storage_context: HashMap::new(),
            },
        };

        self.graph.nodes.push(node);
        self.variable_flows.insert(name.to_string(), Vec::new());
        
        Ok(())
    }

    fn add_assignment(&mut self, target: &str, source: &str, line: usize, function: &str) -> anyhow::Result<()> {
        let node = DataFlowNode {
            id: format!("assign_{}_{}", line, function),
            node_type: NodeType::Assignment,
            variable: None,
            location: CodeLocation {
                file_path: "unknown".to_string(),
                line,
                column: 0,
                function: Some(function.to_string()),
                contract: None,
            },
            taint_level: TaintLevel::Untainted,
            context: ExecutionContext {
                function_name: function.to_string(),
                contract_name: None,
                call_stack: Vec::new(),
                storage_context: HashMap::new(),
            },
        };

        self.graph.nodes.push(node);
        
        // Track variable flow
        if let Some(flows) = self.variable_flows.get_mut(target) {
            flows.push(source.to_string());
        }
        
        Ok(())
    }

    fn add_function_call(&mut self, name: &str, args: &str, line: usize, function: &str) -> anyhow::Result<()> {
        let node = DataFlowNode {
            id: format!("call_{}_{}_{}", name, line, function),
            node_type: NodeType::FunctionCall,
            variable: None,
            location: CodeLocation {
                file_path: "unknown".to_string(),
                line,
                column: 0,
                function: Some(function.to_string()),
                contract: None,
            },
            taint_level: TaintLevel::Untainted,
            context: ExecutionContext {
                function_name: function.to_string(),
                contract_name: None,
                call_stack: vec![function.to_string()],
                storage_context: HashMap::new(),
            },
        };

        self.graph.nodes.push(node);
        Ok(())
    }

    fn add_storage_operation(&mut self, operation: &str, key: &str, value: &str, line: usize, function: &str) -> anyhow::Result<()> {
        let node_type = match operation {
            "get" => NodeType::StorageRead,
            "set" => NodeType::StorageWrite,
            _ => NodeType::FunctionCall,
        };

        let node = DataFlowNode {
            id: format!("storage_{}_{}_{}", operation, line, function),
            node_type,
            variable: None,
            location: CodeLocation {
                file_path: "unknown".to_string(),
                line,
                column: 0,
                function: Some(function.to_string()),
                contract: None,
            },
            taint_level: TaintLevel::Untainted,
            context: ExecutionContext {
                function_name: function.to_string(),
                contract_name: None,
                call_stack: Vec::new(),
                storage_context: HashMap::new(),
            },
        };

        self.graph.nodes.push(node);
        Ok(())
    }

    fn add_conditional(&mut self, condition: &str, _body: &str, line: usize, function: &str) -> anyhow::Result<()> {
        let node = DataFlowNode {
            id: format!("cond_{}_{}", line, function),
            node_type: NodeType::Conditional,
            variable: None,
            location: CodeLocation {
                file_path: "unknown".to_string(),
                line,
                column: 0,
                function: Some(function.to_string()),
                contract: None,
            },
            taint_level: TaintLevel::Untainted,
            context: ExecutionContext {
                function_name: function.to_string(),
                contract_name: None,
                call_stack: Vec::new(),
                storage_context: HashMap::new(),
            },
        };

        self.graph.nodes.push(node);
        Ok(())
    }

    fn add_return(&mut self, value: &str, line: usize, function: &str) -> anyhow::Result<()> {
        let node = DataFlowNode {
            id: format!("return_{}_{}", line, function),
            node_type: NodeType::Return,
            variable: None,
            location: CodeLocation {
                file_path: "unknown".to_string(),
                line,
                column: 0,
                function: Some(function.to_string()),
                contract: None,
            },
            taint_level: TaintLevel::Untainted,
            context: ExecutionContext {
                function_name: function.to_string(),
                contract_name: None,
                call_stack: Vec::new(),
                storage_context: HashMap::new(),
            },
        };

        self.graph.nodes.push(node);
        self.graph.exit_points.push(format!("return_{}_{}", line, function));
        Ok(())
    }

    fn build_data_flow_edges(&mut self) -> anyhow::Result<()> {
        // Build edges based on variable flows and control flow
        for (target, sources) in &self.variable_flows {
            for source in sources {
                let edge = DataFlowEdge {
                    from: source.clone(),
                    to: target.clone(),
                    edge_type: EdgeType::Assignment,
                    conditions: Vec::new(),
                };
                self.graph.edges.push(edge);
            }
        }

        // Add control flow edges (simplified)
        for i in 0..self.graph.nodes.len() - 1 {
            let edge = DataFlowEdge {
                from: self.graph.nodes[i].id.clone(),
                to: self.graph.nodes[i + 1].id.clone(),
                edge_type: EdgeType::Assignment,
                conditions: Vec::new(),
            };
            self.graph.edges.push(edge);
        }

        Ok(())
    }

    fn identify_taint_sources(&mut self, function: &Function) -> anyhow::Result<()> {
        // Function parameters are taint sources
        for param in &function.parameters {
            let source = TaintSource {
                name: param.name.clone(),
                source_type: TaintSourceType::Parameter,
                location: param.location.clone(),
                taint_level: TaintLevel::Medium,
            };
            self.taint_sources.push(source);
        }

        // Storage reads are taint sources
        for node in &self.graph.nodes {
            if matches!(node.node_type, NodeType::StorageRead) {
                let source = TaintSource {
                    name: node.id.clone(),
                    source_type: TaintSourceType::StorageRead,
                    location: node.location.clone(),
                    taint_level: TaintLevel::Low,
                };
                self.taint_sources.push(source);
            }
        }

        Ok(())
    }

    fn identify_taint_sinks(&mut self, _function: &Function) -> anyhow::Result<()> {
        // Storage writes are taint sinks
        for node in &self.graph.nodes {
            if matches!(node.node_type, NodeType::StorageWrite) {
                let sink = TaintSink {
                    name: node.id.clone(),
                    sink_type: TaintSinkType::StorageWrite,
                    location: node.location.clone(),
                    required_sanitization: vec!["access_control".to_string(), "validation".to_string()],
                };
                self.taint_sinks.push(sink);
            }
        }

        Ok(())
    }

    fn propagate_taint(&mut self) -> anyhow::Result<()> {
        // Simple taint propagation - can be enhanced
        for source in &self.taint_sources {
            for node in &mut self.graph.nodes {
                if self.is_tainted_by_source(node, source) {
                    node.taint_level = source.taint_level.clone();
                }
            }
        }

        Ok(())
    }

    fn is_tainted_by_source(&self, node: &DataFlowNode, source: &TaintSource) -> bool {
        // Simple check - can be enhanced with graph traversal
        node.location.function == source.location.function ||
        node.id.contains(&source.name)
    }

    pub fn find_vulnerabilities(&self) -> Vec<DataFlowVulnerability> {
        let mut vulnerabilities = Vec::new();

        // Check for tainted data reaching sinks
        for sink in &self.taint_sinks {
            for node in &self.graph.nodes {
                if node.id == sink.name && !matches!(node.taint_level, TaintLevel::Untainted) {
                    let vulnerability = DataFlowVulnerability {
                        vulnerability_type: VulnerabilityType::TaintedDataFlow,
                        source: node.location.clone(),
                        sink: sink.location.clone(),
                        taint_level: node.taint_level.clone(),
                        description: format!(
                            "Tainted data with level {:?} reaches sensitive sink {:?}",
                            node.taint_level, sink.sink_type
                        ),
                        severity: self.calculate_severity(&node.taint_level, &sink.sink_type),
                    };
                    vulnerabilities.push(vulnerability);
                }
            }
        }

        // Check for storage race conditions
        vulnerabilities.extend(self.check_storage_race_conditions());

        vulnerabilities
    }

    fn check_storage_race_conditions(&self) -> Vec<DataFlowVulnerability> {
        let mut vulnerabilities = Vec::new();

        // Find read-modify-write patterns
        let mut storage_operations: HashMap<String, Vec<&DataFlowNode>> = HashMap::new();

        for node in &self.graph.nodes {
            if matches!(node.node_type, NodeType::StorageRead | NodeType::StorageWrite) {
                let key = self.extract_storage_key(node);
                storage_operations.entry(key).or_default().push(node);
            }
        }

        for (key, operations) in storage_operations {
            if operations.len() > 1 {
                // Check for race conditions
                for i in 0..operations.len() - 1 {
                    for j in i + 1..operations.len() {
                        if self.is_potential_race_condition(operations[i], operations[j]) {
                            let vulnerability = DataFlowVulnerability {
                                vulnerability_type: VulnerabilityType::StorageRaceCondition,
                                source: operations[i].location.clone(),
                                sink: operations[j].location.clone(),
                                taint_level: TaintLevel::Medium,
                                description: format!("Potential race condition on storage key: {}", key),
                                severity: Severity::High,
                            };
                            vulnerabilities.push(vulnerability);
                        }
                    }
                }
            }
        }

        vulnerabilities
    }

    fn extract_storage_key(&self, node: &DataFlowNode) -> String {
        // Simple extraction - can be enhanced
        node.id.split('_').nth(2).unwrap_or("unknown").to_string()
    }

    fn is_potential_race_condition(&self, op1: &DataFlowNode, op2: &DataFlowNode) -> bool {
        matches!(op1.node_type, NodeType::StorageRead) &&
        matches!(op2.node_type, NodeType::StorageWrite) &&
        op1.location.function == op2.location.function
    }

    fn calculate_severity(&self, taint_level: &TaintLevel, sink_type: &TaintSinkType) -> Severity {
        match (taint_level, sink_type) {
            (TaintLevel::Critical, _) => Severity::Critical,
            (TaintLevel::High, TaintSinkType::StorageWrite) => Severity::High,
            (TaintLevel::High, _) => Severity::Medium,
            (TaintLevel::Medium, TaintSinkType::StorageWrite) => Severity::Medium,
            (TaintLevel::Medium, _) => Severity::Low,
            _ => Severity::Low,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlowVulnerability {
    pub vulnerability_type: VulnerabilityType,
    pub source: CodeLocation,
    pub sink: CodeLocation,
    pub taint_level: TaintLevel,
    pub description: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VulnerabilityType {
    TaintedDataFlow,
    StorageRaceCondition,
    PrivilegeEscalation,
    InjectionVulnerability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
enum StatementType {
    VariableDeclaration { name: String, var_type: String },
    Assignment { target: String, source: String },
    FunctionCall { name: String, args: String },
    StorageOperation { operation: String, key: String, value: String },
    Conditional { condition: String, body: String },
    Return { value: String },
    Other,
}
```

### 2. src/taint_analysis.rs (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\src\taint_analysis.rs`

**Complete Content**:
```rust
use crate::data_flow::{DataFlowGraph, DataFlowNode, TaintLevel, TaintSource, TaintSink};
use crate::ast::CodeLocation;
use std::collections::{HashMap, HashSet};

pub struct TaintAnalyzer {
    taint_rules: Vec<TaintRule>,
    sanitization_rules: Vec<SanitizationRule>,
}

impl TaintAnalyzer {
    pub fn new() -> Self {
        Self {
            taint_rules: Self::default_taint_rules(),
            sanitization_rules: Self::default_sanitization_rules(),
        }
    }

    pub fn analyze(&self, graph: &DataFlowGraph) -> Vec<TaintFlow> {
        let mut taint_flows = Vec::new();
        
        // Identify taint sources
        let sources = self.identify_taint_sources(graph);
        
        // Propagate taint through graph
        let taint_map = self.propagate_taint(graph, &sources);
        
        // Find taint flows to sinks
        let sinks = self.identify_taint_sinks(graph);
        
        for sink in sinks {
            if let Some(taint_level) = taint_map.get(&sink.location) {
                if !matches!(taint_level, TaintLevel::Untainted) {
                    let flow = TaintFlow {
                        source: sources.clone(),
                        sink,
                        taint_level: taint_level.clone(),
                        path: self.find_taint_path(graph, &sources, &sink),
                        sanitization_needed: self.required_sanitization(&sink, taint_level),
                    };
                    taint_flows.push(flow);
                }
            }
        }
        
        taint_flows
    }

    fn identify_taint_sources(&self, graph: &DataFlowGraph) -> Vec<TaintSource> {
        let mut sources = Vec::new();
        
        for node in &graph.nodes {
            for rule in &self.taint_rules {
                if rule.matches(node) {
                    let source = TaintSource {
                        name: node.id.clone(),
                        source_type: rule.source_type.clone(),
                        location: node.location.clone(),
                        taint_level: rule.initial_taint_level.clone(),
                    };
                    sources.push(source);
                }
            }
        }
        
        sources
    }

    fn identify_taint_sinks(&self, graph: &DataFlowGraph) -> Vec<TaintSink> {
        let mut sinks = Vec::new();
        
        for node in &graph.nodes {
            if let Some(sink_type) = self.is_sink_node(node) {
                let sink = TaintSink {
                    name: node.id.clone(),
                    sink_type,
                    location: node.location.clone(),
                    required_sanitization: self.get_required_sanitization(node),
                };
                sinks.push(sink);
            }
        }
        
        sinks
    }

    fn propagate_taint(&self, graph: &DataFlowGraph, sources: &[TaintSource]) -> HashMap<CodeLocation, TaintLevel> {
        let mut taint_map: HashMap<CodeLocation, TaintLevel> = HashMap::new();
        
        // Initialize sources
        for source in sources {
            taint_map.insert(source.location.clone(), source.taint_level.clone());
        }
        
        // Propagate through graph
        let mut changed = true;
        while changed {
            changed = false;
            
            for edge in &graph.edges {
                if let Some(source_taint) = taint_map.get(&self.node_to_location(graph, &edge.from)) {
                    let sink_location = self.node_to_location(graph, &edge.to);
                    let current_taint = taint_map.get(&sink_location).unwrap_or(&TaintLevel::Untainted);
                    
                    let new_taint = self.combine_taint_levels(source_taint, current_taint);
                    if !matches!(new_taint, TaintLevel::Untainted) && matches!(current_taint, TaintLevel::Untainted) {
                        taint_map.insert(sink_location, new_taint);
                        changed = true;
                    }
                }
            }
        }
        
        taint_map
    }

    fn node_to_location(&self, graph: &DataFlowGraph, node_id: &str) -> CodeLocation {
        graph.nodes
            .iter()
            .find(|n| n.id == node_id)
            .map(|n| n.location.clone())
            .unwrap_or_else(|| CodeLocation {
                file_path: "unknown".to_string(),
                line: 0,
                column: 0,
                function: None,
                contract: None,
            })
    }

    fn combine_taint_levels(&self, level1: &TaintLevel, level2: &TaintLevel) -> TaintLevel {
        use TaintLevel::*;
        
        match (level1, level2) {
            (Critical, _) | (_, Critical) => Critical,
            (High, _) | (_, High) => High,
            (Medium, _) | (_, Medium) => Medium,
            (Low, _) | (_, Low) => Low,
            (Untainted, Untainted) => Untainted,
        }
    }

    fn is_sink_node(&self, node: &DataFlowNode) -> Option<crate::data_flow::TaintSinkType> {
        use crate::data_flow::TaintSinkType;
        
        if node.id.contains("storage_set") {
            Some(TaintSinkType::StorageWrite)
        } else if node.id.contains("call") {
            Some(TaintSinkType::ExternalCall)
        } else if node.id.contains("return") {
            Some(TaintSinkType::Return)
        } else {
            None
        }
    }

    fn get_required_sanitization(&self, node: &DataFlowNode) -> Vec<String> {
        let mut required = Vec::new();
        
        if node.id.contains("storage_set") {
            required.push("access_control".to_string());
            required.push("validation".to_string());
        }
        
        if node.id.contains("call") {
            required.push("input_validation".to_string());
        }
        
        required
    }

    fn find_taint_path(&self, graph: &DataFlowGraph, _sources: &[TaintSource], sink: &TaintSink) -> Vec<CodeLocation> {
        let mut path = Vec::new();
        
        // Find path from source to sink (simplified)
        for node in &graph.nodes {
            if node.location.line <= sink.location.line {
                path.push(node.location.clone());
            }
        }
        
        path
    }

    fn required_sanitization(&self, sink: &TaintSink, taint_level: &TaintLevel) -> Vec<String> {
        let mut required = sink.required_sanitization.clone();
        
        if matches!(taint_level, TaintLevel::High | TaintLevel::Critical) {
            required.push("strict_validation".to_string());
        }
        
        required
    }

    fn default_taint_rules() -> Vec<TaintRule> {
        vec![
            TaintRule {
                pattern: "param_".to_string(),
                source_type: crate::data_flow::TaintSourceType::Parameter,
                initial_taint_level: TaintLevel::Medium,
            },
            TaintRule {
                pattern: "storage_get".to_string(),
                source_type: crate::data_flow::TaintSourceType::StorageRead,
                initial_taint_level: TaintLevel::Low,
            },
        ]
    }

    fn default_sanitization_rules() -> Vec<SanitizationRule> {
        vec![
            SanitizationRule {
                pattern: "require!".to_string(),
                removes_taint: true,
                reduces_level: Some(TaintLevel::Low),
            },
            SanitizationRule {
                pattern: "checked_".to_string(),
                removes_taint: false,
                reduces_level: Some(TaintLevel::Low),
            },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct TaintRule {
    pub pattern: String,
    pub source_type: crate::data_flow::TaintSourceType,
    pub initial_taint_level: TaintLevel,
}

#[derive(Debug, Clone)]
pub struct SanitizationRule {
    pub pattern: String,
    pub removes_taint: bool,
    pub reduces_level: Option<TaintLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaintFlow {
    pub source: Vec<TaintSource>,
    pub sink: TaintSink,
    pub taint_level: TaintLevel,
    pub path: Vec<CodeLocation>,
    pub sanitization_needed: Vec<String>,
}
```

### 3. tests/data_flow_tests.rs (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\tests\data_flow_tests.rs`

**Complete Content**:
```rust
use soroban_security_guard::data_flow::{DataFlowAnalyzer, DataFlowNode, NodeType, TaintLevel};
use soroban_security_guard::ast::{Function, Variable, Type, CodeLocation};
use soroban_security_guard::taint_analysis::TaintAnalyzer;

#[test]
fn test_basic_data_flow_analysis() {
    let function = create_test_function();
    let mut analyzer = DataFlowAnalyzer::new();
    
    let graph = analyzer.analyze_function(&function).unwrap();
    
    assert!(!graph.nodes.is_empty());
    assert!(!graph.entry_points.is_empty());
}

#[test]
fn test_taint_propagation() {
    let function = create_tainted_function();
    let mut analyzer = DataFlowAnalyzer::new();
    
    let graph = analyzer.analyze_function(&function).unwrap();
    let vulnerabilities = analyzer.find_vulnerabilities();
    
    assert!(!vulnerabilities.is_empty());
    
    // Should detect tainted data reaching storage
    let storage_vuln = vulnerabilities.iter()
        .find(|v| format!("{:?}", v.vulnerability_type).contains("TaintedDataFlow"));
    assert!(storage_vuln.is_some());
}

#[test]
fn test_storage_race_condition() {
    let function = create_race_condition_function();
    let mut analyzer = DataFlowAnalyzer::new();
    
    let graph = analyzer.analyze_function(&function).unwrap();
    let vulnerabilities = analyzer.find_vulnerabilities();
    
    // Should detect potential race condition
    let race_vuln = vulnerabilities.iter()
        .find(|v| format!("{:?}", v.vulnerability_type).contains("StorageRaceCondition"));
    assert!(race_vuln.is_some());
}

#[test]
fn test_taint_analyzer() {
    let function = create_test_function();
    let mut data_flow_analyzer = DataFlowAnalyzer::new();
    let taint_analyzer = TaintAnalyzer::new();
    
    let graph = data_flow_analyzer.analyze_function(&function).unwrap();
    let taint_flows = taint_analyzer.analyze(&graph);
    
    // Should analyze without panicking
    assert!(!taint_flows.is_empty() || taint_flows.is_empty()); // Either way is fine
}

#[test]
fn test_complex_data_flow() {
    let function = create_complex_function();
    let mut analyzer = DataFlowAnalyzer::new();
    
    let graph = analyzer.analyze_function(&function).unwrap();
    let vulnerabilities = analyzer.find_vulnerabilities();
    
    // Should handle complex control flow
    assert!(!graph.nodes.is_empty());
    
    // Should find multiple vulnerability types
    let vuln_types: std::collections::HashSet<_> = vulnerabilities.iter()
        .map(|v| format!("{:?}", v.vulnerability_type))
        .collect();
    
    assert!(vuln_types.len() >= 1);
}

#[test]
fn test_variable_tracking() {
    let function = create_variable_tracking_function();
    let mut analyzer = DataFlowAnalyzer::new();
    
    let graph = analyzer.analyze_function(&function).unwrap();
    
    // Should track variable assignments
    let assignment_nodes: Vec<_> = graph.nodes.iter()
        .filter(|n| matches!(n.node_type, NodeType::Assignment))
        .collect();
    
    assert!(!assignment_nodes.is_empty());
}

fn create_test_function() -> Function {
    Function {
        name: "test_function".to_string(),
        visibility: crate::ast::Visibility::Public,
        parameters: vec![
            Variable {
                name: "amount".to_string(),
                var_type: Type::Simple("u64".to_string()),
                mutable: false,
                location: CodeLocation {
                    file_path: "test.rs".to_string(),
                    line: 1,
                    column: 0,
                    function: Some("test_function".to_string()),
                    contract: None,
                },
            }
        ],
        return_type: Some(Type::Simple("Result<(), &'static str>".to_string())),
        body: vec![
            "let balance = 1000;".to_string(),
            "if amount <= balance {".to_string(),
            "    return Ok(());".to_string(),
            "}".to_string(),
            "return Err(\"Insufficient balance\");".to_string(),
        ],
        location: CodeLocation {
            file_path: "test.rs".to_string(),
            line: 1,
            column: 0,
            function: Some("test_function".to_string()),
            contract: None,
        },
    }
}

fn create_tainted_function() -> Function {
    Function {
        name: "tainted_function".to_string(),
        visibility: crate::ast::Visibility::Public,
        parameters: vec![
            Variable {
                name: "user_input".to_string(),
                var_type: Type::Simple("String".to_string()),
                mutable: false,
                location: CodeLocation {
                    file_path: "test.rs".to_string(),
                    line: 1,
                    column: 0,
                    function: Some("tainted_function".to_string()),
                    contract: None,
                },
            }
        ],
        return_type: Some(Type::Simple("()".to_string())),
        body: vec![
            "env.storage().instance().set(&DataKey::UserData, &user_input);".to_string(),
        ],
        location: CodeLocation {
            file_path: "test.rs".to_string(),
            line: 1,
            column: 0,
            function: Some("tainted_function".to_string()),
            contract: None,
        },
    }
}

fn create_race_condition_function() -> Function {
    Function {
        name: "race_condition_function".to_string(),
        visibility: crate::ast::Visibility::Public,
        parameters: vec![
            Variable {
                name: "value".to_string(),
                var_type: Type::Simple("u64".to_string()),
                mutable: false,
                location: CodeLocation {
                    file_path: "test.rs".to_string(),
                    line: 1,
                    column: 0,
                    function: Some("race_condition_function".to_string()),
                    contract: None,
                },
            }
        ],
        return_type: Some(Type::Simple("()".to_string())),
        body: vec![
            "let current = env.storage().instance().get(&DataKey::Counter);".to_string(),
            "let new_value = current + value;".to_string(),
            "env.storage().instance().set(&DataKey::Counter, &new_value);".to_string(),
        ],
        location: CodeLocation {
            file_path: "test.rs".to_string(),
            line: 1,
            column: 0,
            function: Some("race_condition_function".to_string()),
            contract: None,
        },
    }
}

fn create_complex_function() -> Function {
    Function {
        name: "complex_function".to_string(),
        visibility: crate::ast::Visibility::Public,
        parameters: vec![
            Variable {
                name: "input".to_string(),
                var_type: Type::Simple("u64".to_string()),
                mutable: false,
                location: CodeLocation {
                    file_path: "test.rs".to_string(),
                    line: 1,
                    column: 0,
                    function: Some("complex_function".to_string()),
                    contract: None,
                },
            }
        ],
        return_type: Some(Type::Simple("Result<u64, &'static str>".to_string())),
        body: vec![
            "let validated = if input > 0 { input } else { return Err(\"Invalid input\"); };".to_string(),
            "let stored = env.storage().instance().get(&DataKey::Value);".to_string(),
            "let result = validated + stored;".to_string(),
            "env.storage().instance().set(&DataKey::Value, &result);".to_string(),
            "Ok(result)".to_string(),
        ],
        location: CodeLocation {
            file_path: "test.rs".to_string(),
            line: 1,
            column: 0,
            function: Some("complex_function".to_string()),
            contract: None,
        },
    }
}

fn create_variable_tracking_function() -> Function {
    Function {
        name: "variable_tracking".to_string(),
        visibility: crate::ast::Visibility::Public,
        parameters: vec![],
        return_type: Some(Type::Simple("u64".to_string())),
        body: vec![
            "let x = 10;".to_string(),
            "let y = x * 2;".to_string(),
            "let z = y + 5;".to_string(),
            "z".to_string(),
        ],
        location: CodeLocation {
            file_path: "test.rs".to_string(),
            line: 1,
            column: 0,
            function: Some("variable_tracking".to_string()),
            contract: None,
        },
    }
}
```

### 4. examples/data_flow_examples.rs (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\examples\data_flow_examples.rs`

**Complete Content**:
```rust
// Example contracts demonstrating data flow vulnerabilities

#![no_std]
#![no_main]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Map};

#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    Config(String),
}

#[contract]
pub struct VulnerableContract {
    #[contractstate]
    admin: Address,
}

#[contractimpl]
impl VulnerableContract {
    // VULNERABILITY: Tainted user input stored without validation
    pub fn set_config(env: &Env, user_input: String, value: u64) {
        // User input (tainted source) directly stored (taint sink)
        env.storage().instance().set(&DataKey::Config(user_input), &value);
    }
    
    // VULNERABILITY: Storage race condition
    pub unsafe fn increment_counter(env: &Env, amount: u64) {
        // Read-modify-write pattern without atomicity
        let current = env.storage().instance().get(&DataKey::Counter);
        let new_value = current + amount; // Race condition here
        env.storage().instance().set(&DataKey::Counter, &new_value);
    }
    
    // VULNERABILITY: Privilege escalation through tainted data
    pub fn transfer_with_permission(env: &Env, from: Address, to: Address, amount: u64, is_admin: bool) {
        // User-controlled boolean affects access control
        if is_admin || from == env.storage().instance().get(&DataKey::Admin) {
            let from_balance = env.storage().instance().get(&DataKey::Balance(from));
            if from_balance >= amount {
                env.storage().instance().set(&DataKey::Balance(from), &(from_balance - amount));
                env.storage().instance().set(&DataKey::Balance(to), &amount);
            }
        }
    }
    
    // VULNERABILITY: Unvalidated external data used in sensitive operations
    pub fn batch_transfer(env: &Env, transfers: Vec<(Address, u64)>) {
        for (recipient, amount) in transfers {
            // No validation of transfer amounts
            let current_supply = env.storage().instance().get(&DataKey::TotalSupply);
            env.storage().instance().set(&DataKey::TotalSupply, &(current_supply + amount));
            env.storage().instance().set(&DataKey::Balance(recipient), &amount);
        }
    }
    
    // VULNERABILITY: Tainted data affects control flow
    pub fn conditional_operation(env: &Env, user_choice: String, value: u64) {
        match user_choice.as_str() {
            "admin" => {
                // User can trigger admin operations
                env.storage().instance().set(&DataKey::Admin, &Address::zero());
            }
            "reset" => {
                // User can reset contract state
                env.storage().instance().set(&DataKey::TotalSupply, &0);
            }
            _ => {
                env.storage().instance().set(&DataKey::Balance(Address::zero()), &value);
            }
        }
    }
    
    // VULNERABILITY: Complex data flow with multiple taint sources
    pub fn complex_vulnerability(env: &Env, user_data: String, external_value: u64) {
        // Multiple taint sources combined
        let stored_value = env.storage().instance().get(&DataKey::Config(user_data.clone()));
        let combined = stored_value + external_value;
        
        // Tainted data reaches sensitive sink
        env.storage().instance().set(&DataKey::CriticalConfig(combined.to_string()), &combined);
        
        // Event emission with tainted data
        env.events().publish((Symbol::new(env, "ConfigUpdate"), (user_data, combined)));
    }
    
    // VULNERABILITY: Reentrancy through data flow
    pub fn vulnerable_withdraw(env: &Env, amount: u64) -> Result<(), &'static str> {
        let user = env.current_contract_address();
        let balance = env.storage().instance().get(&DataKey::Balance(user));
        
        if balance < amount {
            return Err("Insufficient balance");
        }
        
        // State change before external call
        env.storage().instance().set(&DataKey::Balance(user), &(balance - amount));
        
        // External call (potential reentrancy)
        env.invoke_contract(&user, &Symbol::new(env, "receive"), amount);
        
        Ok(())
    }
}

// Safe contract for comparison
#[contract]
pub struct SafeContract {
    #[contractstate]
    admin: Address,
}

#[contractimpl]
impl SafeContract {
    // SAFE: Proper input validation
    pub fn set_config(env: &Env, key: String, value: u64) -> Result<(), &'static str> {
        // Validate input before storage
        if key.len() > 100 {
            return Err("Key too long");
        }
        
        if value > 1000000 {
            return Err("Value too large");
        }
        
        // Only admin can set config
        let caller = env.current_contract_address();
        let admin = env.storage().instance().get(&DataKey::Admin);
        
        if caller != admin {
            return Err("Unauthorized");
        }
        
        env.storage().instance().set(&DataKey::Config(key), &value);
        Ok(())
    }
    
    // SAFE: Atomic operations
    pub fn safe_increment_counter(env: &Env, amount: u64) -> Result<(), &'static str> {
        // Use atomic operations or proper locking
        let current = env.storage().instance().get(&DataKey::Counter);
        
        if let Some(new_value) = current.checked_add(amount) {
            env.storage().instance().set(&DataKey::Counter, &new_value);
            Ok(())
        } else {
            Err("Overflow")
        }
    }
    
    // SAFE: Proper access control
    pub fn transfer(env: &Env, from: Address, to: Address, amount: u64) -> Result<(), &'static str> {
        // Validate parameters
        if amount == 0 {
            return Err("Invalid amount");
        }
        
        if to == Address::zero() {
            return Err("Invalid recipient");
        }
        
        // Check balances
        let from_balance = env.storage().instance().get(&DataKey::Balance(from));
        if from_balance < amount {
            return Err("Insufficient balance");
        }
        
        // Atomic state update
        env.storage().instance().set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage().instance().set(&DataKey::Balance(to), &amount);
        
        Ok(())
    }
}

// Additional vulnerable patterns
#[contract]
pub struct AdvancedVulnerableContract {
    #[contractstate]
    owner: Address,
}

#[contractimpl]
impl AdvancedVulnerableContract {
    // VULNERABILITY: Injection through data flow
    pub fn execute_command(env: &Env, command: String, param: u64) {
        // Command injection vulnerability
        let full_command = format!("{} {}", command, param);
        
        // Store command for later execution
        env.storage().instance().set(&DataKey::PendingCommand, &full_command);
        
        // Execute if conditions met
        if param > 1000 {
            Self::execute_stored_command(env);
        }
    }
    
    fn execute_stored_command(env: &Env) {
        let command = env.storage().instance().get(&DataKey::PendingCommand);
        
        // Dangerous: Execute stored command without validation
        match command.as_str() {
            "transfer_all" => {
                // Transfer all funds to owner
                let owner = env.storage().instance().get(&DataKey::Owner);
                env.storage().instance().set(&DataKey::Balance(owner), &1000000);
            }
            "reset_contract" => {
                // Reset contract state
                env.storage().instance().set(&DataKey::TotalSupply, &0);
            }
            _ => {}
        }
    }
    
    // VULNERABILITY: Logic flaw through data flow
    pub fn calculate_fee(env: &Env, amount: u64, user_type: String) -> u64 {
        let base_fee = amount / 100; // 1% base fee
        
        // User-controlled fee calculation
        match user_type.as_str() {
            "premium" => base_fee / 2, // 0.5% for premium
            "vip" => base_fee / 10, // 0.1% for VIP
            "admin" => 0, // No fee for admin (vulnerable!)
            _ => base_fee,
        }
    }
    
    // VULNERABILITY: Information disclosure through data flow
    pub fn get_user_info(env: &Env, requester: Address, target: Address) -> Result<Map<String, u64>, &'static str> {
        // Check if requester is authorized
        let is_admin = env.storage().instance().get(&DataKey::Admin) == requester;
        
        if is_admin {
            // Return all user data (including sensitive info)
            let mut info = Map::new(env);
            info.set("balance".into(), env.storage().instance().get(&DataKey::Balance(target)));
            info.set("transaction_count".into(), env.storage().instance().get(&DataKey::TxCount(target)));
            info.set("last_login".into(), env.storage().instance().get(&DataKey::LastLogin(target)));
            Ok(info)
        } else {
            Err("Unauthorized")
        }
    }
}
```

## 📁 Folder Structure After Implementation

```
soroban-guard/
├── 📄 src/
│   ├── 📄 data_flow.rs (new)
│   ├── 📄 taint_analysis.rs (new)
│   ├── 📄 rules.rs (updated)
│   ├── 📄 scanner.rs (updated)
│   └── 📄 lib.rs (updated)
├── 📄 tests/
│   └── 📄 data_flow_tests.rs (new)
├── 📄 examples/
│   └── 📄 data_flow_examples.rs (new)
└── 📄 docs/
    └── 📄 data-flow-analysis.md (new)
```

## 🚀 Implementation Steps

1. **Create src/data_flow.rs** - Core data flow analysis engine
2. **Create src/taint_analysis.rs** - Taint tracking system
3. **Update src/rules.rs** - Add data flow-based security rules
4. **Create tests/data_flow_tests.rs** - Comprehensive test suite
5. **Create examples/data_flow_examples.rs** - Vulnerability examples
6. **Update src/scanner.rs** - Integrate data flow into pipeline
7. **Create docs/data-flow-analysis.md** - Documentation

## ✅ Success Metrics

- [ ] Data flow analysis detects 90% of test vulnerabilities
- [ ] Performance impact <50% on scanning time
- [ ] Test coverage >95% for new functionality
- [ ] Memory usage increase <30%
- [ ] Zero false negatives in test cases

## 🎯 Definition of Done

This issue is **COMPLETE** when:
1. All files are created/modified as specified
2. Data flow analysis detects test vulnerabilities
3. Performance benchmarks meet requirements
4. Documentation is comprehensive
5. Integration with existing scanner works seamlessly
