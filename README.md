# Ratatui-calloopp

An experiment with using [Calloop] to drive a [Ratatui] app.

- [src/main.rs]: main entry point
- [src/event.rs]: the application event loop and interaction with calloop
- [src/app.rs]: the application logic
- [src/error.rs]: color-eyre setup
- [src/tui.rs]: ratatui setup

## Usage

```shell
gh repo clone https://github.com/joshka/ratatui-calloop
cargo run
```

[Ratatui]: https://crates.io/crates/ratatui
[Calloop]: https://crates.io/crates/calloop
