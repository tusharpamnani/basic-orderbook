# 🔁 Basic Orderbook Matching Engine in Rust

A **simple orderbook and matching engine** for a cryptocurrency exchange built in **Rust**. Supports **limit** and **market** orders, maintains separate orderbooks for multiple trading pairs (e.g., `BTC/USD`), and uses `rust_decimal` to avoid floating-point precision issues.

> 🧪 Designed for experimentation, learning, and extensibility.

**📍 Repo**: [tusharpamnani/basic-orderbook](https://github.com/tusharpamnani/basic-orderbook)

---

## 🚀 Features

- 🏛️ Market-specific orderbooks (e.g., BTC/USD)
- 📥 Limit order placement (bid or ask)
- ⚡ Market order execution (fills at best price)
- 📉 Price sorting and order matching logic
- 🧮 Precision-safe decimal handling with `rust_decimal`
- ✅ Unit-tested core logic

---

## 📁 Project Structure

```
src/
├── main.rs                      # Entry point
└── matching_engine/
    ├── engine.rs                # Matching engine logic (multi-market)
    └── orderbook.rs             # Core orderbook, order types, matching
    └── mod.rs                   # Module definitions
```

---

## 📦 Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
rust_decimal = "1"
rust_decimal_macros = "1"
```

---

## 🔧 Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/tusharpamnani/basic-orderbook.git
cd basic-orderbook
```

### 2. Build the project

```bash
cargo build
```

### 3. Run the demo

```bash
cargo run
```

Sample output:
```
it works bc!
opening a new orderbook for market "BTC/USD"
Placing limit order for "BTC/USD" at price 10000
```

---

## 🧪 Running Tests

This project includes unit tests to verify core matching logic and edge cases.

Run tests with:

```bash
cargo test
```

Test coverage includes:

- ✅ Limit order matching (single & multi-fill)
- ✅ Market order execution
- ✅ Total volume calculation
- ✅ Orderbook structure and behavior

---

## 🧠 Example Code

```rust
use rust_decimal_macros::dec;
use matching_engine::orderbook::{Order, BidOrAsk, Orderbook};

let mut orderbook = Orderbook::new();
orderbook.add_limit_order(dec!(100.0), Order::new(BidOrAsk::Bid, 5.0));

let mut market_sell = Order::new(BidOrAsk::Ask, 5.0);
orderbook.fill_market_order(&mut market_sell);
```

---

## 📌 Design Decisions

- `Decimal` is used for **prices** to safely use them as `HashMap` keys and ensure precision.
- `f64` is used for **order sizes** for simplicity but may be replaced with `Decimal` for full precision control.
- Matching logic:
  - Market `Bid` matches against sorted ascending `Asks`
  - Market `Ask` matches against sorted descending `Bids`

---

## 🛠️ Planned Improvements

- ⏳ Support for order expiry / time-in-force
- ❌ Cancel limit orders
- 🆔 Add order IDs for tracking
- 💾 Persistent storage (e.g., JSON or DB)
- 🌐 REST / WebSocket API for real-time order placement

---