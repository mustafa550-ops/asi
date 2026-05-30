use thiserror::Error;

/// The central error type for ADLER Core.
/// Enforces the "Zero-Mock" policy by distinguishing between real failures and validation errors.
#[derive(Error, Debug)]
pub enum AdlerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Database error: {0}")]
    Db(String), // Will integrate with rusqlite error later

    #[error("LLM Provider error: {0}")]
    Llm(String),

    #[error("Hardware interaction error: {0}")]
    Hardware(String),

    #[error("WASM Sandbox error: {0}")]
    Sandbox(String),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Internal System error: {0}")]
    System(String),
}

/// A specialized `Result` type for ADLER Core operations.
pub type Result<T> = std::result::Result<T, AdlerError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_io_error() {
        let err = AdlerError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "file missing"));
        let msg = format!("{}", err);
        assert!(msg.contains("I/O error"));
        assert!(msg.contains("file missing"));
    }

    #[test]
    fn display_db_error() {
        let err = AdlerError::Db("connection failed".into());
        assert_eq!(format!("{}", err), "Database error: connection failed");
    }

    #[test]
    fn display_llm_error() {
        let err = AdlerError::Llm("timeout".into());
        assert_eq!(format!("{}", err), "LLM Provider error: timeout");
    }

    #[test]
    fn display_hardware_error() {
        let err = AdlerError::Hardware("GPIO pin 17 error".into());
        assert!(format!("{}", err).contains("GPIO"));
    }

    #[test]
    fn display_sandbox_error() {
        let err = AdlerError::Sandbox("memory limit exceeded".into());
        assert!(format!("{}", err).contains("memory limit"));
    }

    #[test]
    fn display_system_error() {
        let err = AdlerError::System("poisoned lock".into());
        assert!(format!("{}", err).contains("poisoned lock"));
    }

    #[test]
    fn from_serde_json_error() {
        let invalid = r#"{"invalid": }"#;
        let result: std::result::Result<serde_json::Value, AdlerError> =
            serde_json::from_str(invalid).map_err(AdlerError::from);
        assert!(result.is_err());
        assert!(format!("{}", result.unwrap_err()).contains("JSON"));
    }

    #[test]
    fn result_type_alias() {
        let ok: Result<i32> = Ok(42);
        assert_eq!(ok.unwrap(), 42);
        let err: Result<i32> = Err(AdlerError::System("fail".into()));
        assert!(err.is_err());
    }
}
