//! Predefined simulation scenarios

use serde::{Deserialize, Serialize};

/// Predefined market scenarios for simulation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scenario {
    /// Steady bullish trend with low volatility
    BullishTrend,
    /// Steady bearish trend with low volatility
    BearishTrend,
    /// High volatility sideways movement
    Sideways,
    /// Rapid sentiment reversal
    SentimentReversal,
    /// Gradual consensus formation
    ConsensusFormation,
    /// Extreme volatility with rapid swings
    HighVolatility,
    /// Slow drift with minimal activity
    LowActivity,
    /// Flash crash scenario
    FlashCrash,
    /// Parabolic rise
    ParabolicRise,
    /// Custom scenario (user-defined)
    Custom,
}

impl Scenario {
    /// Get scenario description
    pub fn description(&self) -> &str {
        match self {
            Scenario::BullishTrend => "Steady upward trend with increasing BSI",
            Scenario::BearishTrend => "Steady downward trend with decreasing BSI",
            Scenario::Sideways => "High volatility with no clear direction",
            Scenario::SentimentReversal => "Rapid shift from bearish to bullish or vice versa",
            Scenario::ConsensusFormation => "Gradual convergence toward threshold",
            Scenario::HighVolatility => "Extreme price swings and rapid BSI changes",
            Scenario::LowActivity => "Minimal trading with slow BSI drift",
            Scenario::FlashCrash => "Sudden sharp drop followed by recovery",
            Scenario::ParabolicRise => "Accelerating upward movement",
            Scenario::Custom => "User-defined scenario parameters",
        }
    }

    /// Get recommended volatility for scenario
    pub fn recommended_volatility(&self) -> f64 {
        match self {
            Scenario::BullishTrend | Scenario::BearishTrend => 0.1,
            Scenario::Sideways => 0.25,
            Scenario::SentimentReversal => 0.3,
            Scenario::ConsensusFormation => 0.15,
            Scenario::HighVolatility => 0.5,
            Scenario::LowActivity => 0.05,
            Scenario::FlashCrash => 0.4,
            Scenario::ParabolicRise => 0.2,
            Scenario::Custom => 0.2,
        }
    }

    /// Get recommended number of participants
    pub fn recommended_participants(&self) -> usize {
        match self {
            Scenario::BullishTrend | Scenario::BearishTrend => 500,
            Scenario::Sideways => 1000,
            Scenario::SentimentReversal => 750,
            Scenario::ConsensusFormation => 300,
            Scenario::HighVolatility => 1500,
            Scenario::LowActivity => 100,
            Scenario::FlashCrash => 800,
            Scenario::ParabolicRise => 600,
            Scenario::Custom => 500,
        }
    }

    /// Get all available scenarios
    pub fn all() -> Vec<Scenario> {
        vec![
            Scenario::BullishTrend,
            Scenario::BearishTrend,
            Scenario::Sideways,
            Scenario::SentimentReversal,
            Scenario::ConsensusFormation,
            Scenario::HighVolatility,
            Scenario::LowActivity,
            Scenario::FlashCrash,
            Scenario::ParabolicRise,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_descriptions() {
        for scenario in Scenario::all() {
            assert!(!scenario.description().is_empty());
        }
    }

    #[test]
    fn test_scenario_parameters() {
        let scenario = Scenario::BullishTrend;
        assert!(scenario.recommended_volatility() > 0.0);
        assert!(scenario.recommended_participants() > 0);
    }
}
