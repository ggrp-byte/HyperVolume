# HyperVolume Build Script
# PowerShell script to build the application and create installer

param(
    [string]$Configuration = "Release",
    [switch]$CreateInstaller = $false,
    [switch]$SkipTests = $false
)

Write-Host "Building HyperVolume..." -ForegroundColor Green

# Set error action preference
$ErrorActionPreference = "Stop"

try {
    # Navigate to project directory
    $ProjectRoot = Split-Path -Parent $PSScriptRoot
    Set-Location $ProjectRoot

    Write-Host "Project root: $ProjectRoot" -ForegroundColor Yellow

    # Install Node.js dependencies
    Write-Host "Installing Node.js dependencies..." -ForegroundColor Cyan
    npm install
    if ($LASTEXITCODE -ne 0) {
        throw "npm install failed"
    }

    # Run tests if not skipped
    if (-not $SkipTests) {
        Write-Host "Running tests..." -ForegroundColor Cyan
        Set-Location "src-tauri"
        cargo test
        if ($LASTEXITCODE -ne 0) {
            throw "Tests failed"
        }
        Set-Location $ProjectRoot
    }

    # Build the application
    Write-Host "Building Tauri application..." -ForegroundColor Cyan
    if ($Configuration -eq "Release") {
        npm run tauri build
    } else {
        npm run tauri build -- --debug
    }
    
    if ($LASTEXITCODE -ne 0) {
        throw "Tauri build failed"
    }

    # Copy built executable to installer directory
    $BuiltExe = "src-tauri\target\release\hypervolume-app.exe"
    if ($Configuration -eq "Debug") {
        $BuiltExe = "src-tauri\target\debug\hypervolume-app.exe"
    }

    if (Test-Path $BuiltExe) {
        Write-Host "Copying executable to installer directory..." -ForegroundColor Cyan
        Copy-Item $BuiltExe "installer\hypervolume-app.exe" -Force
        
        # Copy icon
        Copy-Item "src-tauri\icons\icon.ico" "installer\icon.ico" -Force
    } else {
        throw "Built executable not found at $BuiltExe"
    }

    # Create installer if requested
    if ($CreateInstaller) {
        Write-Host "Creating installer..." -ForegroundColor Cyan
        
        # Check if NSIS is available
        $NSISPath = Get-Command "makensis.exe" -ErrorAction SilentlyContinue
        if (-not $NSISPath) {
            # Try common NSIS installation paths
            $CommonPaths = @(
                "${env:ProgramFiles}\NSIS\makensis.exe",
                "${env:ProgramFiles(x86)}\NSIS\makensis.exe"
            )
            
            foreach ($Path in $CommonPaths) {
                if (Test-Path $Path) {
                    $NSISPath = Get-Command $Path
                    break
                }
            }
        }
        
        if ($NSISPath) {
            Set-Location "installer"
            & $NSISPath.Source "installer.nsi"
            if ($LASTEXITCODE -ne 0) {
                throw "NSIS installer creation failed"
            }
            Set-Location $ProjectRoot
            Write-Host "Installer created successfully!" -ForegroundColor Green
        } else {
            Write-Warning "NSIS not found. Please install NSIS to create installer."
            Write-Host "Download from: https://nsis.sourceforge.io/Download" -ForegroundColor Yellow
        }
    }

    Write-Host "Build completed successfully!" -ForegroundColor Green
    
    # Display build artifacts
    Write-Host "`nBuild artifacts:" -ForegroundColor Yellow
    if (Test-Path $BuiltExe) {
        Write-Host "  Executable: $BuiltExe" -ForegroundColor White
    }
    if (Test-Path "installer\HyperVolume-Setup.exe") {
        Write-Host "  Installer: installer\HyperVolume-Setup.exe" -ForegroundColor White
    }

} catch {
    Write-Error "Build failed: $_"
    exit 1
} finally {
    # Return to original directory
    Set-Location $PSScriptRoot
}

