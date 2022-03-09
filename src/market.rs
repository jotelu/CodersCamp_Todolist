
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