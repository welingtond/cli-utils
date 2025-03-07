//! This is a library for stock price search

#[derive(Debug)]
/// TimeKey is a struct that holds the date and time of a stock price
pub struct TimeKey {
    date: String,
    time: String,
}

impl TimeKey {
    /// Create a new TimeKey
    pub fn new(date: &str, time: &str) -> TimeKey {
        TimeKey {
            date: date.to_string(),
            time: time.to_string(),
        }
    }
    
}
#[derive(Debug)]
/// StockPrice is a struct that holds the stock price data
pub struct StockPrice {
    timestamp: TimeKey,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl StockPrice {
    /// Create a new StockPrice
    pub fn new(data_line: String) -> StockPrice {
        let binding = data_line.replace(" ", ",") ;
        let binding: Vec<&str> = binding.split(",").collect();
        StockPrice {
            timestamp: TimeKey::new(binding[0], binding[1]),
            open: binding[2].parse().unwrap(),
            high: binding[3].parse().unwrap(),
            low: binding[4].parse().unwrap(),
            close: binding[5].parse().unwrap(),
            volume: binding[6].parse().unwrap(),
        }
    }
}

/// search_stock_price searches for the stock price of a given stock
pub fn search_stock_price(stock_name: &str/* , api_key: &str {Not by now...}*/) -> f64 {
    // Call the API to get the stock price
    // For now, just return a hardcoded value
    let input_line: String = "2025-03-06 19:00:00,105.4300,106.0000,105.4000,105.4200,1009".to_string();

    let stock_price = StockPrice::new(input_line);
    stock_price.close
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = search_stock_price("AAPL");
        assert_eq!(result, 105.4200);
    }
}
