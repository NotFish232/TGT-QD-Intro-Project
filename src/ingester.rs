use std::net::TcpStream;

use tungstenite::{Message, WebSocket, connect, stream::MaybeTlsStream};

use crate::{
    models::{CurrencyRequest, OrderBookSnapshot, ServerInfo, SubscribeInfo},
    order_book::OrderBook,
};
use std::error::Error;

// struct that ingests websocket data and updates orderbook accordingly
// websocket is the open websocket connection to the exchange
// order_book is originally initialized as empty because to initialize it we need to pull
// a snapshot of the initial order_book state, which will be done in the run method
// currency_request is the currency and the number of levels we want
#[derive(Debug)]
pub struct Ingester {
    websocket: WebSocket<MaybeTlsStream<TcpStream>>,
    order_book: OrderBook,
    currency_request: CurrencyRequest,
}

impl Ingester {
    pub fn new(
        exchange_url: &str,
        currency_request: CurrencyRequest,
    ) -> Result<Self, Box<dyn Error>> {
        let (mut websocket, _) = connect(exchange_url)?;

        // parse initial websocket msg which is server information
        let initial_msg_bytes = websocket.read()?.into_text()?;
        let _server_info: ServerInfo = serde_json::from_str(&initial_msg_bytes)?;

        Ok(Self {
            websocket,
            order_book: OrderBook::default(),
            currency_request,
        })
    }

    // exchange only supports fetching pricepoints 25, 100, or 250
    // finds the next higher or raises an error if too high
    fn get_num_price_points(&self) -> Result<u64, Box<dyn Error>> {
        match self.currency_request.num_levels {
            0..=25 => Ok(25),
            26..=100 => Ok(100),
            101..=250 => Ok(250),
            _ => Err("Ingester only supports up to 250 levels".into()),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // subscribe to the book websocket by sending initial message
        let subscribe_msg = format!(
            r#"{{
            "event": "subscribe",
            "channel": "book",
            "symbol": "{}",
            "len": {}
            }}"#,
            self.currency_request.name,
            self.get_num_price_points()?
        );
        self.websocket.send(Message::Text(subscribe_msg.into()))?;

        // consume the message that confirms the subscription was successful
        let subscribe_bytes = self.websocket.read()?.into_text()?;
        let _subscribe_info: SubscribeInfo = serde_json::from_str(&subscribe_bytes)?;

        // consume the next message that contains the initial snapshot of the orderbook
        let order_book_snapshot_bytes = self.websocket.read()?.into_text()?;
        let order_book_snapshot: OrderBookSnapshot =
            serde_json::from_str(&order_book_snapshot_bytes)?;
        // makes the initial order book based on the snapshot
        self.order_book = OrderBook::new(order_book_snapshot, self.currency_request.num_levels);

        self.order_book.display();

        loop {
            let order_book_update_bytes = self.websocket.read()?.into_text()?;

            if let Ok(order_book_update) = serde_json::from_str(&order_book_update_bytes) {
                self.order_book.update(order_book_update)?;
                self.order_book.display();
            }
        }
    }
}
