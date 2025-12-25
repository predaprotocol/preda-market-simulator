#!/bin/bash

# Preda Market Simulator - Quick Start Script
# This script demonstrates basic usage of the simulator

set -e

echo "🚀 Preda Market Simulator - Quick Start"
echo "========================================"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust/Cargo detected"
echo ""

# Build the project
echo "📦 Building project..."
cargo build --release
echo "✅ Build complete"
echo ""

# Run tests
echo "🧪 Running tests..."
cargo test --quiet
echo "✅ All tests passed"
echo ""

# Run basic simulation example
echo "🎯 Running basic simulation example..."
echo "--------------------------------------"
cargo run --release --example basic_simulation
echo ""

# Run strategy backtest example
echo "📊 Running strategy backtest example..."
echo "--------------------------------------"
cargo run --release --example strategy_backtest
echo ""

# Run scenario comparison example
echo "📈 Running scenario comparison example..."
echo "--------------------------------------"
cargo run --release --example scenario_comparison
echo ""

echo "✅ Quick start complete!"
echo ""
echo "Next steps:"
echo "  - Read the README.md for detailed documentation"
echo "  - Explore examples/ directory for more use cases"
echo "  - Check out CONTRIBUTING.md to contribute"
echo ""
echo "Happy simulating! 🎉"
