pub fn main() {
    smol::block_on(async {
        let http = isahc::HttpClient::new().unwrap();

        let client = coingecko::Client::new(http);

        pr