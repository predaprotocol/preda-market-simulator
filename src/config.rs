//! Simulation configuration

use crate::error::{Result, SimulatorError};
use serde::{Deserialize, Serialize};

/// Configuration for market simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// Duration of simulation in days
    pub duration_days: u32,
    /// Number of market participants
    pub num_participants: usize,
    /// Initial BSI value
    pub initial_bsi: f64,
    /// Market volatility (0.0 to 1.0)
    pub volatility: f64,
    /// BSI threshold for market resolution
    pub threshold: f64,
    /// Minimum persistence window in hours
    pub persistence_hours: u32,
    /// Update frequency in seconds
    pub update_frequency_secs: u32,
    /// Random seed for reproducibility
    pub seed: Option<u64>,
}

impl SimulationConfig {
    /// Create a new configuration builder
    pub fn builder() -> SimulationConfigBuilder {
        SimulationConfigBuilder::default()
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.duration_days == 0 {
            return Err(SimulatorError::InvalidConfig(
                "Duration must be greater than 0".to_string(),
            ));
        }

        if self.num_participants == 0 {
            return Err(SimulatorError::InvalidConfig(
                "Must have at least 1 participant".to_string(),
            ));
        }

        if !(0.0..=1.0).contains(&self.initial_bsi) {
            return Err(SimulatorError::InvalidConfig(
                "Initial BSI must be between 0.0 and 1.0".to_string(),
            ));
        }

        if !(0.0..=1.0).contains(&self.volatility) {
            return Err(SimulatorError::InvalidConfig(
                "Volatility must be between 0.0 and 1.0".to_string(),
            ));
        }

        if !(0.0..=1.0).contains(&self.threshold) {
            return Err(SimulatorError::InvalidConfig(
                "Threshold must be between 0.0 and 1.0".to_string(),
            ));
        }

        if self.update_frequency_secs == 0 {
            return Err(SimulatorError::InvalidConfig(
                "Update frequency must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }
}

/// Builder for SimulationConfig
#[derive(Debug, Default)]
pub struct SimulationConfigBuilder {
    duration_days: Option<u32>,
    num_participants: Option<usize>,
    initial_bsi: Option<f64>,
    volatility: Option<f64>,
    threshold: Option<f64>,
    persistence_hours: Option<u32>,
    update_frequency_secs: Option<u32>,
    seed: Option<u64>,
}

impl SimulationConfigBuilder {
    /// Set simulation duration in days
    pub fn duration_days(mut self, days: u32) -> Self {
        self.duration_days = Some(days);
        self
    }

    /// Set number of participants
    pub fn num_participants(mut self, num: usize) -> Self {
        self.num_participants = Some(num);
        self
    }

    /// Set initial BSI value
    pub fn initial_bsi(mut self, bsi: f64) -> Self {
        self.initial_bsi = Some(bsi);
        self
    }

    /// Set market volatility
    pub fn volatility(mut self, vol: f64) -> Self {
        self.volatility = Some(vol);
        self
    }

    /// Set BSI threshold
    pub fn threshold(mut self, threshold: f64) -> Self {
        self.threshold = Some(threshold);
        self
    }

    /// Set persistence window in hours
    pub fn persistence_hours(mut self, hours: u32) -> Self {
        self.persistence_hours = Some(hours);
        self
    }

    /// Set update frequency in seconds
    pub fn update_frequency_secs(mut self, secs: u32) -> Self {
        self.update_frequency_secs = Some(secs);
        self
    }

    /// Set random seed for reproducibility
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Build the configuration
    pub fn build(self) -> Result<SimulationConfig> {
        let config = SimulationConfig {
            duration_days: self.duration_days.unwrap_or(30),
            num_participants: self.num_participants.unwrap_or(100),
            initial_bsi: self.initial_bsi.unwrap_or(0.5),
            volatility: self.volatility.unwrap_or(0.1),
            threshold: self.threshold.unwrap_or(0.75),
            persistence_hours: self.persistence_hours.unwrap_or(24),
            update_frequency_secs: self.update_frequency_secs.unwrap_or(300),
            seed: self.seed,
        };

        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = SimulationConfig::builder()
            .duration_days(30)
            .num_participants(1000)
            .initial_bsi(0.5)
            .volatility(0.2)
            .build()
            .unwrap();

        assert_eq!(config.duration_days, 30);
        assert_eq!(config.num_participants, 1000);
    }

    #[test]
    fn test_config_validation() {
        let result = SimulationConfig::builder()
            .duration_days(0)
            .build();

        assert!(result.is_err());
    }
}
