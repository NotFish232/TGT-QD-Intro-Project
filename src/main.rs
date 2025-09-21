mod ingester;
mod models;
mod order_book;

use crate::{ingester::Ingester, models::CurrencyRequest};

fn main() {
    let exchange_url = "wss://api-pub.bitfinex.com/ws/2";

    let currency_request = CurrencyRequest {
        name: "tETHUSD".to_string(),
        num_levels: 5,
    };

    // create a new ingester with an exchange_url and some currency and num levels and run

    let mut ingester =
        Ingester::new(exchange_url, currency_request).expect("Could not initialize Ingester");

    ingester.run().expect("Error running Ingester");
}
