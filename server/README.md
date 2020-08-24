# Darim Server

[![Server CI](https://github.com/ParkSB/darim/workflows/Server%20CI/badge.svg)](https://github.com/ParkSB/darim/actions?query=workflow%3A%22Server+CI%22)

* [flosse/rust-web-framework-comparison](https://github.com/flosse/rust-web-framework-comparison)
* [thecloudmaker/actix_tutorials](https://github.com/thecloudmaker/actix_tutorials)
* [actix/examples](https://github.com/actix/examples)
* [diesel.rs](http://diesel.rs/)
* [Rust Crypto](https://github.com/RustCrypto)
* [Yoshua Wuyts, "Error Handling Survey", 2019](https://blog.yoshuawuyts.com/error-handling-survey/)
* Ji-soo Kim, Jong-sub Moon, “A Study on Data Security of Web Local Storage”, _JICS_, vol.17, no. 3, pp. 55-66, Jun, 2016

![server transaction flow](https://user-images.githubusercontent.com/6410412/91041720-78b0a680-e64b-11ea-9dcf-198006a61b1e.png)

* `main.rs` - An entry point of the application. It runs a http server.
* Routes - A presentation layer that makes API public and passes request/response data to other layers.
* Services - A business layer that processes the transaction.
* Models - A data layer that can access the database and define data structures.
