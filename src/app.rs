use calloop::LoopSignal;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::Rect;

use crate::tui::Terminal;

/// The main application struct.
pub struct App {
    /// The signal used to stop the event loop.
    loop_signal: LoopSignal,
    /// A simple counter to demonstrate the application.
    counter: i32,
}

impl App {
    pub fn new(loop_signal: LoopSignal) -> Self {
        Self {
            loop_signal,
            counter: 0,
        }
    }

    pub fn draw(&self, terminal: &mut Terminal) {
        let output = format!(
            "Counter: {} <j: decrement, k: increment, q: quit>",
            self.counter
        );
        let _ = terminal.draw(|frame| {
            frame.render_widget(&output, Rect::default());
        });
    }

    pub fn on_crossterm_event(&mut self, event: Event) {
        match event {
            Event::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    return;
                }
                match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Char('j') => self.counter -= 1,
                    KeyCode::Char('k') => self.counter += 1,
                    _ => {}
                }
            }
            _ => {
                // ignore other events
            }
        }
    }

    fn exit(&self) {
        self.loop_signal.stop();
    }
}
