use std::collections::HashMap;

/// Information about a coin. INCOMPLETE
#[derive(Debug, Deserialize)]
pub struct CoinInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub asset_platf