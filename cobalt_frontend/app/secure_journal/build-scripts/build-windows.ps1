# Build Windows executables
Write-Host "Building for Windows..." -ForegroundColor Green

# Build backend
cargo build --release --target x86_64-pc-windows-gnu --manifest-path backend/Cargo.toml
# Build frontend
cargo build --release --target x86_64-pc-windows-gnu --manifest-path frontend/Cargo.toml

# Create distribution directory
$distDir = "target/dist/windows"
New-Item -ItemType Directory -Force -Path $distDir

# Copy binaries
Copy-Item "target/x86_64-pc-windows-gnu/release/backend.exe" "$distDir/"
Copy-Item "target/x86_64-pc-windows-gnu/release/frontend.exe" "$distDir/" -ErrorAction SilentlyContinue

Write-Host "Windows build complete! Files in $distDir" -ForegroundColor Green