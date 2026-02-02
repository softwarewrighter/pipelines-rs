//! Error types for pipeline operations.

use thiserror::Error;

/// Errors that can occur during pipeline processing.
#[derive(Debug, Error)]
pub enum PipelineError {
    /// Field position is out of bounds for the record.
    #[error("field position {start}:{length} exceeds record length {record_len}")]
    FieldOutOfBounds {
        start: usize,
        length: usize,
        record_len: usize,
    },

    /// Invalid record length (must be exactly 80 bytes).
    #[error("invalid record length: expected 80, got {0}")]
    InvalidRecordLength(usize),

    /// I/O error during file operations.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Custom stage error.
    #[error("stage error: {0}")]
    Stage(String),
}

/// Result type for pipeline operations.
pub type Result<T> = std::result::Result<T, PipelineError>;
