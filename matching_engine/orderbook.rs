use std::collections::HashMap; // this is a hashmap, it's a key-value store, it's like a dictionary in python, it's like a json objec

#[derive(Debug)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

#[derive(Debug)]
pub struct Limit {
    price: Price, // why not f64? because we'll be adding this to a hashmap and f64 isn't a good choice for hashmap
    orders: Vec<Order>,
}

#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order {
            bid_or_ask,
            size,
        }
    }
}

impl Price {
    pub fn new(price: f64) -> Price {
        let scalar = 1000000;
        let integral = price as u64; // casting a f64 to u64
        let fractional = ((price % 1.0) * (scalar as f64)) as u64;
        Price {
            integral,
            fractional,
            scalar,
        }
    }
}

impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_orderd(&mut self, price: f64, order: Order) {
        let price = Price::new(price);
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                match self.bids.get_mut(&price) {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }
            BidOrAsk::Ask =>
                match self.asks.get_mut(&price) {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    }
                }
        }
    }
}
