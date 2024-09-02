use calloop::LoopSignal;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    text::Text,
    Frame,
};
use ratatui_calloop::{App, ApplicationLoop, Result};

fn main() -> Result<()> {
    let mut app_loop = ApplicationLoop::new()?;
    let mut app = DemoApp {
        counter: 0,
        exit_signal: app_loop.exit_signal(),
    };
    app_loop.run(&mut app)
}

pub struct DemoApp {
    counter: i32,
    exit_signal: LoopSignal,
}

impl App for DemoApp {
    fn draw(&self, frame: &mut Frame) {
        let text = Text::raw(format!("Counter: {} <↑/↓> <q: quit>", self.counter));
        frame.render_widget(&text, Rect::default());
    }

    fn on_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Up => self.counter += 1,
            KeyCode::Down => self.counter -= 1,
            KeyCode::Char('q') => self.exit_signal.stop(),
            _ => {}
        }
    }
}
