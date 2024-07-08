use std::{io, result::Result, thread};

use calloop::{
    channel::{channel, Channel, Event, Sender},
    EventSource, Poll, Readiness, Token, TokenFactory,
};
use ratatui::crossterm::{
    event::{self, Event as CrosstermEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};

/// A [`calloop::EventSource`] of [`CrosstermEvent`]s.
///
/// This wraps the crossterm event handling in a `calloop` event source.
#[derive(Debug)]
pub struct CrosstermEventSource {
    channel: Channel<CrosstermEvent>,
}

impl CrosstermEventSource {
    /// Create a new `CrosstermEventSource`.
    ///
    /// This will enable raw mode on the terminal and start reading events on a separate thread.
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let (sender, channel) = channel();
        thread::spawn(move || Self::read_events(sender));
        Ok(Self { channel })
    }

    /// Read events from crossterm and send them to the channel.
    fn read_events(sender: Sender<CrosstermEvent>) {
        while let Ok(event) = event::read() {
            if sender.send(event).is_err() {
                break; // no need to keep reading event if the receiver is gone
            }
        }
    }
}

impl Drop for CrosstermEventSource {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

impl EventSource for CrosstermEventSource {
    type Event = CrosstermEvent;
    type Metadata = ();
    type Ret = ();
    type Error = io::Error;

    fn process_events<C>(
        &mut self,
        readiness: Readiness,
        token: Token,
        mut callback: C,
    ) -> Result<calloop::PostAction, Self::Error>
    where
        C: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        self.channel
            .process_events(readiness, token, |event, _metadata| {
                if let Event::Msg(event) = event {
                    callback(event, &mut ());
                }
            })
            .map_err(io::Error::other)
    }

    fn register(
        &mut self,
        poll: &mut Poll,
        token_factory: &mut TokenFactory,
    ) -> calloop::Result<()> {
        self.channel.register(poll, token_factory)
    }

    fn reregister(
        &mut self,
        poll: &mut Poll,
        token_factory: &mut TokenFactory,
    ) -> calloop::Result<()> {
        self.channel.reregister(poll, token_factory)
    }

    fn unregister(&mut self, poll: &mut Poll) -> calloop::Result<()> {
        self.channel.unregister(poll)
    }
}
