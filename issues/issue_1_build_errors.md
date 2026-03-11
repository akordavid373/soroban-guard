---
title: Fix Windows build errors due to missing Visual Studio C++ build tools
labels: bug, windows, build, help-wanted
assignees: []
---

## 🚨 Bug Description

The project fails to build on Windows systems that don't have Visual Studio C++ build tools installed. This creates a significant barrier for Windows developers who want to use the tool.

## 📁 Files to Modify

### Primary Files
```
📄 README.md
📄 build.rs (CREATE NEW)
📄 .github/workflows/ci.yml
📄 docs/windows-setup.md (CREATE NEW)
📄 scripts/setup-windows.ps1 (CREATE NEW)
```

### Secondary Files
```
📄 Cargo.toml (optional)
📄 src/main.rs (error handling)
```

## 🎯 Acceptance Criteria

### ✅ MUST HAVE (High Priority)
- [ ] **README.md** - Add comprehensive Windows setup section at top of file
- [ ] **build.rs** - Create new build script with Windows detection
- [ ] **docs/windows-setup.md** - Create detailed Windows setup guide
- [ ] **scripts/setup-windows.ps1** - Create automated Windows setup script

### ✅ SHOULD HAVE (Medium Priority)
- [ ] **.github/workflows/ci.yml** - Add Windows testing matrix
- [ ] **scripts/setup-windows.ps1** - Test script on fresh Windows machine
- [ ] **README.md** - Add troubleshooting section

### ✅ COULD HAVE (Low Priority)
- [ ] **Cargo.toml** - Add Windows-specific dependencies if needed
- [ ] **src/main.rs** - Add graceful error handling for Windows issues

## 🔧 Implementation Details

### 1. README.md Modifications

**Location**: Add after existing installation section

**Content to Add**:
```markdown
## 🪟 Windows Setup

### ⚡ Quick Setup (Recommended)
1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
2. In installer, select:
   - ✅ C++ build tools
   - ✅ Windows 10/11 SDK
   - ✅ MSVC v143 - VS 2022 C++ x64/x86 build tools

### 🔧 Alternative Options
- **Full Visual Studio**: Install Community Edition with "Desktop development with C++"
- **GNU Toolchain**: Install MSYS2 and run:
  ```bash
  rustup toolchain install stable-x86_64-pc-windows-gnu
  rustup default stable-x86_64-pc-windows-gnu
  ```

### ✅ Verification
```bash
rustc --version
cargo --version
cargo build --release
```

### 🐛 Troubleshooting
- **Linker errors**: Install Visual Studio Build Tools
- **Permission errors**: Run PowerShell as Administrator
- **Path issues**: Restart terminal after installation
```

### 2. build.rs (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\build.rs`

**Complete Code**:
```rust
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    #[cfg(target_os = "windows")]
    {
        // Check for MSVC toolchain
        if !check_msvc_toolchain() {
            eprintln!("❌ Windows Build Error Detected!");
            eprintln!("📋 Required: Visual Studio Build Tools with C++ support");
            eprintln!("🔗 Setup Guide: https://github.com/akordavid373/soroban-guard/wiki/Windows-Setup");
            eprintln!("💡 Quick Fix: Install Visual Studio Build Tools from: https://visualstudio.microsoft.com/downloads/");
            std::process::exit(1);
        } else {
            println!("✅ Windows build environment verified");
        }
    }
}

#[cfg(target_os = "windows")]
fn check_msvc_toolchain() -> bool {
    // Try to run link.exe to check if MSVC toolchain is available
    Command::new("link.exe")
        .arg("/?")
        .output()
        .is_ok()
}
```

### 3. docs/windows-setup.md (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\docs\windows-setup.md`

**Complete Content**:
```markdown
# Windows Setup Guide for Soroban Security Guard

## 🎯 Prerequisites

- Windows 10/11
- PowerShell (Administrator privileges recommended)
- Internet connection

## 📦 Installation Options

### Option 1: Visual Studio Build Tools (Recommended)

#### Step 1: Download
1. Visit [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
2. Download "Build Tools for Visual Studio 2022"

#### Step 2: Install
1. Run the installer
2. Select "C++ build tools" workload
3. Ensure these components are checked:
   - ✅ MSVC v143 - VS 2022 C++ x64/x86 build tools
   - ✅ Windows 10/11 SDK
   - ✅ CMake tools
4. Click "Install"

#### Step 3: Verify
```powershell
# Open new PowerShell terminal
rustc --version
cargo --version
```

### Option 2: Full Visual Studio

#### Step 1: Download
1. Download [Visual Studio Community](https://visualstudio.microsoft.com/vs/community/)
2. Run the installer

#### Step 2: Install Workloads
1. Select "Desktop development with C++"
2. Ensure individual components include:
   - ✅ MSVC v143 build tools
   - ✅ Windows 10/11 SDK
   - ✅ Git for Windows

#### Step 3: Verify
```powershell
rustc --version
cargo build --release
```

### Option 3: GNU Toolchain (Advanced)

#### Step 1: Install MSYS2
1. Download [MSYS2](https://www.msys2.org/)
2. Run installer with default settings

#### Step 2: Install GNU Toolchain
```powershell
# In MSYS2 terminal
pacman -Syu
pacman -Su
pacman -S --needed base-devel mingw-w64-x86_64-toolchain
```

#### Step 3: Configure Rust
```powershell
# In PowerShell (not MSYS2)
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

## 🧪 Build and Test

### Clone and Build
```powershell
git clone https://github.com/akordavid373/soroban-guard.git
cd soroban-guard
cargo build --release
```

### Run Tests
```powershell
cargo test
cargo run --bin soroban-security-guard -- --help
```

### Test with Example
```powershell
cargo run --bin soroban-security-guard -- scan examples/vulnerable_contract.rs
```

## 🐛 Common Issues and Solutions

### Issue: "link.exe not found"
**Cause**: Missing Visual Studio C++ build tools
**Solution**: Install Visual Studio Build Tools (Option 1)

### Issue: "Permission denied"
**Cause**: PowerShell not running as administrator
**Solution**: Right-click PowerShell → "Run as administrator"

### Issue: "Command not found"
**Cause**: Rust not in PATH
**Solution**: Restart terminal or restart computer

### Issue: "Build fails with strange errors"
**Cause**: Corrupted installation
**Solution**: 
```powershell
rustup self update
rustup update stable
cargo clean
```

## 🔄 Environment Variables

Check if these are set correctly:
```powershell
echo $env:PATH
echo $env:RUSTUP_HOME
echo $env:CARGO_HOME
```

## 📞 Getting Help

1. **GitHub Issues**: [Create new issue](https://github.com/akordavid373/soroban-guard/issues)
2. **Discord**: Join Rust community
3. **Stack Overflow**: Tag with `rust` and `windows`

## ✅ Success Checklist

- [ ] Visual Studio Build Tools installed
- [ ] Rust toolchain working
- [ ] `cargo build` succeeds
- [ ] `cargo test` passes
- [ ] CLI tool runs successfully
- [ ] Example contracts scan correctly

## 📚 Additional Resources

- [Rust Windows Installation](https://www.rust-lang.org/tools/install)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
- [MSYS2 Documentation](https://www.msys2.org/docs/)
- [Rustup Book](https://rust-lang.github.io/rustup/)
```

### 4. scripts/setup-windows.ps1 (CREATE NEW)

**File Path**: `C:\Users\USER\CascadeProjects\soroban-security-guard\scripts\setup-windows.ps1`

**Complete Code**:
```powershell
#!/usr/bin/env pwsh

# Soroban Security Guard Windows Setup Script
# Run as Administrator for best results

Write-Host "🛡️  Soroban Security Guard Windows Setup" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Check if running as Administrator
if (-NOT ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Warning "Not running as Administrator. Some operations may fail."
    $continue = Read-Host "Continue anyway? (y/N)"
    if ($continue -ne "y") {
        exit 1
    }
}

# Check Rust installation
Write-Host "🔍 Checking Rust installation..." -ForegroundColor Blue
try {
    $rustVersion = rustc --version 2>$null
    Write-Host "✅ Rust found: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Rust not found. Installing Rust..." -ForegroundColor Red
    Write-Host "📥 Downloading Rust installer..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
    Start-Process -FilePath "rustup-init.exe" -ArgumentList "-y" -Wait
    Remove-Item "rustup-init.exe"
    
    # Refresh environment variables
    $env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
}

# Check Visual Studio Build Tools
Write-Host "🔍 Checking Visual Studio Build Tools..." -ForegroundColor Blue
try {
    $linkVersion = link.exe 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Visual Studio Build Tools found" -ForegroundColor Green
    }
} catch {
    Write-Host "❌ Visual Studio Build Tools not found" -ForegroundColor Red
    Write-Host "📥 Please install Visual Studio Build Tools:" -ForegroundColor Yellow
    Write-Host "   https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022" -ForegroundColor Yellow
    Write-Host "   Select 'C++ build tools' workload" -ForegroundColor Yellow
    
    $continue = Read-Host "Continue setup anyway? (y/N)"
    if ($continue -ne "y") {
        exit 1
    }
}

# Clone repository if not already present
if (-not (Test-Path "soroban-guard")) {
    Write-Host "📥 Cloning Soroban Security Guard repository..." -ForegroundColor Blue
    git clone https://github.com/akordavid373/soroban-guard.git
} else {
    Write-Host "✅ Repository already exists" -ForegroundColor Green
}

# Build the project
Write-Host "🔨 Building Soroban Security Guard..." -ForegroundColor Blue
Set-Location soroban-guard

try {
    cargo build --release
    Write-Host "✅ Build successful!" -ForegroundColor Green
} catch {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    Write-Host "🐛 Error: $LASTEXITCODE" -ForegroundColor Red
    Write-Host "💡 Try running setup as Administrator" -ForegroundColor Yellow
    exit 1
}

# Run tests
Write-Host "🧪 Running tests..." -ForegroundColor Blue
try {
    cargo test
    Write-Host "✅ Tests passed!" -ForegroundColor Green
} catch {
    Write-Host "❌ Tests failed!" -ForegroundColor Red
    Write-Host "🐛 Error: $LASTEXITCODE" -ForegroundColor Red
}

# Test CLI
Write-Host "🔍 Testing CLI..." -ForegroundColor Blue
try {
    $version = ./target/release/soroban-security-guard.exe --version
    Write-Host "✅ CLI working: $version" -ForegroundColor Green
} catch {
    Write-Host "❌ CLI test failed!" -ForegroundColor Red
}

# Test with example
Write-Host "🧪 Testing with example contract..." -ForegroundColor Blue
try {
    $result = ./target/release/soroban-security-guard.exe scan examples/vulnerable_contract.rs --output json
    Write-Host "✅ Example scan successful!" -ForegroundColor Green
} catch {
    Write-Host "❌ Example scan failed!" -ForegroundColor Red
}

Write-Host "🎉 Setup completed!" -ForegroundColor Green
Write-Host "📁 Binary location: target/release/soroban-security-guard.exe" -ForegroundColor Cyan
Write-Host "📚 Documentation: docs/windows-setup.md" -ForegroundColor Cyan
Write-Host "🐛 Issues: https://github.com/akordavid373/soroban-guard/issues" -ForegroundColor Cyan
```

### 5. .github/workflows/ci.yml Modifications

**Location**: Add to existing matrix strategy

**Add to jobs.test.strategy.matrix**:
```yaml
matrix:
  os: [ubuntu-latest, windows-latest]
  rust: [stable, beta, nightly]
  include:
    - os: windows-latest
      toolchain: msvc
    - os: windows-latest
      toolchain: gnu
```

## 🧪 Testing Requirements

### Manual Testing
1. **Fresh Windows 10 VM** - Test from clean install
2. **Windows 11** - Test on latest Windows
3. **Different PowerShell versions** - Test 5.1 and 7+
4. **Administrator vs User** - Test both privilege levels

### Automated Testing
1. **GitHub Actions** - Verify CI/CD works
2. **Build Script** - Test build.rs detection
3. **Setup Script** - Test PowerShell script

## 📁 Folder Structure After Implementation

```
soroban-guard/
├── 📄 README.md (updated)
├── 📄 build.rs (new)
├── 📄 docs/
│   └── 📄 windows-setup.md (new)
├── 📄 scripts/
│   └── 📄 setup-windows.ps1 (new)
├── 📄 .github/workflows/
│   └── 📄 ci.yml (updated)
└── 📄 src/main.rs (optional error handling)
```

## 🚀 Deployment Steps

1. **Create build.rs** - Add Windows detection
2. **Update README.md** - Add Windows setup section
3. **Create docs/windows-setup.md** - Comprehensive guide
4. **Create scripts/setup-windows.ps1** - Automated setup
5. **Update .github/workflows/ci.yml** - Add Windows testing
6. **Test on fresh Windows machine** - Verify everything works
7. **Update documentation** - Add troubleshooting tips

## ✅ Success Metrics

- [ ] Windows users can build without errors
- [ ] Setup script works on fresh Windows install
- [ ] CI/CD passes on Windows runners
- [ ] Documentation covers all Windows scenarios
- [ ] Error messages are helpful and actionable

## 🎯 Definition of Done

This issue is **COMPLETE** when:
1. All files listed above are created/modified
2. Windows build works on fresh machine
3. CI/CD passes on Windows
4. Documentation is comprehensive
5. Setup script is tested and working

## 🔗 Related Issues

- None currently, but may affect other Windows-specific issues

## 📞 Help Resources

- **Build.rs issues**: Check Rust build script documentation
- **PowerShell issues**: Check Microsoft PowerShell docs
- **CI/CD issues**: Check GitHub Actions documentation
