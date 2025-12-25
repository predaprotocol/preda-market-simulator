//! Core types for the market simulator

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Belief State Index value (0.0 to 1.0)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BSI(f64);

impl BSI {
    /// Create a new BSI value
    ///
    /// # Errors
    ///
    /// Returns error if value is not in range [0.0, 1.0]
    pub fn new(value: f64) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&value) {
            return Err(format!("BSI value must be between 0.0 and 1.0, got {}", value));
        }
        Ok(BSI(value))
    }

    /// Get the raw BSI value
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Calculate distance from threshold
    pub fn distance_from(&self, threshold: f64) -> f64 {
        (self.0 - threshold).abs()
    }

    /// Check if BSI crossed threshold
    pub fn crossed_threshold(&self, previous: BSI, threshold: f64) -> bool {
        (previous.0 < threshold && self.0 >= threshold)
            || (previous.0 > threshold && self.0 <= threshold)
    }
}

impl Default for BSI {
    fn default() -> Self {
        BSI(0.5)
    }
}

/// Trading position in a market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Participant ID
    pub participant_id: String,
    /// Position size
    pub size: f64,
    /// Entry price
    pub entry_price: f64,
    /// Entry time
    pub entry_time: DateTime<Utc>,
    /// Position type (long/short)
    pub position_type: PositionType,
}

/// Type of position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionType {
    /// Long position (betting BSI will increase)
    Long,
    /// Short position (betting BSI will decrease)
    Short,
}

/// A trade execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Trade ID
    pub id: String,
    /// Participant ID
    pub participant_id: String,
    /// Trade type
    pub trade_type: TradeType,
    /// Trade size
    pub size: f64,
    /// Execution price
    pub price: f64,
    /// Execution time
    pub timestamp: DateTime<Utc>,
    /// Current BSI at trade time
    pub bsi_at_trade: BSI,
}

/// Type of trade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeType {
    /// Open a new position
    Open,
    /// Close an existing position
    Close,
    /// Increase position size
    Increase,
    /// Decrease position size
    Decrease,
}

/// Time interval for simulation
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeInterval {
    /// Start time
    pub start: DateTime<Utc>,
    /// End time
    pub end: DateTime<Utc>,
}

impl TimeInterval {
    /// Create a new time interval
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        TimeInterval { start, end }
    }

    /// Get duration in seconds
    pub fn duration_secs(&self) -> i64 {
        (self.end - self.start).num_seconds()
    }

    /// Get duration in days
    pub fn duration_days(&self) -> i64 {
        (self.end - self.start).num_days()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsi_creation() {
        assert!(BSI::new(0.5).is_ok());
        assert!(BSI::new(0.0).is_ok());
        assert!(BSI::new(1.0).is_ok());
        assert!(BSI::new(-0.1).is_err());
        assert!(BSI::new(1.1).is_err());
    }

    #[test]
    fn test_bsi_threshold_crossing() {
        let prev = BSI::new(0.4).unwrap();
        let curr = BSI::new(0.6).unwrap();
        assert!(curr.crossed_threshold(prev, 0.5));
    }

    #[test]
    fn test_bsi_distance() {
        let bsi = BSI::new(0.7).unwrap();
        let distance = bsi.distance_from(0.5);
        assert!((distance - 0.2).abs() < 1e-10);
    }
}
