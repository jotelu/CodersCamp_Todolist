
use std::fmt::Display;
use rust_decimal::Decimal;

#[derive(Debug, Deserialize)]
pub struct Market {
    pub id: String,
    pub symbol: String,