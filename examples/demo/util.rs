use std::{fs::File, panic};

use color_eyre::{
    config::{EyreHook, HookBuilder, PanicHook},
    eyre::{self, WrapErr},
};
use ratatui_calloop::restore_terminal;
use tracing_subscriber::EnvFilter;

/// Installs hooks for panic and error handling.
///
/// Makes the app resilient to panics and errors by restoring the terminal before printing the
/// panic or error message. This prevents error messages from being messed up by the terminal
/// state.
pub fn int_error_handling() -> color_eyre::Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();
    install_panic_hook(panic_hook);
    install_error_hook(eyre_hook)?;
    Ok(())
}

/// Install a panic hook that restores the terminal before printing the panic.
fn install_panic_hook(panic_hook: PanicHook) {
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        panic_hook(panic_info);
    }));
}

/// Install an error hook that restores the terminal before printing the error.
fn install_error_hook(eyre_hook: EyreHook) -> color_eyre::Result<()> {
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error| {
        let _ = restore_terminal();
        eyre_hook(error)
    }))?;
    Ok(())
}

pub fn init_logging() -> color_eyre::Result<()> {
    let file = File::create("trace.log").wrap_err("unable to create log file trace.log")?;
    let env_filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::TRACE.into())
        .from_env()
        .wrap_err("unable to parse RUST_LOG environment variable")?;
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(file)
        .init();
    Ok(())
}
