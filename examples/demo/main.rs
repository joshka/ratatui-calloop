use std::io;

use calloop::LoopSignal;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui_calloop::{App, ApplicationLoop, Terminal};

mod util;

fn main() -> color_eyre::Result<()> {
    util::int_error_handling()?;
    util::init_logging()?;

    let mut app_loop = ApplicationLoop::new()?;
    let mut app = DemoApp::new(app_loop.exit_signal());
    app_loop.run(&mut app)?;
    Ok(())
}

/// The main application struct.
#[derive(Debug)]
pub struct DemoApp {
    /// A simple counter to demonstrate the application.
    counter: i32,
    exit_signal: LoopSignal,
}

impl DemoApp {
    /// Create a new `DemoApp`.
    pub fn new(exit_signal: LoopSignal) -> Self {
        Self {
            counter: 0,
            exit_signal,
        }
    }

    /// Exit the application.
    fn exit(&self) {
        self.exit_signal.stop();
    }

    /// Increment the counter.
    fn increment(&mut self) {
        self.counter += 1;
    }

    /// Decrement the counter.
    fn decrement(&mut self) {
        self.counter -= 1;
    }
}

impl App for DemoApp {
    /// Draw the application to the terminal.
    fn draw(&self, terminal: &mut Terminal) -> io::Result<()> {
        terminal.draw(|frame| {
            let output = format!(
                "Counter: {} <j: decrement, k: increment, q: quit>",
                self.counter
            );
            frame.render_widget(&output, Rect::default());
        })?;
        Ok(())
    }

    /// Handle a key event.
    fn on_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('j') => self.decrement(),
            KeyCode::Char('k') => self.increment(),
            _ => {}
        }
    }
}
