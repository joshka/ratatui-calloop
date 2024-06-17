mod app;
mod error;
mod event;
mod tui;

use app::App;
use event::ApplicationLoop;

fn main() -> color_eyre::Result<()> {
    error::install_hooks()?;

    let mut app_loop = ApplicationLoop::new()?;
    let mut app = App::new(app_loop.loop_signal());
    app_loop.run(&mut app)?;

    Ok(())
}
