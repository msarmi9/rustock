#[macro_use(load_yaml)]
extern crate clap;

use chrono::{DateTime, Utc};
use clap::App;
use yahoo_finance_api as yahoo;


fn min(series: &[f64]) -> f64 {
    series.iter().fold(f64::MAX, |x, &y| if x <= y { x } else { y })
}


fn max(series: &[f64]) -> f64 {
    series.iter().fold(f64::MIN, |x, &y| if x >= y { x } else { y })
}


fn percent_change(series: &[f64]) -> f64 {
    let (first, last) = (series.first().unwrap(), series.last().unwrap());
    100. * (last - first) / first
}


fn moving_average(series: &[f64], n: usize) -> Vec<f64> {
    let mut avgs = Vec::new();
    for w in series.windows(n) {
        let sum: f64 = Iterator::sum(w.iter());
        let avg = sum / (w.len() as f64);
        avgs.push(avg);
    }
    avgs
}


fn main() {
    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml).get_matches();
    let symbol = args.value_of("symbol").unwrap();
    let date = args.value_of("date").unwrap();

    let start = date.parse::<DateTime<Utc>>().unwrap();
    let end = Utc::now();

    let provider = yahoo::YahooConnector::new();
    let response = provider.get_quote_history(symbol, start, end).unwrap();
    let quotes = response.quotes().unwrap();

    let adjcloses = quotes.iter().map(|x| x.adjclose).collect::<Vec<_>>();
    let price = adjcloses.last().unwrap();
    let min = min(&adjcloses);
    let max = max(&adjcloses);
    let change = percent_change(&adjcloses);

    let avgs = moving_average(&adjcloses, 30);
    let avg = avgs.last().unwrap();

    println!("period start,symbol,price,change %,min,max,30d avg");
    println!("{},{},${:.2},{:.2}%,${:.2},${:.2},{:.2}", date, symbol, price, change, min, max, avg);
}
