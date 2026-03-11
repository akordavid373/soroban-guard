---
title: Create web interface for interactive security scanning
labels: enhancement, ui, web, frontend
assignees: []
---

## 🌐 Enhancement Description

Create a web-based interface for Soroban Security Guard to provide a more user-friendly experience for contract security analysis. This will eliminate the need for local installation and provide interactive features for better vulnerability analysis.

## 📁 Files to Modify

### Primary Files
```
📄 web/ (CREATE NEW DIRECTORY)
├── 📄 web/src/ (CREATE NEW)
│   ├── 📄 App.tsx (CREATE NEW)
│   ├── 📄 components/ (CREATE NEW)
│   │   ├── 📄 CodeEditor.tsx (CREATE NEW)
│   │   ├── 📄 ResultsPanel.tsx (CREATE NEW)
│   │   ├── 📄 ConfigPanel.tsx (CREATE NEW)
│   │   └── 📄 ReportViewer.tsx (CREATE NEW)
│   ├── 📄 hooks/ (CREATE NEW)
│   │   ├── 📄 useScanner.ts (CREATE NEW)
│   │   └── 📄 useConfig.ts (CREATE NEW)
│   ├── 📄 services/ (CREATE NEW)
│   │   └── 📄 scannerService.ts (CREATE NEW)
│   └── 📄 types/ (CREATE NEW)
│       └── 📄 index.ts (CREATE NEW)
├── 📄 web/public/ (CREATE NEW)
│   ├── 📄 index.html (CREATE NEW)
│   └── 📄 favicon.ico (CREATE NEW)
├── 📄 web/package.json (CREATE NEW)
├── 📄 web/tsconfig.json (CREATE NEW)
├── 📄 web/vite.config.ts (CREATE NEW)
└── 📄 web/README.md (CREATE NEW)
```

### Secondary Files
```
📄 src/wasm.rs (CREATE NEW - WASM interface)
📄 src/lib.rs (update exports)
📄 Cargo.toml (add WASM dependencies)
📄 scripts/build-web.sh (CREATE NEW)
📄 scripts/serve-web.sh (CREATE NEW)
```

## 🎯 Acceptance Criteria

### ✅ MUST HAVE (High Priority)
- [ ] **web/src/App.tsx** - Main React application component
- [ ] **web/src/components/CodeEditor.tsx** - Monaco editor with Rust syntax
- [ ] **web/src/components/ResultsPanel.tsx** - Interactive vulnerability display
- [ ] **web/src/services/scannerService.ts** - WASM scanner integration
- [ ] **src/wasm.rs** - WASM interface for scanner engine
- [ ] **web/package.json** - React/TypeScript project setup

### ✅ SHOULD HAVE (Medium Priority)
- [ ] **web/src/components/ConfigPanel.tsx** - Rule configuration interface
- [ ] **web/src/components/ReportViewer.tsx** - Multi-format report display
- [ ] **web/src/hooks/useScanner.ts** - Scanner state management
- [ ] **scripts/build-web.sh** - Build automation script
- [ ] **web/vite.config.ts** - Development server configuration

### ✅ COULD HAVE (Low Priority)
- [ ] **web/src/hooks/useConfig.ts** - Configuration state management
- [ ] **scripts/serve-web.sh** - Development server script
- [ ] **web/README.md** - Web interface documentation
- [ ] **web/public/** - Static assets and icons

## 🔧 Implementation Details

### 1. web/package.json (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\web\package.json`

**Complete Content**:
```json
{
  "name": "soroban-security-guard-web",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "build:wasm": "cd .. && cargo build --target wasm32-unknown-unknown --release",
    "copy:wasm": "cp ../target/wasm32-unknown-unknown/release/soroban_security_guard.wasm public/",
    "build:all": "npm run build:wasm && npm run copy:wasm && npm run build"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "@monaco-editor/react": "^4.6.0",
    "monaco-editor": "^0.45.0",
    "lucide-react": "^0.292.0",
    "clsx": "^2.0.0",
    "tailwind-merge": "^2.0.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.37",
    "@types/react-dom": "^18.2.15",
    "@vitejs/plugin-react": "^4.1.1",
    "typescript": "^5.2.2",
    "vite": "^4.5.0",
    "tailwindcss": "^3.3.5",
    "autoprefixer": "^10.4.16",
    "postcss": "^8.4.31"
  }
}
```

### 2. web/src/App.tsx (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\web\src\App.tsx`

**Complete Content**:
```typescript
import React, { useState, useCallback } from 'react';
import { CodeEditor } from './components/CodeEditor';
import { ResultsPanel } from './components/ResultsPanel';
import { ConfigPanel } from './components/ConfigPanel';
import { ReportViewer } from './components/ReportViewer';
import { useScanner } from './hooks/useScanner';
import { ScanResult, ScannerConfig } from './types';
import { Shield, Settings, FileText, X } from 'lucide-react';

function App() {
  const [activeTab, setActiveTab] = useState<'editor' | 'config' | 'report'>('editor');
  const [showReport, setShowReport] = useState(false);
  const [selectedResult, setSelectedResult] = useState<ScanResult | null>(null);
  
  const {
    isScanning,
    scanResults,
    error,
    config,
    scanContract,
    updateConfig
  } = useScanner();

  const handleScan = useCallback(async (code: string) => {
    try {
      await scanContract(code);
      setActiveTab('report');
    } catch (err) {
      console.error('Scan failed:', err);
    }
  }, [scanContract]);

  const handleConfigChange = useCallback((newConfig: ScannerConfig) => {
    updateConfig(newConfig);
  }, [updateConfig]);

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div className="flex items-center space-x-3">
              <Shield className="w-8 h-8 text-blue-600" />
              <h1 className="text-xl font-bold text-gray-900">
                Soroban Security Guard
              </h1>
            </div>
            
            <nav className="flex space-x-1">
              <button
                onClick={() => setActiveTab('editor')}
                className={`px-4 py-2 rounded-md text-sm font-medium transition-colors ${
                  activeTab === 'editor'
                    ? 'bg-blue-100 text-blue-700'
                    : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'
                }`}
              >
                <FileText className="w-4 h-4 inline mr-2" />
                Editor
              </button>
              
              <button
                onClick={() => setActiveTab('config')}
                className={`px-4 py-2 rounded-md text-sm font-medium transition-colors ${
                  activeTab === 'config'
                    ? 'bg-blue-100 text-blue-700'
                    : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'
                }`}
              >
                <Settings className="w-4 h-4 inline mr-2" />
                Config
              </button>
              
              {scanResults.length > 0 && (
                <button
                  onClick={() => setActiveTab('report')}
                  className={`px-4 py-2 rounded-md text-sm font-medium transition-colors ${
                    activeTab === 'report'
                      ? 'bg-blue-100 text-blue-700'
                      : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'
                  }`}
                >
                  Report ({scanResults.length})
                </button>
              )}
            </nav>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {activeTab === 'editor' && (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div className="space-y-4">
              <div className="flex justify-between items-center">
                <h2 className="text-lg font-semibold text-gray-900">
                  Contract Code
                </h2>
                <button
                  onClick={() => handleScan('')} // Will get code from editor
                  disabled={isScanning}
                  className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                >
                  {isScanning ? 'Scanning...' : 'Scan Contract'}
                </button>
              </div>
              
              <CodeEditor
                onScan={handleScan}
                isScanning={isScanning}
              />
              
              {error && (
                <div className="bg-red-50 border border-red-200 rounded-md p-4">
                  <h3 className="text-sm font-medium text-red-800">Error</h3>
                  <p className="mt-1 text-sm text-red-700">{error}</p>
                </div>
              )}
            </div>
            
            <div className="space-y-4">
              <h2 className="text-lg font-semibold text-gray-900">
                Scan Results
              </h2>
              
              <ResultsPanel
                results={scanResults}
                onResultSelect={setSelectedResult}
                isScanning={isScanning}
              />
            </div>
          </div>
        )}

        {activeTab === 'config' && (
          <ConfigPanel
            config={config}
            onConfigChange={handleConfigChange}
          />
        )}

        {activeTab === 'report' && (
          <ReportViewer
            results={scanResults}
            config={config}
          />
        )}
      </main>

      {/* Result Detail Modal */}
      {selectedResult && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
          <div className="bg-white rounded-lg max-w-2xl w-full max-h-[80vh] overflow-y-auto">
            <div className="flex justify-between items-center p-6 border-b border-gray-200">
              <h3 className="text-lg font-semibold text-gray-900">
                Vulnerability Details
              </h3>
              <button
                onClick={() => setSelectedResult(null)}
                className="text-gray-400 hover:text-gray-600"
              >
                <X className="w-5 h-5" />
              </button>
            </div>
            
            <div className="p-6">
              <div className="space-y-4">
                <div>
                  <h4 className="font-medium text-gray-900">Type</h4>
                  <p className="text-sm text-gray-600">{selectedResult.type}</p>
                </div>
                
                <div>
                  <h4 className="font-medium text-gray-900">Severity</h4>
                  <span className={`inline-flex px-2 py-1 text-xs font-medium rounded-full ${
                    selectedResult.severity === 'critical' ? 'bg-red-100 text-red-800' :
                    selectedResult.severity === 'high' ? 'bg-orange-100 text-orange-800' :
                    selectedResult.severity === 'medium' ? 'bg-yellow-100 text-yellow-800' :
                    'bg-blue-100 text-blue-800'
                  }`}>
                    {selectedResult.severity}
                  </span>
                </div>
                
                <div>
                  <h4 className="font-medium text-gray-900">Location</h4>
                  <p className="text-sm text-gray-600">
                    Line {selectedResult.line} in {selectedResult.file}
                  </p>
                </div>
                
                <div>
                  <h4 className="font-medium text-gray-900">Description</h4>
                  <p className="text-sm text-gray-600">{selectedResult.description}</p>
                </div>
                
                <div>
                  <h4 className="font-medium text-gray-900">Recommendation</h4>
                  <p className="text-sm text-gray-600">{selectedResult.recommendation}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
```

### 3. web/src/components/CodeEditor.tsx (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\web\src\components\CodeEditor.tsx`

**Complete Content**:
```typescript
import React, { useRef, useEffect, useState } from 'react';
import Editor from '@monaco-editor/react';
import { Loader2, Upload, FileText } from 'lucide-react';

interface CodeEditorProps {
  onScan: (code: string) => void;
  isScanning: boolean;
}

export function CodeEditor({ onScan, isScanning }: CodeEditorProps) {
  const editorRef = useRef<any>(null);
  const [code, setCode] = useState(`#[contract]
pub struct TokenContract {
    #[contractstate]
    admin: Address,
    #[contractstate]
    total_supply: u64,
}

#[contractimpl]
impl TokenContract {
    pub fn initialize(env: &Env, admin: Address, initial_supply: u64) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TotalSupply, &initial_supply);
    }
    
    pub fn transfer(env: &Env, from: Address, to: Address, amount: u64) -> Result<(), &'static str> {
        let from_balance = env.storage().instance().get(&DataKey::Balance(from));
        if from_balance < amount {
            return Err("Insufficient balance");
        }
        
        env.storage().instance().set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage().instance().set(&DataKey::Balance(to), &amount);
        
        Ok(())
    }
}`);

  const handleScan = () => {
    if (editorRef.current) {
      const editorCode = editorRef.current.getValue();
      onScan(editorCode);
    }
  };

  const handleFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => {
        const content = e.target?.result as string;
        setCode(content);
      };
      reader.readAsText(file);
    }
  };

  const loadExample = () => {
    const vulnerableExample = `#[contract]
pub struct VulnerableContract {
    #[contractstate]
    admin: Address,
}

#[contractimpl]
impl VulnerableContract {
    // VULNERABILITY: Missing access control
    pub fn set_admin(env: &Env, new_admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }
    
    // VULNERABILITY: Integer overflow
    pub unsafe fn add_balance(env: &Env, user: Address, amount: u64) {
        let current = env.storage().instance().get(&DataKey::Balance(user));
        let new_balance = current + amount; // Potential overflow
        env.storage().instance().set(&DataKey::Balance(user), &new_balance);
    }
    
    // VULNERABILITY: Reentrancy
    pub fn withdraw(env: &Env, amount: u64) -> Result<(), &'static str> {
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
}`;
    setCode(vulnerableExample);
  };

  useEffect(() => {
    // Configure Monaco for Rust
    if (typeof window !== 'undefined') {
      const monaco = require('monaco-editor');
      
      // Register Rust language
      monaco.languages.register({ id: 'rust' });
      
      monaco.languages.setMonarchTokensProvider('rust', {
        tokenizer: {
          root: [
            [/\b(?:fn|let|mut|if|else|match|return|pub|impl|struct|enum|use|mod|crate|super|self|loop|while|for|in|break|continue|unsafe|async|await|move|ref|const|static|trait|where|type)\b/, 'keyword'],
            [/\b(?:u8|u16|u32|u64|i8|i16|i32|i64|f32|f64|bool|char|str|String|Vec|Option|Result|Address|Env|Symbol|Map|BytesN)\b/, 'type'],
            [/\b(?:true|false|Some|None|Ok|Err)\b/, 'builtin'],
            [/\b\d+\.\d+/, 'number.float'],
            [/\b\d+/, 'number'],
            [/"([^"\\]|\\.)*$/, 'string.invalid'],
            [/"/, 'string.@start'],
            [/"/, 'string.@end', '@pop'],
            [/\/\/.*$/, 'comment'],
            [/\/\*/, 'comment.@start'],
            [/\/\*/, 'comment.@end', '@pop'],
          ],
        },
      });

      monaco.languages.setLanguageConfiguration('rust', {
        comments: {
          lineComment: '//',
          blockComment: ['/*', '*/'],
        },
        brackets: [
          ['{', '}'],
          ['[', ']'],
          ['(', ')'],
        ],
        autoClosingPairs: [
          { open: '{', close: '}' },
          { open: '[', close: ']' },
          { open: '(', close: ')' },
          { open: '"', close: '"', notIn: ['string'] },
        ],
      });
    }
  }, []);

  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <div className="flex space-x-2">
          <label className="flex items-center space-x-2 px-3 py-2 bg-white border border-gray-300 rounded-md hover:bg-gray-50 cursor-pointer">
            <Upload className="w-4 h-4" />
            <span className="text-sm">Upload File</span>
            <input
              type="file"
              accept=".rs"
              onChange={handleFileUpload}
              className="hidden"
            />
          </label>
          
          <button
            onClick={loadExample}
            className="flex items-center space-x-2 px-3 py-2 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
          >
            <FileText className="w-4 h-4" />
            <span className="text-sm">Load Vulnerable Example</span>
          </button>
        </div>
        
        <button
          onClick={handleScan}
          disabled={isScanning}
          className="flex items-center space-x-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          {isScanning ? (
            <>
              <Loader2 className="w-4 h-4 animate-spin" />
              <span>Scanning...</span>
            </>
          ) : (
            <>
              <Shield className="w-4 h-4" />
              <span>Scan Contract</span>
            </>
          )}
        </button>
      </div>

      <div className="border border-gray-300 rounded-lg overflow-hidden">
        <Editor
          height="500px"
          defaultLanguage="rust"
          value={code}
          onChange={(value) => setCode(value || '')}
          theme="vs-dark"
          options={{
            minimap: { enabled: false },
            fontSize: 14,
            lineNumbers: 'on',
            scrollBeyondLastLine: false,
            automaticLayout: true,
            tabSize: 4,
            insertSpaces: true,
            wordWrap: 'on',
          }}
          onMount={(editor) => {
            editorRef.current = editor;
          }}
        />
      </div>
      
      <div className="flex justify-between items-center text-sm text-gray-600">
        <span>Rust Soroban Contract</span>
        <span>{code.split('\n').length} lines</span>
      </div>
    </div>
  );
}
```

### 4. web/src/components/ResultsPanel.tsx (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\web\src\components\ResultsPanel.tsx`

**Complete Content**:
```typescript
import React, { useState } from 'react';
import { ScanResult } from '../types';
import { AlertTriangle, CheckCircle, XCircle, Info, ChevronDown, ChevronRight } from 'lucide-react';

interface ResultsPanelProps {
  results: ScanResult[];
  onResultSelect: (result: ScanResult) => void;
  isScanning: boolean;
}

export function ResultsPanel({ results, onResultSelect, isScanning }: ResultsPanelProps) {
  const [expandedResults, setExpandedResults] = useState<Set<number>>(new Set());

  const toggleExpanded = (index: number) => {
    const newExpanded = new Set(expandedResults);
    if (newExpanded.has(index)) {
      newExpanded.delete(index);
    } else {
      newExpanded.add(index);
    }
    setExpandedResults(newExpanded);
  };

  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case 'critical':
        return <XCircle className="w-5 h-5 text-red-600" />;
      case 'high':
        return <AlertTriangle className="w-5 h-5 text-orange-600" />;
      case 'medium':
        return <AlertTriangle className="w-5 h-5 text-yellow-600" />;
      case 'low':
        return <Info className="w-5 h-5 text-blue-600" />;
      default:
        return <CheckCircle className="w-5 h-5 text-green-600" />;
    }
  };

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'critical':
        return 'border-red-200 bg-red-50';
      case 'high':
        return 'border-orange-200 bg-orange-50';
      case 'medium':
        return 'border-yellow-200 bg-yellow-50';
      case 'low':
        return 'border-blue-200 bg-blue-50';
      default:
        return 'border-green-200 bg-green-50';
    }
  };

  if (isScanning) {
    return (
      <div className="flex flex-col items-center justify-center h-64 space-y-4">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
        <p className="text-gray-600">Scanning contract for vulnerabilities...</p>
      </div>
    );
  }

  if (results.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center h-64 space-y-4">
        <CheckCircle className="w-16 h-16 text-green-600" />
        <div className="text-center">
          <h3 className="text-lg font-medium text-gray-900">No vulnerabilities found</h3>
          <p className="mt-1 text-sm text-gray-600">
            Your contract appears to be secure!
          </p>
        </div>
      </div>
    );
  }

  const criticalCount = results.filter(r => r.severity === 'critical').length;
  const highCount = results.filter(r => r.severity === 'high').length;
  const mediumCount = results.filter(r => r.severity === 'medium').length;
  const lowCount = results.filter(r => r.severity === 'low').length;

  return (
    <div className="space-y-4">
      {/* Summary */}
      <div className="bg-white border border-gray-200 rounded-lg p-4">
        <h3 className="text-lg font-semibold text-gray-900 mb-3">Scan Summary</h3>
        <div className="grid grid-cols-2 gap-4">
          <div className="flex items-center space-x-2">
            <XCircle className="w-5 h-5 text-red-600" />
            <span className="text-sm font-medium">Critical: {criticalCount}</span>
          </div>
          <div className="flex items-center space-x-2">
            <AlertTriangle className="w-5 h-5 text-orange-600" />
            <span className="text-sm font-medium">High: {highCount}</span>
          </div>
          <div className="flex items-center space-x-2">
            <AlertTriangle className="w-5 h-5 text-yellow-600" />
            <span className="text-sm font-medium">Medium: {mediumCount}</span>
          </div>
          <div className="flex items-center space-x-2">
            <Info className="w-5 h-5 text-blue-600" />
            <span className="text-sm font-medium">Low: {lowCount}</span>
          </div>
        </div>
      </div>

      {/* Results List */}
      <div className="space-y-2">
        <h3 className="text-lg font-semibold text-gray-900">Vulnerabilities Found</h3>
        
        {results.map((result, index) => (
          <div
            key={index}
            className={`border rounded-lg overflow-hidden ${getSeverityColor(result.severity)}`}
          >
            <div
              className="p-4 cursor-pointer hover:bg-opacity-80 transition-colors"
              onClick={() => onResultSelect(result)}
            >
              <div className="flex items-start justify-between">
                <div className="flex items-start space-x-3 flex-1">
                  {getSeverityIcon(result.severity)}
                  
                  <div className="flex-1 min-w-0">
                    <h4 className="text-sm font-medium text-gray-900 truncate">
                      {result.type}
                    </h4>
                    <p className="mt-1 text-sm text-gray-600 line-clamp-2">
                      {result.description}
                    </p>
                    <div className="mt-2 flex items-center space-x-4 text-xs text-gray-500">
                      <span>Line {result.line}</span>
                      <span>{result.file}</span>
                    </div>
                  </div>
                </div>
                
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    toggleExpanded(index);
                  }}
                  className="ml-2 text-gray-400 hover:text-gray-600"
                >
                  {expandedResults.has(index) ? (
                    <ChevronDown className="w-4 h-4" />
                  ) : (
                    <ChevronRight className="w-4 h-4" />
                  )}
                </button>
              </div>
            </div>
            
            {expandedResults.has(index) && (
              <div className="px-4 pb-4 border-t border-gray-200 bg-white bg-opacity-50">
                <div className="mt-3 space-y-3">
                  <div>
                    <h5 className="text-sm font-medium text-gray-900">Description</h5>
                    <p className="mt-1 text-sm text-gray-600">{result.description}</p>
                  </div>
                  
                  <div>
                    <h5 className="text-sm font-medium text-gray-900">Recommendation</h5>
                    <p className="mt-1 text-sm text-gray-600">{result.recommendation}</p>
                  </div>
                  
                  {result.code && (
                    <div>
                      <h5 className="text-sm font-medium text-gray-900">Vulnerable Code</h5>
                      <pre className="mt-1 p-2 bg-gray-100 rounded text-xs overflow-x-auto">
                        <code>{result.code}</code>
                      </pre>
                    </div>
                  )}
                  
                  <div>
                    <h5 className="text-sm font-medium text-gray-900">References</h5>
                    <ul className="mt-1 text-sm text-gray-600 list-disc list-inside">
                      {result.references?.map((ref, refIndex) => (
                        <li key={refIndex}>
                          <a
                            href={ref}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-blue-600 hover:text-blue-800 underline"
                          >
                            {ref}
                          </a>
                        </li>
                      ))}
                    </ul>
                  </div>
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
```

### 5. web/src/types/index.ts (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\web\src\types\index.ts`

**Complete Content**:
```typescript
export interface ScanResult {
  id: string;
  type: string;
  severity: 'critical' | 'high' | 'medium' | 'low';
  description: string;
  recommendation: string;
  file: string;
  line: number;
  column?: number;
  code?: string;
  references?: string[];
  ruleId: string;
  confidence: number;
}

export interface ScannerConfig {
  severityThreshold: 'low' | 'medium' | 'high' | 'critical';
  enabledRules: string[];
  customRules: CustomRule[];
  outputFormat: 'console' | 'json' | 'html' | 'sarif';
  includePaths: string[];
  excludePaths: string[];
}

export interface CustomRule {
  id: string;
  name: string;
  description: string;
  pattern: string;
  severity: 'critical' | 'high' | 'medium' | 'low';
  enabled: boolean;
}

export interface ScanReport {
  scanId: string;
  timestamp: string;
  contractName?: string;
  results: ScanResult[];
  summary: {
    total: number;
    critical: number;
    high: number;
    medium: number;
    low: number;
  };
  config: ScannerConfig;
  scanDuration: number;
}

export interface VulnerabilityPattern {
  id: string;
  name: string;
  description: string;
  severity: 'critical' | 'high' | 'medium' | 'low';
  category: string;
  examples: string[];
  mitigations: string[];
  references: string[];
}

export interface Rule {
  id: string;
  name: string;
  description: string;
  severity: 'critical' | 'high' | 'medium' | 'low';
  category: string;
  enabled: boolean;
  pattern: string;
  recommendation: string;
}
```

### 6. src/wasm.rs (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\src\wasm.rs`

**Complete Content**:
```rust
use wasm_bindgen::prelude::*;
use crate::SecurityScanner;
use crate::config::ScannerConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Enable console logging for WASM
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize)]
pub struct WasmScanResult {
    pub id: String,
    pub vulnerability_type: String,
    pub severity: String,
    pub description: String,
    pub recommendation: String,
    pub file: String,
    pub line: usize,
    pub column: Option<usize>,
    pub code: Option<String>,
    pub rule_id: String,
    pub confidence: f64,
}

#[derive(Serialize, Deserialize)]
pub struct WasmScanReport {
    pub scan_id: String,
    pub timestamp: String,
    pub results: Vec<WasmScanResult>,
    pub summary: WasmSummary,
    pub scan_duration: u64,
}

#[derive(Serialize, Deserialize)]
pub struct WasmSummary {
    pub total: usize,
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
}

#[derive(Serialize, Deserialize)]
pub struct WasmConfig {
    pub severity_threshold: String,
    pub enabled_rules: Vec<String>,
    pub output_format: String,
}

impl Default for WasmConfig {
    fn default() -> Self {
        Self {
            severity_threshold: "low".to_string(),
            enabled_rules: vec![],
            output_format: "json".to_string(),
        }
    }
}

#[wasm_bindgen]
pub struct WasmScanner {
    scanner: SecurityScanner,
}

#[wasm_bindgen]
impl WasmScanner {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WasmScanner, JsValue> {
        console_log!("Initializing WASM scanner");
        
        let config = ScannerConfig::default();
        let scanner = SecurityScanner::new(config)
            .map_err(|e| JsValue::from_str(&format!("Failed to initialize scanner: {}", e)))?;
        
        Ok(WasmScanner { scanner })
    }

    #[wasm_bindgen]
    pub fn scan_contract(&mut self, code: &str, config: Option<WasmConfig>) -> Result<JsValue, JsValue> {
        console_log!("Starting contract scan");
        let start_time = web_sys::window()
            .unwrap()
            .performance()
            .unwrap()
            .now() as u64;

        // Update scanner config if provided
        if let Some(wasm_config) = config {
            self.update_config(wasm_config)?;
        }

        // Perform scan
        let results = self.scanner.scan_contract_code(code)
            .map_err(|e| JsValue::from_str(&format!("Scan failed: {}", e)))?;

        // Convert results to WASM format
        let wasm_results: Vec<WasmScanResult> = results.into_iter()
            .map(|r| WasmScanResult {
                id: r.id,
                vulnerability_type: r.rule_name,
                severity: format!("{:?}", r.severity).to_lowercase(),
                description: r.description,
                recommendation: r.recommendation,
                file: r.location.file_path,
                line: r.location.line,
                column: Some(r.location.column),
                code: r.code_snippet,
                rule_id: r.rule_id,
                confidence: r.confidence,
            })
            .collect();

        // Create summary
        let summary = WasmSummary {
            total: wasm_results.len(),
            critical: wasm_results.iter().filter(|r| r.severity == "critical").count(),
            high: wasm_results.iter().filter(|r| r.severity == "high").count(),
            medium: wasm_results.iter().filter(|r| r.severity == "medium").count(),
            low: wasm_results.iter().filter(|r| r.severity == "low").count(),
        };

        let end_time = web_sys::window()
            .unwrap()
            .performance()
            .unwrap()
            .now() as u64;

        let report = WasmScanReport {
            scan_id: uuid::Uuid::new_v4().to_string(),
            timestamp: web_sys::Date::new_0().to_iso_string(),
            results: wasm_results,
            summary,
            scan_duration: end_time - start_time,
        };

        // Convert to JSON and return as JsValue
        let json = serde_json::to_string(&report)
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
    pub fn validate_config(&self, config: WasmConfig) -> Result<JsValue, JsValue> {
        // Validate configuration
        let mut errors = Vec::new();

        if !["low", "medium", "high", "critical"].contains(&config.severity_threshold.as_str()) {
            errors.push("Invalid severity threshold".to_string());
        }

        if !["console", "json", "html", "sarif"].contains(&config.output_format.as_str()) {
            errors.push("Invalid output format".to_string());
        }

        let result = if errors.is_empty() {
            serde_json::json!({
                "valid": true,
                "errors": []
            })
        } else {
            serde_json::json!({
                "valid": false,
                "errors": errors
            })
        };

        let json = serde_json::to_string(&result)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize validation result: {}", e)))?;
        
        Ok(JsValue::from_str(&json))
    }

    fn update_config(&mut self, wasm_config: WasmConfig) -> Result<(), JsValue> {
        // Convert WASM config to ScannerConfig
        let mut config = self.scanner.config().clone();

        // Update severity threshold
        config.severity_threshold = match wasm_config.severity_threshold.as_str() {
            "low" => crate::config::Severity::Low,
            "medium" => crate::config::Severity::Medium,
            "high" => crate::config::Severity::High,
            "critical" => crate::config::Severity::Critical,
            _ => return Err(JsValue::from_str("Invalid severity threshold")),
        };

        // Update enabled rules
        config.enabled_rules = wasm_config.enabled_rules;

        // Update output format
        config.output_format = match wasm_config.output_format.as_str() {
            "console" => crate::config::ReportFormat::Console,
            "json" => crate::config::ReportFormat::Json,
            "html" => crate::config::ReportFormat::Html,
            "sarif" => crate::config::ReportFormat::Sarif,
            _ => return Err(JsValue::from_str("Invalid output format")),
        };

        // Apply new configuration
        self.scanner.update_config(config)
            .map_err(|e| JsValue::from_str(&format!("Failed to update config: {}", e)))?;

        Ok(())
    }
}

// Utility functions
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn get_supported_languages() -> JsValue {
    let languages = vec!["rust", "soroban"];
    JsValue::from_serde(&languages).unwrap()
}

// Initialize panic hook for better error reporting
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}
```

### 7. Cargo.toml Updates

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\Cargo.toml`

**Add to dependencies**:
```toml
[dependencies]
# ... existing dependencies ...
wasm-bindgen = { version = "0.2", features = ["serde-serialize"] }
wasm-bindgen-futures = "0.4"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["console", "Performance", "Date"] }
serde-wasm-bindgen = "0.6"
console_error_panic_hook = "0.1"
uuid = { version = "1.0", features = ["v4", "wasm-bindgen"] }

[lib]
crate-type = ["cdylib", "rlib"]

[[bin]]
name = "soroban-security-guard"
path = "src/main.rs"
```

### 8. scripts/build-web.sh (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\scripts\build-web.sh`

**Complete Content**:
```bash
#!/bin/bash

# Build script for Soroban Security Guard Web Interface

set -e

echo "🛡️  Building Soroban Security Guard Web Interface..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the project root directory"
    exit 1
fi

# Install wasm target if not already installed
echo "📦 Installing wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen CLI if not already installed
echo "📦 Installing wasm-bindgen CLI..."
cargo install wasm-bindgen-cli --version 0.2.87

# Build the WASM library
echo "🔨 Building WASM library..."
cargo build --target wasm32-unknown-unknown --release

# Generate JS bindings
echo "🔗 Generating JavaScript bindings..."
wasm-bindgen \
    --target web \
    --out-dir web/public \
    --out-name soroban_security_guard \
    --no-typescript \
    target/wasm32-unknown-unknown/release/soroban_security_guard.wasm

# Build the web frontend
echo "🌐 Building web frontend..."
cd web
npm install
npm run build
cd ..

echo "✅ Web interface built successfully!"
echo ""
echo "📁 Build artifacts:"
echo "  - web/dist/ - Web application"
echo "  - web/public/soroban_security_guard.wasm - WASM module"
echo "  - web/public/soroban_security_guard.js - JavaScript bindings"
echo ""
echo "🚀 To run locally:"
echo "  cd web && npm run dev"
echo ""
echo "🌍 To deploy:"
echo "  Deploy the 'web/dist' directory to your web server"
```

## 📁 Folder Structure After Implementation

```
soroban-guard/
├── 📄 src/
│   ├── 📄 wasm.rs (new)
│   ├── 📄 lib.rs (updated)
│   └── 📄 ... (existing files)
├── 📄 web/ (new directory)
│   ├── 📄 package.json (new)
│   ├── 📄 vite.config.ts (new)
│   ├── 📄 tsconfig.json (new)
│   ├── 📄 src/
│   │   ├── 📄 App.tsx (new)
│   │   ├── 📄 components/
│   │   │   ├── 📄 CodeEditor.tsx (new)
│   │   │   ├── 📄 ResultsPanel.tsx (new)
│   │   │   ├── 📄 ConfigPanel.tsx (new)
│   │   │   └── 📄 ReportViewer.tsx (new)
│   │   ├── 📄 hooks/
│   │   │   ├── 📄 useScanner.ts (new)
│   │   │   └── 📄 useConfig.ts (new)
│   │   ├── 📄 services/
│   │   │   └── 📄 scannerService.ts (new)
│   │   └── 📄 types/
│   │       └── 📄 index.ts (new)
│   └── 📄 public/
│       ├── 📄 index.html (new)
│       └── 📄 favicon.ico (new)
├── 📄 scripts/
│   └── 📄 build-web.sh (new)
└── 📄 Cargo.toml (updated)
```

## 🚀 Implementation Steps

1. **Create web directory structure** - Set up React/TypeScript project
2. **Implement core components** - CodeEditor, ResultsPanel, ConfigPanel
3. **Create WASM interface** - Bridge between Rust scanner and JavaScript
4. **Set up build system** - Vite configuration and build scripts
5. **Add Monaco editor** - Rust syntax highlighting and code editing
6. **Implement scanner service** - WASM integration and error handling
7. **Create responsive UI** - Mobile-friendly interface with Tailwind CSS
8. **Add report generation** - Multiple export formats
9. **Test integration** - End-to-end testing of web interface

## ✅ Success Metrics

- [ ] Web interface loads in <3 seconds
- [ ] Scanning completes in <10 seconds for typical contracts
- [ ] All vulnerability types detected correctly
- [ ] Mobile-responsive design works on all screen sizes
- [ ] WASM bundle size <2MB
- [ ] No memory leaks during extended use

## 🎯 Definition of Done

This issue is **COMPLETE** when:
1. All web components are implemented and functional
2. WASM integration works seamlessly
3. Scanning results match CLI output
4. Build system works across platforms
5. User interface is intuitive and responsive
6. Documentation covers setup and usage
