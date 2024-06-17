# Ratatui-calloopp

An experiment with using [Calloop] to drive a [Ratatui] app.

- [main.rs](src/main.rs) main entry point
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
