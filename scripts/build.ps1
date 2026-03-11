# Soroban Security Guard Build Script for PowerShell

Write-Host "🛡️  Building Soroban Security Guard..." -ForegroundColor Cyan

# Check if Rust is installed
try {
    $rustVersion = cargo --version 2>$null
    Write-Host "✅ Found Rust: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Rust/Cargo not found. Please install Rust first." -ForegroundColor Red
    Write-Host "Visit: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

Write-Host "📦 Installing dependencies..." -ForegroundColor Blue
cargo fetch

Write-Host "🔧 Building project..." -ForegroundColor Blue
cargo build --release

Write-Host "🧪 Running tests..." -ForegroundColor Blue
cargo test

Write-Host "✅ Build completed successfully!" -ForegroundColor Green
Write-Host "📁 Binary location: target/release/soroban-security-guard.exe" -ForegroundColor Cyan
