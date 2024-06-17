# Ratatui-calloopp

An experiment with using [Calloop] to drive a [Ratatui] app.

- [main.rs](src/main.rs) main entry point
  <https://github.com/joshka/ratatui-calloop/blob/5b7cf61e07e4400e7d6eeee4a0d14e11b4c49bdf/src/main.rs#L9-L17>
- [event.rs](src/event.rs): the application event loop and interaction with calloop
  <https://github.com/joshka/ratatui-calloop/blob/3743a96de83dcc444f131d1478bd08447e468c4b/src/event.rs#L40-L48>
- [app.rs](src/app.rs): the application logic
  <https://github.com/joshka/ratatui-calloop/blob/3743a96de83dcc444f131d1478bd08447e468c4b/src/app.rs#L33-L50>
- [error.rs](src/error.rs): color-eyre setup
- [tui.rs](src/tui.rs): ratatui setup

## Usage

```shell
gh repo clone https://github.com/joshka/ratatui-calloop
cargo run
```

[Ratatui]: https://crates.io/crates/ratatui
[Calloop]: https://crates.io/crates/calloop
