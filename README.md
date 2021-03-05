# RUSH

RUst SHell.

![](https://github.com/luchev/rush/blob/master/demo.gif?raw=true)

## Installation

To build the project you will need the Nightly [Rust compiler](https://rustup.rs/). You'll need to activate the nightly mode

```
rustup default nightly
rustup update
```

Clone the repo

```
git clone https://github.com/luchev/rush
```

Enter the directory and run the project

```
cd rush
cargo run
```

## Running tests

### Unit tests

The tests rely on environment variables so they cannot be ran in parallel. You can run unit tests with

```
cargo test -- --test-threads=1
```

### Integration tests

In order to run integration tests you will need to build the project first. 
Secondly you need to download the integration tests from [build-your-own-shell](https://github.com/tokenrove/build-your-own-shell).
You also need [expect](https://wiki.tcl-lang.org/page/Expect), which is usually in a package called expect, and a C compiler.
Finally run the integration tests.

```
cargo build --release
git clone https://github.com/tokenrove/build-your-own-shell
build-your-own-shell/validate ./target/release/rush
```

## Roadmap

To see the roadmap and what's been implemented as well as future plans go to the [Roadmap](https://github.com/luchev/rush/projects/2).

## Acknowledgements

Huge thanks to **tokenrove** and their repo [build-your-own-shell](https://github.com/tokenrove/build-your-own-shell), which inspired this project.

The Rust team at Sofia University in the face of [Andrew](https://github.com/AndrewRadev) and team for the inspiration to try this amazing language Rust.
