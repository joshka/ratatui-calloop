#[cfg(feature = "color-eyre")]
use color_eyre::config::{EyreHook, HookBuilder, PanicHook};

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

#[cfg(feature = "color-eyre")]
/// Installs hooks for panic and error handling.
///
/// Makes the app resilient to panics and errors by restoring the terminal before printing the
/// panic or error message. This prevents error messages from being messed up by the terminal
/// state.
pub fn install_hooks() -> Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();
    install_panic_hook(panic_hook);
    install_error_hook(eyre_hook)?;
    Ok(())
}

#[cfg(feature = "color-eyre")]
/// Install a panic hook that restores the terminal before printing the panic.
fn install_panic_hook(panic_hook: PanicHook) {
    let panic_hook = panic_hook.into_panic_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = crate::terminal::restore();
        panic_hook(panic_info);
    }));
}

#[cfg(feature = "color-eyre")]
/// Install an error hook that restores the terminal before printing the error.
fn install_error_hook(eyre_hook: EyreHook) -> Result<()> {
    let eyre_hook = eyre_hook.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |error| {
        let _ = crate::terminal::restore();
        eyre_hook(error)
    }))?;
    Ok(())
}
