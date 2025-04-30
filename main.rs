mod matching_engine;
use matching_engine::orderbook::{Order, Orderbook, BidOrAsk};
use matching_engine::engine::{MatchingEngine, TradingPair};
fn main() {
    // let price = Price::new(50.5);
    // println!("{:?}", price);

    // let mut limit = Limit::new(64.0);
    // let buy_order = Order::new(BidOrAsk::Bid, 5.5);
    // let sell_order = Order::new(BidOrAsk::Ask, 6.2);
    // limit.add_order(buy_order); // this will require the "limit" to be mutable
    // limit.add_order(sell_order); // this will require the "limit" to be mutable
    // println!("{:?}", limit);

    println!("it works bc!");

    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);
    // let sell_order = Order::new(BidOrAsk::Ask, 6.2);

    let mut orderbook = Orderbook::new();
    orderbook.add_orderd(4.3, buy_order_from_alice);
    orderbook.add_orderd(4.3, buy_order_from_bob);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_orderd(20.0, sell_order);

    // println!("{:?}", orderbook);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    
    engine.add_new_market(pair.clone());
    
    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    engine.place_limit_order(pair, 10000.0, buy_order).unwrap();

    // this will panic the main.rs
    // let eth_pair = TradingPair::new("ETH".to_string(), "USD".to_string());
    // engine.place_limit_order(eth_pair, 10000.0, buy_order).unwrap();
}
