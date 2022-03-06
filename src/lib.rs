
#[macro_use]
extern crate derive_setters;
#[macro_use]
extern crate fomat_macros;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

mod coin_info;
mod market;
mod simple_price;
mod utils;

use const_format::concatcp;

use isahc::HttpClient;

pub use crate::coin_info::*;
pub use crate::simple_price::*;
pub use crate::market::*;

const API: &str = "https://api.coingecko.com/api/v3/";

pub struct Client {
    http: HttpClient,
}

impl Client {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Check if CoinGecko is reachable
    pub async fn ping(&self) -> Result<Ping, Error> {
        const PING: &str = concatcp!(crate::API, "/ping");

        utils::get_json(&self.http, PING).await
    }

    /// Fetches the current price of any cryptocurrencies in any other supported currencies you need.
    pub async fn simple_price(&self, req: SimplePriceReq) -> Result<SimplePrices, Error> {
        const SIMPLE: &str = concatcp!(crate::API, "/simple/price");

        let uri = fomat!((SIMPLE) "?" (req.query()));

        utils::get_json(&self.http, &uri).await
    }

    /// Fetches detailed information about a particular coin by its ID.
    pub async fn coin_info(&self, coin: &str) -> Result<CoinInfo, Error> {
        const COINS: &str = concatcp!(crate::API, "/coins");

        let uri = fomat!((COINS) "/" (coin));