//! Main simulator implementation

use crate::config::SimulationConfig;
use crate::error::{Result, SimulatorError};
use crate::market::{Market, MarketState};
use crate::oracle::{OracleConfig, OracleSimulator};
use crate::participant::{Participant, ParticipantBehavior};
use crate::scenario::Scenario;
use crate::types::{BSI, TimeInterval, Trade, TradeType};
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Main simulator
pub struct Simulator {
    config: SimulationConfig,
}

impl Simulator {
    /// Create a new simulator
    pub fn new(config: SimulationConfig) -> Self {
        Simulator { config }
    }

    /// Run simulation with given scenario
    pub async fn run(&self, scenario: Scenario) -> Result<SimulationResult> {
        // Initialize market
        let start_time = Utc::now();
        let end_time = start_time + Duration::days(self.config.duration_days as i64);
        let interval = TimeInterval::new(start_time, end_time);

        let initial_bsi = BSI::new(self.config.initial_bsi)
            .map_err(|e| SimulatorError::InvalidConfig(e))?;
        let mut market = Market::new(
            format!("sim-{}", start_time.timestamp()),
            initial_bsi,
            self.config.threshold,
            interval,
        );

        // Initialize oracle
        let oracle_config = OracleConfig {
            update_frequency: self.config.update_frequency_secs,
            noise_level: self.config.volatility * 0.5,
            drift_rate: self.config.volatility * 0.1,
            mean_reversion: 0.1,
        };

        let mut oracle = OracleSimulator::new(oracle_config, initial_bsi);

        // Set oracle target based on scenario
        match scenario {
            Scenario::BullishTrend => oracle.set_target(0.8),
            Scenario::BearishTrend => oracle.set_target(0.2),
            Scenario::SentimentReversal => oracle.set_target(0.9),
            Scenario::ConsensusFormation => oracle.set_target(self.config.threshold),
            Scenario::ParabolicRise => oracle.set_target(0.95),
            _ => {}
        }

        // Initialize participants
        let mut participants = self.create_participants();

        // Simulation loop
        let mut current_time = start_time;
        let update_interval = Duration::seconds(self.config.update_frequency_secs as i64);
        let mut trade_counter = 0;

        while current_time < end_time && market.state == MarketState::Active {
            // Update BSI
            let new_bsi = oracle.next_bsi()?;
            market.update_bsi(new_bsi);

            // Apply scenario-specific events
            if let Some(shock) = self.should_apply_shock(&scenario, current_time, start_time) {
                oracle.apply_shock(shock)?;
            }

            // Simulate participant trading
            for participant in &mut participants {
                if participant.should_trade(new_bsi, self.config.threshold) {
                    let trade = self.create_trade(
                        participant,
                        new_bsi,
                        current_time,
                        &mut trade_counter,
                    );
                    market.add_trade(trade);
                }
            }

            // Check for resolution
            if market.should_resolve(current_time) {
                market.resolve(current_time);
                break;
            }

            current_time = current_time + update_interval;
        }

        // Generate result
        let result = SimulationResult {
            market_id: market.id.clone(),
            scenario,
            final_bsi: market.current_bsi.value(),
            total_volume: market.total_volume,
            total_trades: market.trades.len(),
            resolution_time: market.resolution_time,
            duration_days: (current_time - start_time).num_days() as u32,
            threshold_reached: market.state == MarketState::Resolved,
            statistics: market.statistics(),
        };

        Ok(result)
    }

    /// Create participants for simulation
    fn create_participants(&self) -> Vec<Participant> {
        let mut participants = Vec::new();
        let behaviors = ParticipantBehavior::all();

        for i in 0..self.config.num_participants {
            let behavior = behaviors[i % behaviors.len()];
            let capital = 1000.0; // Default capital
            let participant = Participant::new(
                format!("participant-{}", i),
                behavior,
                capital,
            );
            participants.push(participant);
        }

        participants
    }

    /// Create a trade for a participant
    fn create_trade(
        &self,
        participant: &Participant,
        current_bsi: BSI,
        timestamp: DateTime<Utc>,
        counter: &mut usize,
    ) -> Trade {
        *counter += 1;
        let _position_type = participant.determine_position_type(current_bsi, self.config.threshold);
        let size = participant.calculate_position_size();

        Trade {
            id: format!("trade-{}", counter),
            participant_id: participant.id.clone(),
            trade_type: TradeType::Open,
            size,
            price: current_bsi.value(),
            timestamp,
            bsi_at_trade: current_bsi,
        }
    }

    /// Determine if shock should be applied based on scenario
    fn should_apply_shock(
        &self,
        scenario: &Scenario,
        current_time: DateTime<Utc>,
        start_time: DateTime<Utc>,
    ) -> Option<f64> {
        let elapsed_days = (current_time - start_time).num_days();
        let mut rng = rand::thread_rng();

        match scenario {
            Scenario::FlashCrash if elapsed_days == 7 => Some(-0.3),
            Scenario::SentimentReversal if elapsed_days == 10 => Some(0.4),
            Scenario::HighVolatility if rng.gen_bool(0.1) => {
                Some(rng.gen_range(-0.2..0.2))
            }
            _ => None,
        }
    }
}

/// Result of a simulation run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    /// Market ID
    pub market_id: String,
    /// Scenario used
    pub scenario: Scenario,
    /// Final BSI value
    pub final_bsi: f64,
    /// Total trading volume
    pub total_volume: f64,
    /// Total number of trades
    pub total_trades: usize,
    /// Resolution time (if resolved)
    pub resolution_time: Option<DateTime<Utc>>,
    /// Actual duration in days
    pub duration_days: u32,
    /// Whether threshold was reached
    pub threshold_reached: bool,
    /// Market statistics
    pub statistics: crate::market::MarketStatistics,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simulator_run() {
        let config = SimulationConfig::builder()
            .duration_days(30)
            .num_participants(100)
            .initial_bsi(0.5)
            .volatility(0.2)
            .threshold(0.75)
            .build()
            .unwrap();

        let simulator = Simulator::new(config);
        let result = simulator.run(Scenario::BullishTrend).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.final_bsi >= 0.0 && result.final_bsi <= 1.0);
    }
}
