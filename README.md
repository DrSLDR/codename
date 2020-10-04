# Codename

Codename is a small utility program to randomly generate phrases, akin to
GfyCat URLs and Jitsi room names.

### Installation

The simplest way to get the binary is to install it via cargo

```bash
$ cargo install codename
```

If you instead prefer to build from source, you can clone this repository and
build/run via `cargo build` and `cargo run` respectively.

### License

Codename is licensed under the MIT license.

The wordlists and basic logic are adapted from Jitsi's
[js-utils](https://github.com/jitsi/js-utils) collection, specifically the
[roomNameGenerator](https://github.com/jitsi/js-utils/blob/master/random/roomNameGenerator.js).
