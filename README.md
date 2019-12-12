# Airly-rs - in development process

<p align="center">
    <img 
    width="50%" height="50%" 
    src="https://github.com/Bartoshko/airly_rs/blob/master/assets/LogoBlue.svg"/>
</p>

Rust handler for Airly API - not official. 

This library purpose is to simplify calls to [Airly Api](https://developer.airly.eu/api).

At this point it is implemented with synchronous code execution. 
There is future plan for to add asynchronous calls based on new async/await rust implementation.


## Dependencies:

 - Project build with rust 2018 edition
 - serde: "1.0.103"
 - serde_json: "1.0.44"
 - serde_derive: "1.0.103"
 - reqwest: "0.9.22"

## Build and test:

- version is not set yet, and usage as library is not ready
- build with: `cargo build --release`
- test with: `cargo test --verbose`, for printing downloaded data: `cargo test -- --nocapture`

### Future development steps:

- 1 - documentation examples
- 2 - test integration with GitHub Rust build and test cargo action
- 3 - async api calls
- 4 - test coverage
- 5 - versioning and deployment to cargo crates.

