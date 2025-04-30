#![allow(dead_code)]

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

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
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

    pub fn fill_order(&mut self, market_order: &mut Order) {
        for limit_orders in self.orders.iter_mut() {
            match market_order.size >= limit_orders.size {
                true => {
                    market_order.size -= limit_orders.size;
                    limit_orders.size = 0.0;
                }
                false => {
                    limit_orders.size -= market_order.size;
                    market_order.size = 0.0;
                }
            }
            if market_order.is_filled() {
                break;
            }
        }
    }

    pub fn total_volume(&self) -> f64 {
        return self.orders
            .iter()
            .map(|order| order.size)
            .reduce(|a, b| a + b)
            .unwrap();
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

    // TODO: SORTING LIMITS BY PRICE
    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        return self.asks.values_mut().collect::<Vec<&mut Limit>>();
    }

    
    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        return self.bids.values_mut().collect::<Vec<&mut Limit>>();
    }
    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        match market_order.bid_or_ask {
            BidOrAsk::Bid => {
                for limit_order in self.ask_limits() {
                    limit_order.fill_order(market_order);

                    if market_order.is_filled() {
                        break;
                    }
                }
            }
            BidOrAsk::Ask => {}
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn limit_order_single_fill() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 97.0);
        limit.fill_order(&mut market_sell_order);

        println!("{:?}", limit);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 3.0);
    }

    #[test]
    fn limit_order_multi_fill() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order_alice = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order_bob = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_alice);
        limit.add_order(buy_limit_order_bob);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 197.0);
        limit.fill_order(&mut market_sell_order);

        println!("{:?}", limit);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.orders.get(1).unwrap().is_filled(), false);
        assert_eq!(limit.orders.get(1).unwrap().size, 3.0);
    }

    #[test]
    fn limit_total_volume() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order_alice = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order_bob = Order::new(BidOrAsk::Bid, 99.0);

        limit.add_order(buy_limit_order_alice);
        limit.add_order(buy_limit_order_bob);

        assert_eq!(limit.total_volume(), 199.0);
    }
}