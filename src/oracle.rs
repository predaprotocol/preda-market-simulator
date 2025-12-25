//! Oracle simulation for belief signal generation

use crate::error::{Result, SimulatorError};
use crate::types::BSI;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

/// Oracle simulator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleConfig {
    /// Base update frequency in seconds
    pub update_frequency: u32,
    /// Noise level (0.0 to 1.0)
    pub noise_level: f64,
    /// Drift rate per update
    pub drift_rate: f64,
    /// Mean reversion strength
    pub mean_reversion: f64,
}

impl Default for OracleConfig {
    fn default() -> Self {
        OracleConfig {
            update_frequency: 300, // 5 minutes
            noise_level: 0.05,
            drift_rate: 0.01,
            mean_reversion: 0.1,
        }
    }
}

/// Oracle simulator for generating belief signals
#[derive(Debug)]
pub struct OracleSimulator {
    config: OracleConfig,
    current_bsi: BSI,
    target_bsi: Option<f64>,
}

impl OracleSimulator {
    /// Create a new oracle simulator
    pub fn new(config: OracleConfig, initial_bsi: BSI) -> Self {
        OracleSimulator {
            config,
            current_bsi: initial_bsi,
            target_bsi: None,
        }
    }

    /// Set target BSI for scenario-driven simulation
    pub fn set_target(&mut self, target: f64) {
        self.target_bsi = Some(target);
    }

    /// Generate next BSI value
    pub fn next_bsi(&mut self) -> Result<BSI> {
        let mut rng = rand::thread_rng();
        
        // Base value
        let mut next_value = self.current_bsi.value();

        // Add drift
        if let Some(target) = self.target_bsi {
            // Drift toward target
            let diff = target - next_value;
            next_value += diff * self.config.drift_rate;
        } else {
            // Random walk
            let drift = rng.gen_range(-self.config.drift_rate..self.config.drift_rate);
            next_value += drift;
        }

        // Add noise
        let normal = Normal::new(0.0, self.config.noise_level)
            .map_err(|e| SimulatorError::OracleError(e.to_string()))?;
        let noise = normal.sample(&mut rng);
        next_value += noise;

        // Mean reversion
        let mean = 0.5;
        let reversion = (mean - next_value) * self.config.mean_reversion;
        next_value += reversion;

        // Clamp to valid range
        next_value = next_value.clamp(0.0, 1.0);

        self.current_bsi = BSI::new(next_value)
            .map_err(|e| SimulatorError::OracleError(e))?;

        Ok(self.current_bsi)
    }

    /// Get current BSI
    pub fn current_bsi(&self) -> BSI {
        self.current_bsi
    }

    /// Simulate sentiment shock (sudden large change)
    pub fn apply_shock(&mut self, magnitude: f64) -> Result<BSI> {
        let mut new_value = self.current_bsi.value() + magnitude;
        new_value = new_value.clamp(0.0, 1.0);
        
        self.current_bsi = BSI::new(new_value)
            .map_err(|e| SimulatorError::OracleError(e))?;

        Ok(self.current_bsi)
    }

    /// Reset oracle to initial state
    pub fn reset(&mut self, initial_bsi: BSI) {
        self.current_bsi = initial_bsi;
        self.target_bsi = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle_creation() {
        let config = OracleConfig::default();
        let initial_bsi = BSI::new(0.5).unwrap();
        let oracle = OracleSimulator::new(config, initial_bsi);

        assert_eq!(oracle.current_bsi().value(), 0.5);
    }

    #[test]
    fn test_oracle_next_bsi() {
        let config = OracleConfig::default();
        let initial_bsi = BSI::new(0.5).unwrap();
        let mut oracle = OracleSimulator::new(config, initial_bsi);

        let next = oracle.next_bsi().unwrap();
        assert!(next.value() >= 0.0 && next.value() <= 1.0);
    }

    #[test]
    fn test_oracle_shock() {
        let config = OracleConfig::default();
        let initial_bsi = BSI::new(0.5).unwrap();
        let mut oracle = OracleSimulator::new(config, initial_bsi);

        let shocked = oracle.apply_shock(0.3).unwrap();
        assert!(shocked.value() > 0.5);
    }

    #[test]
    fn test_oracle_target_drift() {
        let config = OracleConfig {
            drift_rate: 0.1,
            ..Default::default()
        };
        let initial_bsi = BSI::new(0.3).unwrap();
        let mut oracle = OracleSimulator::new(config, initial_bsi);
        
        oracle.set_target(0.7);
        
        // After several updates, should drift toward target
        for _ in 0..10 {
            oracle.next_bsi().unwrap();
        }
        
        assert!(oracle.current_bsi().value() > 0.3);
    }
}
