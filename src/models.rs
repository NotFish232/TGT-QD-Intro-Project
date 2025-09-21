use core::str;

use serde::Deserialize;

// A struct representing which currency we are interested in viewing
// contains the num levels we want to fetch on that currency
#[derive(Debug)]
pub struct CurrencyRequest {
    pub name: String,
    pub num_levels: u64,
}

// a struct representing the server information returned by the server on first websocket request
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub version: u64,
    pub server_id: String,
}

// a struct representing the information returned by the server after subscribing
#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct SubscribeInfo {
    #[serde(rename(deserialize = "chanId"))]
    pub channel_id: u64,
    #[serde(rename(deserialize = "symbol"))]
    pub currency_name: String,
}

// struct for a single order book entry
// contains price as a float, count as an unsigned integer, and amount as a float
#[derive(Debug, Deserialize)]
pub struct OrderBookEntry {
    pub price: f64,
    pub count: u64,
    pub amount: f64,
}

// struct for a single order book update
// contains  the channel id and an order book entry
#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct OrderBookUpdate {
    #[serde(rename = "0")]
    pub channel_id: u64,
    #[serde(rename = "1")]
    pub data: OrderBookEntry,
}

// struct for the initial message with the snapshot of the order book
#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct OrderBookSnapshot {
    #[serde(rename = "0")]
    pub channel_id: u64,
    #[serde(rename = "1")]
    pub entries: Vec<OrderBookEntry>,
}
