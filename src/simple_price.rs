use rust_decimal::Decimal;
use std::collections::HashMap;

pub type SimplePrice = HashMap<String, Decimal>;
pub type SimplePrices = HashMap<String, SimplePrice>;

#[derive(Default, Sett