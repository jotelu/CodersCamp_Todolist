use rust_decimal::Decimal;
use std::collections::HashMap;

pub type SimplePrice = HashMap<String, Decimal>;
pub type SimplePrices = HashMap<String, SimplePrice>;

#[derive(Default, Setters)]
pub struct SimplePriceReq {
    /// ids of coins, comma-separated
    #[setters(skip)]
    pub ids: String,

    /// ids of currency pairs, comma-separated
    #[setters(sk