use std::io::{self, stdout, Stdout};

use crossterm::{
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::backend::CrosstermBackend;

// A type alias to simplify the usage of the terminal and make it easier to change the backend
// or choice of writer.
pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal by enabling raw mode and entering the alternate screen.
///
/// This function should be called before the program starts to ensure that the terminal is in
/// the correct state for the application.
pub fn init_terminal() -> io::Result<Terminal> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    Terminal::new(backend)
}

/// Restore the terminal by leaving the alternate screen and disabling raw mode.
///
/// This function should be called before the program exits to ensure that the terminal is
/// restored to its original state.
pub fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        stdout(),
        LeaveAlternateScreen,
        Clear(ClearType::FromCursorDown),
    )
}
