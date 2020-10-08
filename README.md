
# CoinGecko Rust

Rust crate for interacting with the [CoinGecko v3 API](https://www.coingecko.com/en/api).

At this time, async support is provided by [isahc](https://lib.rs/crates/isahc). I would like to have generic HTTP client support, but [surf](https://lib.rs/crates/surf) is currently unsuitable for user in a library due to it's error type not being compatible with `std::Error`. The `isahc` crate is suitable for now and ideal on Linux systems since it takes advantage of the system `libcurl` for HTTPS support.

## Supported APIs

- `/ping`