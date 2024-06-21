use std::{io, time::Duration};

use crate::{
    crossterm::CrosstermEventSource,
    ratatui::{init_terminal, restore_terminal, Terminal},
};
use calloop::{EventLoop, LoopSignal};
use color_eyre::eyre::eyre;
use crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};

pub trait App {
    const IGNORE_KEY_RELEASE: bool = true;

    fn draw(&self, terminal: &mut Terminal) -> io::Result<()>;
    fn on_crossterm_event(&mut self, event: CrosstermEvent) {
        match event {
            CrosstermEvent::FocusGained => self.on_focus_gained(),
            CrosstermEvent::FocusLost => self.on_focus_lost(),
            CrosstermEvent::Key(key_event) => {
                if !Self::IGNORE_KEY_RELEASE || key_event.kind == KeyEventKind::Press {
                    self.on_key_event(key_event);
                }
            }
            CrosstermEvent::Mouse(event) => self.on_mouse_event(event),
            CrosstermEvent::Paste(text) => self.on_paste(text),
            CrosstermEvent::Resize(width, height) => self.on_resize(width, height),
        }
    }
    fn on_focus_lost(&mut self) {}
    fn on_focus_gained(&mut self) {}

    #[allow(unused_variables)]
    fn on_key_event(&mut self, event: KeyEvent) {}

    #[allow(unused_variables)]
    fn on_mouse_event(&mut self, event: MouseEvent) {}

    #[allow(unused_variables)]
    fn on_paste(&mut self, text: String) {}

    #[allow(unused_variables)]
    fn on_resize(&mut self, width: u16, height: u16) {}
}

/// The main event loop for the application.
///
/// This is based on the `calloop` crate, which provides a cross-platform event loop that can handle
/// multiple sources of events. In this case, we're using it to handle both the terminal input and
/// the frame timing for the TUI.
pub struct ApplicationLoop<T: App> {
    event_loop: EventLoop<'static, T>,
}

impl<T: App> ApplicationLoop<T> {
    /// Create a new `ApplicationLoop`.
    ///
    /// This will create a new event loop and insert a source for crossterm events.
    pub fn new() -> color_eyre::Result<Self> {
        let event_loop = EventLoop::<T>::try_new()?;
        let crossterm_event_source = CrosstermEventSource::new()?;
        event_loop
            .handle()
            .insert_source(crossterm_event_source, |event, _metadata, app| {
                app.on_crossterm_event(event);
            })
            .map_err(|e| eyre!("failed to insert crossterm event source: {e}"))?;

        Ok(Self { event_loop })
    }

    /// Run the event loop.
    ///
    /// This will run the event loop until the application signals that it should stop.
    /// The application will be drawn to the terminal on each frame (60 fps).
    pub fn run(&mut self, app: &mut T) -> color_eyre::Result<()> {
        let mut terminal = init_terminal()?;
        let frame_rate = Duration::from_secs_f32(1.0 / 2.0); // 60 fps
        self.event_loop.run(frame_rate, app, |app| {
            // TODO handle errors here nicely somehow rather than swallowing them
            // likely needs to send a message or something
            let _ = app.draw(&mut terminal);
        })?;
        restore_terminal()?;
        Ok(())
    }

    /// Get the loop signal.
    ///
    /// This can be used to stop the event loop from outside the loop (e.g. when the application
    /// should exit).s
    pub fn exit_signal(&self) -> LoopSignal {
        self.event_loop.get_signal()
    }
}
