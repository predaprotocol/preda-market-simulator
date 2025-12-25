//! Strategy backtesting example

use preda_market_simulator::{Scenario, SimulationConfig, Simulator, Strategy, StrategyBacktest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Strategy Backtesting Example ===\n");

    // Configure simulation
    let config = SimulationConfig::builder()
        .duration_days(30)
        .num_participants(500)
        .initial_bsi(0.5)
        .volatility(0.15)
        .threshold(0.75)
        .seed(42) // For reproducibility
        .build()?;

    let simulator = Simulator::new(config);

    // Define strategies to test
    let strategies = vec![
        Strategy::ThresholdCrossing { threshold: 0.7 },
        Strategy::Momentum {
            lookback_periods: 10,
        },
        Strategy::MeanReversion {
            mean: 0.5,
            deviation: 0.2,
        },
        Strategy::Contrarian { threshold: 0.6 },
    ];

    // Run simulations for each strategy
    for strategy in strategies {
        println!("Testing strategy: {}", strategy.name());

        // Run multiple simulations
        let mut returns = Vec::new();
        for _ in 0..10 {
            let result = simulator.run(Scenario::Sideways).await?;
            // Simplified return calculation
            let ret = result.final_bsi - 0.5;
            returns.push(ret);
        }

        // Calculate backtest metrics
        let mut backtest = StrategyBacktest::new(strategy.name());
        backtest.calculate_metrics(&returns);

        println!("  Total Return: {:.4}", backtest.total_return);
        println!("  Win Rate: {:.2}%", backtest.win_rate * 100.0);
        println!("  Sharpe Ratio: {:.4}", backtest.sharpe_ratio);
        println!("  Max Drawdown: {:.4}", backtest.max_drawdown);
        println!("  Number of Trades: {}\n", backtest.num_trades);
    }

    Ok(())
}
