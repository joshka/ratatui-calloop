mod app;
mod error;
mod event;
mod tui;

use std::fs::File;

use app::App;
use color_eyre::eyre::Context;
use event::ApplicationLoop;
use tracing_subscriber::EnvFilter;

fn main() -> color_eyre::Result<()> {
    error::install_hooks()?;
    init_logging()?;

    let mut app_loop = ApplicationLoop::new()?;
    let mut app = App::new(app_loop.loop_signal());
    app_loop.run(&mut app)?;

    Ok(())
}

fn init_logging() -> color_eyre::Result<()> {
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
