pub fn main() {
    smol::block_on(async {
        let http = isahc::HttpClient::n