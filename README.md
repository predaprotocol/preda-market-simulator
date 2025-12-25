# Preda Market Simulator

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A comprehensive testing suite and simulation framework for time-shifted prediction markets built on Solana.

## Overview

Preda Market Simulator enables developers and researchers to test, validate, and analyze prediction market behavior before deployment. Simulate thousands of market scenarios, backtest trading strategies, stress test oracle configurations, and model belief dynamics with statistical rigor.

## Features

- **🎯 Market Simulation**: Run thousands of market scenarios with configurable parameters
- **📊 Strategy Backtesting**: Test trading strategies against historical or synthetic data
- **🔧 Oracle Stress Testing**: Validate oracle configurations under extreme conditions
- **📈 Belief Dynamics Modeling**: Simulate sentiment shifts, consensus changes, and narrative velocity
- **📉 Statistical Analysis**: Comprehensive performance metrics and analytics
- **🎲 Scenario Generation**: 9 predefined scenarios + custom scenario support
- **⚡ High Performance**: Async Rust implementation optimized for speed
- **🔬 Research Tools**: Export data for external analysis and visualization

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
preda-market-simulator = "0.1.0"
```

## Quick Start

```rust
use preda_market_simulator::{Simulator, SimulationConfig, Scenario};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure simulation
    let config = SimulationConfig::builder()
        .duration_days(30)
        .num_participants(1000)
        .initial_bsi(0.5)
        .volatility(0.2)
        .threshold(0.75)
        .build()?;

    // Run simulation
    let simulator = Simulator::new(config);
    let result = simulator.run(Scenario::BullishTrend).await?;

    // Analyze results
    println!("Final BSI: {}", result.final_bsi);
    println!("Total volume: ${}", result.total_volume);
    println!("Threshold reached: {}", result.threshold_reached);

    Ok(())
}
```

## Scenarios

The simulator includes 9 predefined market scenarios:

| Scenario | Description | Use Case |
|----------|-------------|----------|
| **BullishTrend** | Steady upward trend | Test long strategies |
| **BearishTrend** | Steady downward trend | Test short strategies |
| **Sideways** | High volatility, no direction | Test range-bound strategies |
| **SentimentReversal** | Rapid belief shift | Test reversal detection |
| **ConsensusFormation** | Gradual threshold approach | Test timing precision |
| **HighVolatility** | Extreme price swings | Stress test risk management |
| **LowActivity** | Minimal trading | Test low-liquidity scenarios |
| **FlashCrash** | Sudden sharp drop | Test circuit breakers |
| **ParabolicRise** | Accelerating upward movement | Test momentum strategies |

## Strategy Backtesting

Test your trading strategies before risking capital:

```rust
use preda_market_simulator::{Strategy, StrategyBacktest};

let strategy = Strategy::ThresholdCrossing { threshold: 0.7 };

// Run multiple simulations
let mut returns = Vec::new();
for _ in 0..100 {
    let result = simulator.run(Scenario::Sideways).await?;
    returns.push(result.final_bsi - 0.5);
}

// Calculate performance metrics
let mut backtest = StrategyBacktest::new(strategy.name());
backtest.calculate_metrics(&returns);

println!("Sharpe Ratio: {}", backtest.sharpe_ratio);
println!("Win Rate: {}%", backtest.win_rate * 100.0);
println!("Max Drawdown: {}", backtest.max_drawdown);
```

## Analytics

Compare scenarios and analyze market behavior:

```rust
use preda_market_simulator::Analytics;
use std::collections::HashMap;

// Run simulations for multiple scenarios
let mut results_by_scenario = HashMap::new();
// ... populate with simulation results

// Compare performance
let comparisons = Analytics::compare_scenarios(&results_by_scenario);

for comparison in comparisons {
    println!("Scenario: {}", comparison.scenario_name);
    println!("Resolution Rate: {:.2}%", comparison.metrics.resolution_rate * 100.0);
    println!("Avg Final BSI: {:.4}", comparison.metrics.avg_final_bsi);
}
```

## Configuration Options

| Parameter | Description | Default | Range |
|-----------|-------------|---------|-------|
| `duration_days` | Simulation duration | 30 | 1-365 |
| `num_participants` | Number of traders | 100 | 1-10000 |
| `initial_bsi` | Starting BSI value | 0.5 | 0.0-1.0 |
| `volatility` | Market volatility | 0.1 | 0.0-1.0 |
| `threshold` | Resolution threshold | 0.75 | 0.0-1.0 |
| `persistence_hours` | Threshold persistence | 24 | 1-168 |
| `update_frequency_secs` | BSI update interval | 300 | 1-3600 |
| `seed` | Random seed | None | Any u64 |

## Examples

Run the included examples:

```bash
# Basic simulation
cargo run --example basic_simulation

# Strategy backtesting
cargo run --example strategy_backtest

# Scenario comparison
cargo run --example scenario_comparison
```

## Architecture

```
preda-market-simulator/
├── src/
│   ├── lib.rs              # Main library interface
│   ├── config.rs           # Configuration & builder
│   ├── types.rs            # Core types (BSI, Position, Trade)
│   ├── error.rs            # Error handling
│   ├── scenario.rs         # Predefined scenarios
│   ├── participant.rs      # Participant behavior models
│   ├── oracle.rs           # Oracle simulation
│   ├── market.rs           # Market state management
│   ├── simulator.rs        # Main simulation engine
│   ├── strategy.rs         # Trading strategies
│   └── analytics.rs        # Performance analytics
└── examples/               # Usage examples
```

## Use Cases

### For Developers

- **Pre-deployment Testing**: Validate market configurations before mainnet
- **Oracle Calibration**: Find optimal oracle parameters
- **Risk Assessment**: Identify edge cases and failure modes
- **Performance Tuning**: Optimize gas costs and execution

### For Traders

- **Strategy Development**: Backtest strategies with historical data
- **Risk Management**: Model position sizing and stop-losses
- **Market Analysis**: Understand belief dynamics patterns
- **Entry/Exit Optimization**: Find optimal timing windows

### For Researchers

- **Belief Dynamics**: Study how consensus forms and breaks
- **Market Microstructure**: Analyze participant behavior
- **Reflexivity**: Observe feedback loops
- **Comparative Analysis**: Test hypotheses across scenarios

## Performance

- **Simulation Speed**: ~1000 market updates/second
- **Memory Efficient**: Handles 10,000+ participants
- **Parallel Execution**: Async/await for concurrent simulations
- **Reproducible**: Seed-based deterministic results

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

## Links

- **GitHub**: <https://github.com/predaprotocol/preda-market-simulator>
- **Documentation**: <https://docs.rs/preda-market-simulator>
- **Preda Protocol**: <https://github.com/predaprotocol>

## Citation

If you use this simulator in your research, please cite:

```bibtex
@software{preda_market_simulator,
  title = {Preda Market Simulator},
  author = {Preda Protocol Team},
  year = {2024},
  url = {https://github.com/predaprotocol/preda-market-simulator}
}
```

---

**Built with ❤️ by the Preda Protocol team**

*"Predict not the event — predict when consensus will flip."*
