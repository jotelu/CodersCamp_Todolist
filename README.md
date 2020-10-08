
# CoinGecko Rust

Rust crate for interacting with the [CoinGecko v3 API](https://www.coingecko.com/en/api).

At this time, async support is provided by [isahc](https://lib.rs/crates/isahc). I would like to have generic HTTP client support, but [surf](https://lib.rs/crates/surf) is currently unsuitable for user in a library due to it's error type not being compatible with `std::Error`. The `isahc` crate is suitable for now and ideal on Linux systems since it takes advantage of the system `libcurl` for HTTPS support.

## Supported APIs

- `/ping`
- `/simple/price`
- `/coins/list`
- `/coins/{id}`
- `/coins/markets`

## Example

```rust
use coingecko::{Client, SimplePriceReq};

pub fn main() {
    smol::block_on(async {
        let http = isahc::HttpClient::new().unwrap();

        let client = Client::new(http);

        let req = SimplePriceReq::new("ethereum,algorand,tezos".into(), "usd".into())
            .include_market_cap()
            .include_24hr_vol()
            .include_24hr_change()
            .include_last_updated_at();

        println!("{:#?}", client.simple_price(req).await);
    })
}
```

```rust