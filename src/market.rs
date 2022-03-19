
use std::fmt::Display;
use rust_decimal::Decimal;

#[derive(Debug, Deserialize)]
pub struct Market {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub current_price: Option<Decimal>,
    pub market_cap: Option<Decimal>,
    pub market_cap_rank: Option<u64>,
    pub fully_diluted_valuation: Option<Decimal>,
    pub total_volume: Option<Decimal>,
    pub high_24h: Option<Decimal>,
    pub low_24h: Option<Decimal>,
    pub price_change_24h: Option<Decimal>,
    pub price_change_percentage_24h: Option<Decimal>,
    pub market_cap_change_24h: Option<Decimal>,
    pub market_cap_change_percentage_24h: Option<Decimal>,
    pub circulating_supply: Option<Decimal>,
    pub total_supply: Option<Decimal>,
    pub max_supply: Option<Decimal>,
    pub ath: Option<Decimal>,
    pub ath_change_percentage: Option<Decimal>,
    pub ath_date: Option<String>,
    pub atl: Option<Decimal>,
    pub atl_change_percentage: Option<Decimal>,
    pub atl_date: Option<String>,
    pub roi: Option<Roi>,
    pub last_updated: Option<String>,
    /// only available if specified in the request
    pub sparkline_in_7d: Option<SparklineIn7D>,

    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_14d_in_currency: Option<Decimal>,
    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_1h_in_currency: Option<Decimal>,
    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_1y_in_currency: Option<Decimal>,
    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_200d_in_currency: Option<Decimal>,
    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_24h_in_currency: Option<Decimal>,
    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_30d_in_currency: Option<Decimal>,
    /// only available if specified in the request's price_change_percentage, even then it can be None
    pub price_change_percentage_7d_in_currency: Option<Decimal>
}

#[derive(Debug, Deserialize)]
pub struct Roi {
    pub times: Option<Decimal>,
    pub currency: String,
    pub percentage: Option<Decimal>,
}