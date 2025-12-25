//! Market state and lifecycle management

use crate::types::{BSI, Position, Trade, TimeInterval};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Market state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    /// Market ID
    pub id: String,
    /// Market state
    pub state: MarketState,
    /// Current BSI
    pub current_bsi: BSI,
    /// BSI threshold for resolution
    pub threshold: f64,
    /// Time interval
    pub time_interval: TimeInterval,
    /// All trades
    pub trades: Vec<Trade>,
    /// Active positions
    pub positions: Vec<Position>,
    /// Total volume
    pub total_volume: f64,
    /// Resolution time (if resolved)
    pub resolution_time: Option<DateTime<Utc>>,
}

impl Market {
    /// Create a new market
    pub fn new(
        id: String,
        initial_bsi: BSI,
        threshold: f64,
        time_interval: TimeInterval,
    ) -> Self {
        Market {
            id,
            state: MarketState::Active,
            current_bsi: initial_bsi,
            threshold,
            time_interval,
            trades: Vec::new(),
            positions: Vec::new(),
            total_volume: 0.0,
            resolution_time: None,
        }
    }

    /// Update market BSI
    pub fn update_bsi(&mut self, new_bsi: BSI) {
        self.current_bsi = new_bsi;
    }

    /// Add a trade to the market
    pub fn add_trade(&mut self, trade: Trade) {
        self.total_volume += trade.size;
        self.trades.push(trade);
    }

    /// Add a position
    pub fn add_position(&mut self, position: Position) {
        self.positions.push(position);
    }

    /// Check if market should resolve
    pub fn should_resolve(&self, current_time: DateTime<Utc>) -> bool {
        // Check if BSI crossed threshold
        let threshold_crossed = self.current_bsi.value() >= self.threshold;
        
        // Check if within time interval
        let within_interval = current_time >= self.time_interval.start
            && current_time <= self.time_interval.end;

        threshold_crossed && within_interval
    }

    /// Resolve the market
    pub fn resolve(&mut self, resolution_time: DateTime<Utc>) {
        self.state = MarketState::Resolved;
        self.resolution_time = Some(resolution_time);
    }

    /// Get market statistics
    pub fn statistics(&self) -> MarketStatistics {
        MarketStatistics {
            total_trades: self.trades.len(),
            total_volume: self.total_volume,
            active_positions: self.positions.len(),
            current_bsi: self.current_bsi.value(),
            threshold: self.threshold,
            time_to_resolution: self.resolution_time.map(|rt| {
                (rt - self.time_interval.start).num_seconds()
            }),
        }
    }
}

/// Market state enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketState {
    /// Market is active and accepting trades
    Active,
    /// Market has resolved
    Resolved,
    /// Market has expired without resolution
    Expired,
    /// Market is paused
    Paused,
}

/// Market statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStatistics {
    /// Total number of trades
    pub total_trades: usize,
    /// Total trading volume
    pub total_volume: f64,
    /// Number of active positions
    pub active_positions: usize,
    /// Current BSI value
    pub current_bsi: f64,
    /// Resolution threshold
    pub threshold: f64,
    /// Time to resolution in seconds (if resolved)
    pub time_to_resolution: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_market_creation() {
        let initial_bsi = BSI::new(0.5).unwrap();
        let start = Utc::now();
        let end = start + Duration::days(30);
        let interval = TimeInterval::new(start, end);

        let market = Market::new(
            "test-market".to_string(),
            initial_bsi,
            0.75,
            interval,
        );

        assert_eq!(market.state, MarketState::Active);
        assert_eq!(market.threshold, 0.75);
    }

    #[test]
    fn test_market_resolution() {
        let initial_bsi = BSI::new(0.5).unwrap();
        let start = Utc::now();
        let end = start + Duration::days(30);
        let interval = TimeInterval::new(start, end);

        let mut market = Market::new(
            "test-market".to_string(),
            initial_bsi,
            0.75,
            interval,
        );

        // Update BSI to cross threshold
        market.update_bsi(BSI::new(0.8).unwrap());

        assert!(market.should_resolve(Utc::now()));

        market.resolve(Utc::now());
        assert_eq!(market.state, MarketState::Resolved);
    }
}
