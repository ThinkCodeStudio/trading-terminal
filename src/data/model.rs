// src/data/model.rs
#[derive(Clone, Debug)]
pub struct Candle {
    pub time: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[derive(Clone, Debug)]
pub struct OrderBookLevel {
    pub price: f64,
    pub size: f64,
}

#[derive(Clone, Debug)]
pub struct OrderBookSnapshot {
    pub bids: Vec<OrderBookLevel>, // 按价格降序
    pub asks: Vec<OrderBookLevel>, // 按价格升序
}

#[derive(Clone, Debug)]
pub struct Ticker {
    pub symbol: String,
    pub last: f64,
    pub change_pct: f64,
}