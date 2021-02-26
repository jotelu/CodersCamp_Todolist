use coingecko::{Client, SimplePriceReq};

pub fn main() {
    smol::block_on(async {
        let http = isahc::HttpClient::new().unwrap();

        let client = Client::new(http);

        let req = SimplePriceReq::new("ethereum,algorand,tezos".into(), "usd".into())
            .include_market