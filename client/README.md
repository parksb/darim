# Patic Client

[![Client CI](https://github.com/ParkSB/patic/workflows/Client%20CI/badge.svg)](https://github.com/ParkSB/patic/actions?query=workflow%3A%22Client+CI%22)

* [seed-rs.org](https://seed-rs.org/)
* [seed-rs/seed-quickstart](https://github.com/seed-rs/seed-quickstart)
* [seed-rs/seed/examples](https://github.com/seed-rs/seed/tree/master/examples)

```shell script
$ rustup update
$ rustup target add wasm32-unknown-unknown
$ cargo install cargo-make
```

```shell script
$ cargo make build
$ cargo make serve # Runs dev server on 127.0.0.1:8000
```
