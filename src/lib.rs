//! # Preda Market Simulator
//!
//! A comprehensive testing suite and simulation framework for time-shifted prediction markets.
//!
//! ## Overview
//!
//! The Preda Market Simulator enables developers and researchers to:
//! - Simulate market outcomes before deployment
//! - Backtest trading strategies against historical BSI data
//! - Stress test oracle configurations
//! - Model belief dynamics scenarios
//! - Analyze market behavior under various conditions
//!
//! ## Features
//!
//! - **Market Simulation**: Run thousands of market scenarios with configurable parameters
//! - **Strategy Backtesting**: Test trading strategies against historical or synthetic data
//! - **Oracle Stress Testing**: Validate oracle configurations under extreme conditions
//! - **Belief Dynamics Modeling**: Simulate sentiment shifts, consensus changes, and narrative velocity
//! - **Statistical Analysis**: Comprehensive metrics and performance analysis
//! - **Scenario Generation**: Create realistic market conditions for testing
//!
//! ## Example
//!
//! ```rust
//! use preda_market_simulator::{Simulator, SimulationConfig, Scenario};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a simulation configuration
//! let config = SimulationConfig::builder()
//!     .duration_days(30)
//!     .num_participants(1000)
//!     .initial_bsi(0.5)
//!     .volatility(0.2)
//!     .build()?;
//!
//! // Run simulation
//! let simulator = Simulator::new(config);
//! let result = simulator.run(Scenario::BullishTrend).await?;
//!
//! // Analyze results
//! println!("Final BSI: {}", result.final_bsi);
//! println!("Total volume: {}", result.total_volume);
//! println!("Resolution time: {:?}", result.resolution_time);
//! # Ok(())
//! # }
//! ```

pub mod config;
pub mod error;
pub mod market;
pub mod oracle;
pub mod participant;
pub mod scenario;
pub mod simulator;
pub mod strategy;
pub mod types;
pub mod analytics;

pub use config::SimulationConfig;
pub use error::{SimulatorError, Result};
pub use market::{Market, MarketState};
pub use oracle::{OracleSimulator, OracleConfig};
pub use participant::{Participant, ParticipantBehavior};
pub use scenario::Scenario;
pub use simulator::{Simulator, SimulationResult};
pub use strategy::{Strategy, StrategyBacktest};
pub use types::{BSI, Position, Trade};
pub use analytics::{Analytics, PerformanceMetrics};
