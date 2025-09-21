use crate::models::{OrderBookSnapshot, OrderBookUpdate};
use std::{
    collections::{BTreeMap, btree_map::Entry},
    error::Error,
};

#[derive(Debug, Default)]
pub struct OrderBook {
    bids: BTreeMap<String, u64>,
    asks: BTreeMap<String, u64>,
    num_levels: u64,
}

impl OrderBook {
    pub fn new(order_book_snapshot: OrderBookSnapshot, num_levels: u64) -> Self {
        let mut asks = BTreeMap::new();
        let mut bids = BTreeMap::new();

        for entry in order_book_snapshot.entries {
            // per the documentation if amount > 0 then bids else asks
            if entry.amount > 0.0 {
                bids.insert(entry.price.to_string(), entry.count);
            } else {
                asks.insert(entry.price.to_string(), entry.count);
            }
        }

        OrderBook {
            bids,
            asks,
            num_levels,
        }
    }

    pub fn update(&mut self, order_book_update: OrderBookUpdate) -> Result<(), Box<dyn Error>> {
        // if amount is positive update bids else update asks
        let map = if order_book_update.data.amount > 0.0 {
            &mut self.bids
        } else {
            &mut self.asks
        };

        if order_book_update.data.count == 0 {
            if let Entry::Occupied(e) = map.entry(order_book_update.data.price.to_string()) {
                e.remove_entry();
            } else {
                return Err(format!(
                    "Error, cannot update OrderBook, price {} not found",
                    order_book_update.data.price
                )
                .into());
            }
        } else {
            map.insert(
                order_book_update.data.price.to_string(),
                order_book_update.data.count,
            );
        }

        Ok(())
    }
}
