

pub struct Ticker{
    symbol: String,
    open_time:i64,
    open_price:String,
    last_price:String,
    close_time:i64,
}

// example response json
//[
//   {
//     "symbol": "BTCUSDT",
//     "priceChange": "193.60000000",
//     "priceChangePercent": "0.190",
//     "weightedAvgPrice": "101903.06972077",
//     "openPrice": "101810.58000000",
//     "highPrice": "102019.11000000",
//     "lowPrice": "101810.58000000",
//     "lastPrice": "102004.18000000",
//     "volume": "29.26609000",
//     "quoteVolume": "2982304.40972440",
//     "openTime": 1747318680000,
//     "closeTime": 1747318751094,
//     "firstId": 4910724105,
//     "lastId": 4910730264,
//     "count": 6160
//   },
//   {
//     "symbol": "BNBUSDT",
//     "priceChange": "0.85000000",
//     "priceChangePercent": "0.131",
//     "weightedAvgPrice": "647.04823221",
//     "openPrice": "646.76000000",
//     "highPrice": "647.72000000",
//     "lowPrice": "646.60000000",
//     "lastPrice": "647.61000000",
//     "volume": "197.75500000",
//     "quoteVolume": "127957.02316000",
//     "openTime": 1747318680000,
//     "closeTime": 1747318751094,
//     "firstId": 1056079497,
//     "lastId": 1056080605,
//     "count": 1109
//   }
// ]