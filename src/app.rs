use std::time::Duration;

use crate::{crossterm::CrosstermEventSource, Result};
use calloop::{EventLoop, LoopSignal};
use ratatui::{
    crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent},
    Frame,
};

/// A trait for the main application struct.
///
/// This trait defines the main interface for the application. It provides methods for drawing the
/// application to the terminal and handling input events. Applications should implement this trait
/// to define their behavior.
///
/// The `App` trait provides a default implementation for handling crossterm events. This
/// implementation will call the appropriate event handler method based on the event type. The
/// default implementation will ignore key release events, but this can be overridden by setting
/// the `IGNORE_KEY_RELEASE` constant to `false`.
///
/// # Example
///
/// ```
/// use std::io;
/// use ratatui_calloop::{App, Terminal};
/// use ratatui::{crossterm::event::{KeyCode, KeyEvent}, text::Text};
///
/// struct MyApp {
///     should_exit: bool,
///     counter: i32,
/// }
///
/// impl App for MyApp {
///     fn draw(&self, terminal: &mut Terminal) -> io::Result<()> {
///         terminal.draw(|frame| {
///             let text = Text::raw(format!("Counter: {}", self.counter));
///             frame.render_widget(&text, frame.size());
///         })?;
///         Ok(())
///     }
///
///     fn on_key_event(&mut self, event: KeyEvent) {
///         match event.code {
///             KeyCode::Char('j') => self.counter -= 1,
///             KeyCode::Char('k') => self.counter += 1,
///             KeyCode::Char('q') => self.should_exit = true,
///             _ => {}
///         }
///     }
/// }
/// ```
pub trait App {
    const IGNORE_KEY_RELEASE: bool = true;

    /// Draw the application to the terminal.
    ///
    /// This method should draw the application to the terminal using the provided `Terminal` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use std::io;
    /// use ratatui_calloop::App;
    /// use ratatui::{text::Text, Frame};
    ///
    /// struct MyApp {
    ///     counter: i32,
    /// }
    ///
    /// impl App for MyApp {
    ///     fn draw(&self, frame: &mut Frame) {
    ///         let text = Text::raw(format!("Counter: {}", self.counter));
    ///         frame.render_widget(&text, frame.size());
    ///         Ok(())
    ///     }
    /// }
    /// ```
    fn draw(&self, frame: &mut Frame);

    /// Handle a crossterm event.
    ///
    /// This method will be called when a crossterm event is received. The default implementation
    /// will call the appropriate event handler method based on the event type. This can be
    /// overridden to provide custom event handling.
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

    /// Handle the focus lost event.
    ///
    /// This method will be called when the application loses focus.
    fn on_focus_lost(&mut self) {}

    /// Handle the focus gained event.
    ///
    /// This method will be called when the application gains focus.
    fn on_focus_gained(&mut self) {}

    /// Handle a key event.
    ///
    /// This method will be called when a key event is received.
    #[allow(unused_variables)]
    fn on_key_event(&mut self, event: KeyEvent) {}

    /// Handle a mouse event.
    ///
    /// This method will be called when a mouse event is received.
    #[allow(unused_variables)]
    fn on_mouse_event(&mut self, event: MouseEvent) {}

    /// Handle a paste event.
    ///
    /// This method will be called when a paste event is received.
    #[allow(unused_variables)]
    fn on_paste(&mut self, text: String) {}

    /// Handle a resize event.
    ///
    /// This method will be called when a resize event is received.
    #[allow(unused_variables)]
    fn on_resize(&mut self, width: u16, height: u16) {}
}

/// The main event loop for the application.
///
/// This is based on the [Calloop] crate, which provides a cross-platform event loop that can handle
/// multiple sources of events. In this case, we're using it to handle both the terminal input and
/// the frame timing for the TUI.
///
/// [Calloop]: https://docs.rs/calloop
pub struct ApplicationLoop<T: App> {
    event_loop: EventLoop<'static, T>,
}

impl<T: App> ApplicationLoop<T> {
    /// Create a new `ApplicationLoop`.
    ///
    /// This will create a new event loop and insert a source for crossterm events.
    pub fn new() -> Result<Self> {
        let event_loop = EventLoop::<T>::try_new()?;
        let crossterm_event_source = CrosstermEventSource::new()?;
        event_loop
            .handle()
            .insert_source(crossterm_event_source, |event, _metadata, app| {
                app.on_crossterm_event(event);
            })
            .map_err(|e| e.error)?;

        Ok(Self { event_loop })
    }

    /// Run the event loop.
    ///
    /// This will run the event loop until the application signals that it should stop.
    /// The application will be drawn to the terminal on each frame (60 fps).
    pub fn run(&mut self, app: &mut T) -> Result<()> {
        let mut terminal = ratatui::init();
        let frame_rate = Duration::from_secs_f32(1.0 / 2.0); // 60 fps
        self.event_loop.run(frame_rate, app, |app| {
            // TODO handle errors here nicely somehow rather than swallowing them
            // likely needs to send a message or something
            let _ = terminal.draw(|frame| app.draw(frame));
        })?;
        ratatui::restore();
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
