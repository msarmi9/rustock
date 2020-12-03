#[macro_use(load_yaml)]
extern crate clap;

use chrono::{DateTime, Utc};
use clap::App;
use yahoo_finance_api as yahoo;


fn min(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        None
    } else {
        Some(series.iter().fold(f64::MAX, |a, &b| a.min(b)))
    }
}


fn max(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        None
    } else {
        Some(series.iter().fold(f64::MIN, |a, &b| a.max(b)))
    }
}


fn percent_change(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        None
    } else {
        let (first, last) = (series.first().unwrap(), series.last().unwrap());
        let first = if *first == 0. { 1. } else { *first };
        Some(100. * (last - first) / first)
    }
}


fn moving_average(series: &[f64], n: usize) -> Option<Vec<f64>> {
    if series.is_empty() || n == 0 {
        None
    } else {
        Some(series.windows(n).map(|w| w.iter().sum::<f64>() / (w.len() as f64)).collect())
    }
}


fn main() {
    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml).get_matches();

    // Safe to unwrap as these args are required
    let symbol = args.value_of("symbol").unwrap();
    let from = args.value_of("from").unwrap();

    let start = from.parse::<DateTime<Utc>>().expect("Could not parse --from date");
    let end = Utc::now();

    let provider = yahoo::YahooConnector::new();
    if let Ok(response) = provider.get_quote_history(symbol, start, end) {
        if let Ok(quotes) = response.quotes() {
            let adjcloses = quotes.iter().map(|q| q.adjclose).collect::<Vec<_>>();
            let price = adjcloses.last().unwrap_or(&0.);
            let min = min(&adjcloses).unwrap_or_default();
            let max = max(&adjcloses).unwrap_or_default();
            let change = percent_change(&adjcloses).unwrap_or_default();
            let avgs = moving_average(&adjcloses, 30).unwrap_or_default();
            let avg = avgs.last().unwrap_or(&0.);

            println!("period start,symbol,price,change %,min,max,30d avg");
            println!("{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}", from, symbol, price, change, min, max, avg);
        } else {
            eprintln!("No quotes found for symbol: {}", symbol);
        }
    } else {
        eprintln!("Could not get response from yahoo finance api.");
    }
}
 
