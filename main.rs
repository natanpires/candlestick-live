use cli_candlestick_chart::{Candle, Chart};
use forex_candlestick::ws;
use std::collections::HashMap;
use std::env;

fn main() {
    // Receive pid from args
    let args: Vec<String> = env::args().collect();
    let pair_id = &args[1];
    let mut candles = vec![];
    let mut titles = HashMap::new();

    // This can be refactored to use a configuration file.
    titles.insert(String::from("1617"), "EUR/BRL");
    titles.insert(String::from("2103"), "USD/BRL");
    titles.insert(String::from("945629"), "USD/BTC");
    titles.insert(String::from("1024807"), "BRL/BTC");
    titles.insert(String::from("8830"), "GOLD");
    titles.insert(String::from("8833"), "BRENT OIL");
    titles.insert(String::from("17920"), "IBOV");

    let title = titles.get(&String::from(pair_id)).copied().unwrap_or("N/A");

    let stream = ws::Stream::new(pair_id.to_string(), move |candle| {
        let mut to_append = vec![Candle::new(
            candle.last.parse::<f64>().unwrap(),
            candle.high.parse::<f64>().unwrap(),
            candle.low.parse::<f64>().unwrap(),
            candle.last_close.parse::<f64>().unwrap(),
            Some(candle.turnover_numeric.parse::<f64>().unwrap()),
            Some(candle.timestamp as i64),
        )];

        // Append data to our chart
        candles.append(&mut to_append);

        // Clean CLI so it'll properly display the chart with the past and received information.
        print!("\x1B[2J\x1B[1;1H");

        let mut chart = Chart::new(&mut candles);

        // Set the chart title
        chart.set_name(String::from(title));

        // Set customs colors
        chart.set_bear_color(1, 205, 254);
        chart.set_bull_color(255, 107, 153);
        chart.set_vol_bull_color(1, 205, 254);
        chart.set_vol_bear_color(255, 107, 153);

        chart.set_volume_pane_height(6);
        chart.set_volume_pane_enabled(false);

        chart.draw();

        Ok({})
    })
    .expect("Failed to create stream");

    println!("main: stream.pair_id={}", stream.pair_id);
    println!("main: stream.handler={:?}", stream.stream_handle_spawn);

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(stream.stream_handle_spawn)
        .unwrap()
        .unwrap();
}
