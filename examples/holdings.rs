// use std::fs;

// use personalcapital::types;
// use serde_json;

// fn main() {
//   let json = fs::read_to_string("./.secret/getHoldings.json").unwrap();
//   let v: types::Response = serde_json::from_str(&json).unwrap();
//   let v: types::Holdings = serde_json::from_str(v.sp_data.get()).unwrap();

//   let mut holdings: Vec<types::Holding> = v
//     .holdings
//     .into_iter()
//     .filter(
//       |x| /*x.source != types::HoldingSource::User && x.value >= 100.0 && */x.ticker.is_some(),
//     )
//     .collect();
//   holdings.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap());
//   let total: f64 = holdings.iter().map(|x| x.value).sum();

//   // println!("total: {}", total);
//   let mut hp = 0.0;
//   println!("Ticker,Dollar Amount");
//   for h in holdings {
//     println!(
//       "{},{}",
//       h.ticker.unwrap(),
//       // h.quantity,
//       h.value,
//       // h.value / total * 100.0,
//     );
//   }

//   // for h in v.holdings {
//   //   if h.source == types::HoldingSource::User {
//   //     continue;
//   //   }

//   //   if h.value < 100.0 {
//   //     continue;
//   //   }

//   //   if let Some(ticker) = h.ticker {
//   //     total += h.value;
//   //     println!("{} - {}", ticker, h.value);
//   //   }
//   // }
// }
