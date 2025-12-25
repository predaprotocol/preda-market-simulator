//! Market participant simulation

use crate::types::{BSI, Position, PositionType};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Market participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// Unique participant ID
    pub id: String,
    /// Participant behavior type
    pub behavior: ParticipantBehavior,
    /// Current positions
    pub positions: Vec<Position>,
    /// Total capital
    pub capital: f64,
    /// Risk tolerance (0.0 to 1.0)
    pub risk_tolerance: f64,
}

impl Participant {
    /// Create a new participant
    pub fn new(id: String, behavior: ParticipantBehavior, capital: f64) -> Self {
        let mut rng = rand::thread_rng();
        Participant {
            id,
            behavior,
            positions: Vec::new(),
            capital,
            risk_tolerance: rng.gen_range(0.1..0.9),
        }
    }

    /// Decide whether to trade based on current BSI
    pub fn should_trade(&self, current_bsi: BSI, threshold: f64) -> bool {
        let mut rng = rand::thread_rng();
        
        match self.behavior {
            ParticipantBehavior::Rational => {
                // Trade based on distance from threshold
                let distance = current_bsi.distance_from(threshold);
                distance > 0.1 && rng.gen_bool(0.3)
            }
            ParticipantBehavior::Momentum => {
                // Always trade in direction of momentum
                rng.gen_bool(0.5)
            }
            ParticipantBehavior::Contrarian => {
                // Trade against the trend
                rng.gen_bool(0.4)
            }
            ParticipantBehavior::Random => {
                // Random trading
                rng.gen_bool(0.2)
            }
            ParticipantBehavior::Conservative => {
                // Trade rarely, only on strong signals
                let distance = current_bsi.distance_from(threshold);
                distance > 0.2 && rng.gen_bool(0.15)
            }
            ParticipantBehavior::Aggressive => {
                // Trade frequently
                rng.gen_bool(0.7)
            }
        }
    }

    /// Determine position type based on behavior and market state
    pub fn determine_position_type(&self, current_bsi: BSI, threshold: f64) -> PositionType {
        match self.behavior {
            ParticipantBehavior::Rational => {
                if current_bsi.value() < threshold {
                    PositionType::Long
                } else {
                    PositionType::Short
                }
            }
            ParticipantBehavior::Momentum => {
                if current_bsi.value() > 0.5 {
                    PositionType::Long
                } else {
                    PositionType::Short
                }
            }
            ParticipantBehavior::Contrarian => {
                if current_bsi.value() > 0.5 {
                    PositionType::Short
                } else {
                    PositionType::Long
                }
            }
            ParticipantBehavior::Random => {
                if rand::thread_rng().gen_bool(0.5) {
                    PositionType::Long
                } else {
                    PositionType::Short
                }
            }
            ParticipantBehavior::Conservative | ParticipantBehavior::Aggressive => {
                if current_bsi.value() < threshold {
                    PositionType::Long
                } else {
                    PositionType::Short
                }
            }
        }
    }

    /// Calculate position size based on capital and risk tolerance
    pub fn calculate_position_size(&self) -> f64 {
        self.capital * self.risk_tolerance * 0.1
    }
}

/// Participant behavior types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParticipantBehavior {
    /// Makes rational decisions based on BSI distance from threshold
    Rational,
    /// Follows momentum and trends
    Momentum,
    /// Trades against the trend
    Contrarian,
    /// Random trading decisions
    Random,
    /// Conservative, low-frequency trading
    Conservative,
    /// Aggressive, high-frequency trading
    Aggressive,
}

impl ParticipantBehavior {
    /// Get all behavior types
    pub fn all() -> Vec<ParticipantBehavior> {
        vec![
            ParticipantBehavior::Rational,
            ParticipantBehavior::Momentum,
            ParticipantBehavior::Contrarian,
            ParticipantBehavior::Random,
            ParticipantBehavior::Conservative,
            ParticipantBehavior::Aggressive,
        ]
    }

    /// Get random behavior type
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let behaviors = Self::all();
        behaviors[rng.gen_range(0..behaviors.len())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_participant_creation() {
        let participant = Participant::new(
            "test-1".to_string(),
            ParticipantBehavior::Rational,
            1000.0,
        );

        assert_eq!(participant.id, "test-1");
        assert_eq!(participant.capital, 1000.0);
        assert!(participant.risk_tolerance > 0.0);
    }

    #[test]
    fn test_position_size_calculation() {
        let participant = Participant::new(
            "test-1".to_string(),
            ParticipantBehavior::Rational,
            1000.0,
        );

        let size = participant.calculate_position_size();
        assert!(size > 0.0);
        assert!(size <= participant.capital);
    }
}
