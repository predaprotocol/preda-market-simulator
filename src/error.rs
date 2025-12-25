//! Error types for the Preda Market Simulator

use thiserror::Error;

/// Result type alias for simulator operations
pub type Result<T> = std::result::Result<T, SimulatorError>;

/// Errors that can occur during simulation
#[derive(Error, Debug)]
pub enum SimulatorError {
    /// Configuration validation error
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Simulation execution error
    #[error("Simulation failed: {0}")]
    SimulationFailed(String),

    /// Oracle error
    #[error("Oracle error: {0}")]
    OracleError(String),

    /// Market state error
    #[error("Invalid market state: {0}")]
    InvalidMarketState(String),

    /// Strategy error
    #[error("Strategy error: {0}")]
    StrategyError(String),

    /// Data error
    #[error("Data error: {0}")]
    DataError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
