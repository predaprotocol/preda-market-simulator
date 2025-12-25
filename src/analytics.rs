//! Analytics and performance metrics

use crate::simulator::SimulationResult;
use serde::{Deserialize, Serialize};

/// Analytics engine
pub struct Analytics;

impl Analytics {
    /// Analyze simulation results
    pub fn analyze(results: &[SimulationResult]) -> PerformanceMetrics {
        if results.is_empty() {
            return PerformanceMetrics::default();
        }

        let total_runs = results.len();
        let successful_resolutions = results
            .iter()
            .filter(|r| r.threshold_reached)
            .count();

        let avg_final_bsi = results.iter().map(|r| r.final_bsi).sum::<f64>() / total_runs as f64;

        let avg_volume = results.iter().map(|r| r.total_volume).sum::<f64>() / total_runs as f64;

        let avg_trades = results.iter().map(|r| r.total_trades).sum::<usize>() as f64
            / total_runs as f64;

        let avg_duration = results
            .iter()
            .map(|r| r.duration_days as f64)
            .sum::<f64>()
            / total_runs as f64;

        // Calculate BSI volatility
        let bsi_variance = results
            .iter()
            .map(|r| (r.final_bsi - avg_final_bsi).powi(2))
            .sum::<f64>()
            / total_runs as f64;
        let bsi_volatility = bsi_variance.sqrt();

        PerformanceMetrics {
            total_simulations: total_runs,
            successful_resolutions,
            resolution_rate: successful_resolutions as f64 / total_runs as f64,
            avg_final_bsi,
            avg_volume,
            avg_trades,
            avg_duration_days: avg_duration,
            bsi_volatility,
        }
    }

    /// Compare multiple scenarios
    pub fn compare_scenarios(
        results_by_scenario: &std::collections::HashMap<String, Vec<SimulationResult>>,
    ) -> Vec<ScenarioComparison> {
        results_by_scenario
            .iter()
            .map(|(scenario, results)| {
                let metrics = Self::analyze(results);
                ScenarioComparison {
                    scenario_name: scenario.clone(),
                    metrics,
                }
            })
            .collect()
    }
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total number of simulations
    pub total_simulations: usize,
    /// Number of successful resolutions
    pub successful_resolutions: usize,
    /// Resolution rate (0.0 to 1.0)
    pub resolution_rate: f64,
    /// Average final BSI
    pub avg_final_bsi: f64,
    /// Average trading volume
    pub avg_volume: f64,
    /// Average number of trades
    pub avg_trades: f64,
    /// Average duration in days
    pub avg_duration_days: f64,
    /// BSI volatility
    pub bsi_volatility: f64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        PerformanceMetrics {
            total_simulations: 0,
            successful_resolutions: 0,
            resolution_rate: 0.0,
            avg_final_bsi: 0.0,
            avg_volume: 0.0,
            avg_trades: 0.0,
            avg_duration_days: 0.0,
            bsi_volatility: 0.0,
        }
    }
}

/// Scenario comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioComparison {
    /// Scenario name
    pub scenario_name: String,
    /// Performance metrics
    pub metrics: PerformanceMetrics,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::Scenario;
    use crate::market::MarketStatistics;

    #[test]
    fn test_analytics() {
        let results = vec![
            SimulationResult {
                market_id: "test-1".to_string(),
                scenario: Scenario::BullishTrend,
                final_bsi: 0.8,
                total_volume: 10000.0,
                total_trades: 100,
                resolution_time: None,
                duration_days: 30,
                threshold_reached: true,
                statistics: MarketStatistics {
                    total_trades: 100,
                    total_volume: 10000.0,
                    active_positions: 50,
                    current_bsi: 0.8,
                    threshold: 0.75,
                    time_to_resolution: Some(2592000),
                },
            },
            SimulationResult {
                market_id: "test-2".to_string(),
                scenario: Scenario::BullishTrend,
                final_bsi: 0.7,
                total_volume: 8000.0,
                total_trades: 80,
                resolution_time: None,
                duration_days: 30,
                threshold_reached: false,
                statistics: MarketStatistics {
                    total_trades: 80,
                    total_volume: 8000.0,
                    active_positions: 40,
                    current_bsi: 0.7,
                    threshold: 0.75,
                    time_to_resolution: None,
                },
            },
        ];

        let metrics = Analytics::analyze(&results);

        assert_eq!(metrics.total_simulations, 2);
        assert_eq!(metrics.successful_resolutions, 1);
        assert_eq!(metrics.resolution_rate, 0.5);
    }
}
