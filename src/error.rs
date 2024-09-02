/// A type alias for the `Result` type used in this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error type for this crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[cfg(feature = "color-eyre")]
    #[error(transparent)]
    InstallError(#[from] color_eyre::eyre::InstallError),
    #[error("Calloop error: {0}")]
    Calloop(#[from] calloop::Error),
    #[error("Other error: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl Error {
    pub fn other(error: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Self {
        Self::Other(error.into())
    }
}
