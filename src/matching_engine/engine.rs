use super::orderbook::{Orderbook, Order};
use std::collections::HashMap; // this is a hashmap, it's a key-value store, it's like a dictionary in python, it's like a json objec
use rust_decimal::prelude::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String, // in BTC/USD => BTC = base and USD = quote
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair {
            base,
            quote,
        }
    }

    pub fn to_string(self) -> String {
        format!("{}/{}", self.base, self.quote)
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
}

impl MatchingEngine{
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair:TradingPair){
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("opening a new orderbook for market {:?}", pair.to_string());
    }

    pub fn place_limit_order(&mut self, pair: TradingPair, price: Decimal, order: Order) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);

                println!("Placing limit order for {:?} at price {:?}", pair.to_string(), price);

                Ok(())
            }
            None => Err(format!("No orderbook exists for market {:?}", pair.to_string())),
        }
    }
}
