//! Scenario comparison example

use preda_market_simulator::{Analytics, Scenario, SimulationConfig, Simulator};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Scenario Comparison Example ===\n");

    // Configure simulation
    let config = SimulationConfig::builder()
        .duration_days(30)
        .num_participants(1000)
        .initial_bsi(0.5)
        .volatility(0.2)
        .threshold(0.75)
        .build()?;

    let simulator = Simulator::new(config);

    // Test all scenarios
    let scenarios = Scenario::all();
    let mut results_by_scenario = HashMap::new();

    for scenario in scenarios {
        println!("Running scenario: {:?}", scenario);
        println!("Description: {}", scenario.description());

        let mut scenario_results = Vec::new();

        // Run 5 simulations per scenario
        for run in 1..=5 {
            print!("  Run {}/5...", run);
            let result = simulator.run(scenario).await?;
            scenario_results.push(result);
            println!(" Final BSI: {:.4}", scenario_results.last().unwrap().final_bsi);
        }

        results_by_scenario.insert(format!("{:?}", scenario), scenario_results);
        println!();
    }

    // Compare scenarios
    println!("\n=== Scenario Performance Comparison ===\n");
    let comparisons = Analytics::compare_scenarios(&results_by_scenario);

    for comparison in comparisons {
        println!("Scenario: {}", comparison.scenario_name);
        println!("  Total Simulations: {}", comparison.metrics.total_simulations);
        println!(
            "  Resolution Rate: {:.2}%",
            comparison.metrics.resolution_rate * 100.0
        );
        println!("  Avg Final BSI: {:.4}", comparison.metrics.avg_final_bsi);
        println!("  Avg Volume: ${:.2}", comparison.metrics.avg_volume);
        println!("  Avg Trades: {:.0}", comparison.metrics.avg_trades);
        println!(
            "  Avg Duration: {:.1} days",
            comparison.metrics.avg_duration_days
        );
        println!("  BSI Volatility: {:.4}", comparison.metrics.bsi_volatility);
        println!();
    }

    Ok(())
}
