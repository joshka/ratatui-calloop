use std::io::{self, stdout, Stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{
            disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
            LeaveAlternateScreen,
        },
    },
};

use crate::Result;

/// A type alias to simplify the usage of the terminal type from the ratatui library.
pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal by enabling raw mode and entering the alternate screen.
///
/// This function should be called before the program starts to ensure that the terminal is in
/// the correct state for the application.
pub fn init() -> Result<Terminal> {
    #[cfg(feature = "color-eyre")]
    crate::error::install_hooks()?;
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal by leaving the alternate screen and disabling raw mode.
///
/// This function should be called before the program exits to ensure that the terminal is
/// restored to its original state.
pub fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        stdout(),
        LeaveAlternateScreen,
        Clear(ClearType::FromCursorDown),
    )
}
