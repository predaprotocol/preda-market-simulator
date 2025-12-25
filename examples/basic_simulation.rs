//! Basic simulation example

use preda_market_simulator::{Scenario, SimulationConfig, Simulator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure simulation
    let config = SimulationConfig::builder()
        .duration_days(30)
        .num_participants(1000)
        .initial_bsi(0.5)
        .volatility(0.2)
        .threshold(0.75)
        .persistence_hours(24)
        .update_frequency_secs(300) // 5 minutes
        .build()?;

    // Create simulator
    let simulator = Simulator::new(config);

    // Run simulation with bullish trend scenario
    println!("Running simulation with Bullish Trend scenario...");
    let result = simulator.run(Scenario::BullishTrend).await?;

    // Print results
    println!("\n=== Simulation Results ===");
    println!("Market ID: {}", result.market_id);
    println!("Scenario: {:?}", result.scenario);
    println!("Final BSI: {:.4}", result.final_bsi);
    println!("Total Volume: ${:.2}", result.total_volume);
    println!("Total Trades: {}", result.total_trades);
    println!("Duration: {} days", result.duration_days);
    println!("Threshold Reached: {}", result.threshold_reached);

    if let Some(resolution_time) = result.resolution_time {
        println!("Resolution Time: {}", resolution_time);
    }

    println!("\n=== Market Statistics ===");
    println!("Active Positions: {}", result.statistics.active_positions);
    if let Some(time_to_res) = result.statistics.time_to_resolution {
        println!("Time to Resolution: {} seconds", time_to_res);
    }

    Ok(())
}
