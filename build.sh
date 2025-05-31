# Build the tools first
echo "Building tools..."
RUSTFLAGS="-A warnings" cargo build --release

