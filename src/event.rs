use std::{thread, time::Duration};

use calloop::{
    channel::{channel, Channel, Event},
    EventLoop, LoopSignal,
};
use color_eyre::{eyre::eyre, Result};
use crossterm::event::Event as CrosstermEvent;

use crate::{tui, App};

/// The main event loop for the application.
///
/// This is based on the `calloop` crate, which provides a cross-platform event loop that can handle
/// multiple sources of events. In this case, we're using it to handle both the terminal input and
/// the frame timing for the TUI.
pub struct ApplicationLoop {
    event_loop: EventLoop<'static, App>,
}

impl ApplicationLoop {
    /// Create a new `ApplicationLoop`.
    ///
    /// This will create a new event loop and insert a source for crossterm events.
    pub fn new() -> Result<Self> {
        let event_loop = EventLoop::<App>::try_new()?;
        let crossterm_event_channel = start_crossterm_event_thread();
        event_loop
            .handle()
            .insert_source(crossterm_event_channel, |event, _metadata, app| {
                if let Event::Msg(event) = event {
                    app.on_crossterm_event(event);
                }
            })
            .map_err(|e| eyre!("failed to insert crossterm event source: {e}"))?;

        Ok(Self { event_loop })
    }

    /// Run the event loop.
    ///
    /// This will run the event loop until the application signals that it should stop.
    /// The application will be drawn to the terminal on each frame (60 fps).
    pub fn run(&mut self, app: &mut App) -> Result<()> {
        let mut terminal = tui::init_terminal()?;
        let frame_rate = Duration::from_secs_f32(1.0 / 60.0); // 60 fps
        self.event_loop.run(frame_rate, app, |app| {
            app.draw(&mut terminal);
        })?;
        tui::restore_terminal()?;
        Ok(())
    }

    /// Get the loop signal.
    ///
    /// This can be used to stop the event loop from outside the loop (e.g. when the application
    /// should exit).s
    pub fn loop_signal(&self) -> LoopSignal {
        self.event_loop.get_signal()
    }
}

/// Start a thread that polls for crossterm events and sends them to a channel.
fn start_crossterm_event_thread() -> Channel<CrosstermEvent> {
    let (sender, channel) = channel();
    thread::spawn(move || {
        while let Ok(event) = crossterm::event::read() {
            if sender.send(event).is_err() {
                // the channel has been dropped, so no need to keep reading events
                break;
            }
        }
    });
    channel
}
