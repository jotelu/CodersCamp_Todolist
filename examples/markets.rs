use coingecko::{CoinsMarketsReq, MarketsReqOrder};

pub fn main() {
    smol::block_on(async {
        l