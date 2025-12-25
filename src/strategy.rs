//! Trading strategy and backtesting

use crate::types::BSI;
use serde::{Deserialize, Serialize};

/// Trading strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Strategy {
    /// Buy when BSI is below threshold, sell when above
    ThresholdCrossing { threshold: f64 },
    /// Follow momentum
    Momentum { lookback_periods: usize },
    /// Mean reversion strategy
    MeanReversion { mean: f64, deviation: f64 },
    /// Contrarian strategy
    Contrarian { threshold: f64 },
    /// Custom strategy with user-defined logic
    Custom { name: String },
}

impl Strategy {
    /// Evaluate strategy signal (-1.0 to 1.0)
    /// Negative = short signal, Positive = long signal
    pub fn evaluate(&self, current_bsi: BSI, history: &[BSI]) -> f64 {
        match self {
            Strategy::ThresholdCrossing { threshold } => {
                if current_bsi.value() < *threshold {
                    1.0 // Long signal
                } else {
                    -1.0 // Short signal
                }
            }
            Strategy::Momentum { lookback_periods } => {
                if history.len() < *lookback_periods {
                    return 0.0;
                }

                let start_idx = history.len() - lookback_periods;
                let start_bsi = history[start_idx].value();
                let momentum = current_bsi.value() - start_bsi;

                momentum.clamp(-1.0, 1.0)
            }
            Strategy::MeanReversion { mean, deviation } => {
                let distance = current_bsi.value() - mean;
                if distance.abs() > *deviation {
                    -distance.signum() // Revert to mean
                } else {
                    0.0
                }
            }
            Strategy::Contrarian { threshold } => {
                if current_bsi.value() > *threshold {
                    -1.0 // Short when above threshold
                } else {
                    1.0 // Long when below threshold
                }
            }
            Strategy::Custom { .. } => 0.0,
        }
    }

    /// Get strategy name
    pub fn name(&self) -> String {
        match self {
            Strategy::ThresholdCrossing { .. } => "Threshold Crossing".to_string(),
            Strategy::Momentum { .. } => "Momentum".to_string(),
            Strategy::MeanReversion { .. } => "Mean Reversion".to_string(),
            Strategy::Contrarian { .. } => "Contrarian".to_string(),
            Strategy::Custom { name } => name.clone(),
        }
    }
}

/// Strategy backtest results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyBacktest {
    /// Strategy name
    pub strategy_name: String,
    /// Total return
    pub total_return: f64,
    /// Number of trades
    pub num_trades: usize,
    /// Win rate
    pub win_rate: f64,
    /// Sharpe ratio
    pub sharpe_ratio: f64,
    /// Maximum drawdown
    pub max_drawdown: f64,
}

impl StrategyBacktest {
    /// Create a new backtest result
    pub fn new(strategy_name: String) -> Self {
        StrategyBacktest {
            strategy_name,
            total_return: 0.0,
            num_trades: 0,
            win_rate: 0.0,
            sharpe_ratio: 0.0,
            max_drawdown: 0.0,
        }
    }

    /// Calculate performance metrics
    pub fn calculate_metrics(&mut self, returns: &[f64]) {
        if returns.is_empty() {
            return;
        }

        // Total return
        self.total_return = returns.iter().sum();

        // Win rate
        let wins = returns.iter().filter(|&&r| r > 0.0).count();
        self.win_rate = wins as f64 / returns.len() as f64;

        // Sharpe ratio (simplified)
        let mean_return = self.total_return / returns.len() as f64;
        let variance: f64 = returns
            .iter()
            .map(|r| (r - mean_return).powi(2))
            .sum::<f64>()
            / returns.len() as f64;
        let std_dev = variance.sqrt();

        self.sharpe_ratio = if std_dev > 0.0 {
            mean_return / std_dev
        } else {
            0.0
        };

        // Maximum drawdown
        let mut peak = 0.0;
        let mut max_dd = 0.0;
        let mut cumulative = 0.0;

        for &ret in returns {
            cumulative += ret;
            if cumulative > peak {
                peak = cumulative;
            }
            let drawdown = peak - cumulative;
            if drawdown > max_dd {
                max_dd = drawdown;
            }
        }

        self.max_drawdown = max_dd;
        self.num_trades = returns.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threshold_strategy() {
        let strategy = Strategy::ThresholdCrossing { threshold: 0.7 };
        let bsi = BSI::new(0.5).unwrap();
        let signal = strategy.evaluate(bsi, &[]);

        assert_eq!(signal, 1.0); // Should be long below threshold
    }

    #[test]
    fn test_backtest_metrics() {
        let mut backtest = StrategyBacktest::new("Test Strategy".to_string());
        let returns = vec![0.1, -0.05, 0.15, 0.2, -0.1];
        
        backtest.calculate_metrics(&returns);

        assert!(backtest.total_return > 0.0);
        assert!(backtest.win_rate > 0.0);
        assert_eq!(backtest.num_trades, 5);
    }
}
