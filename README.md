# Ratatui-calloopp

An experiment with using [Calloop] to drive a [Ratatui] app.

- [main.rs](src/main.rs) main entry point
  <https://github.com/joshka/ratatui-calloop/blob/5b7cf61e07e4400e7d6eeee4a0d14e11b4c49bdf/src/main.rs#L9-L17>
- [event.rs](src/event.rs): the application event loop and interaction with calloop
- [app.rs](src/app.rs): the application logic
- [error.rs](src/error.rs): color-eyre setup
- [tui.rs](src/tui.rs): ratatui setup

## Usage

```shell
gh repo clone https://github.com/joshka/ratatui-calloop
cargo run
```

[Ratatui]: https://crates.io/crates/ratatui
[Calloop]: https://crates.io/crates/calloop
