use std::cmp::Ordering;

use crate::pc_types;

fn transaction_cmp_status(a: &pc_types::Transaction, b: &pc_types::Transaction) -> Ordering {
  let i = a.status == pc_types::TransactionStatus::Pending;
  let j = b.status == pc_types::TransactionStatus::Pending;
  if i && !j {
    Ordering::Less
  } else if !i && j {
    Ordering::Greater
  } else {
    Ordering::Equal
  }
}

fn transaction_cmp_date(a: &pc_types::Transaction, b: &pc_types::Transaction) -> Ordering {
  b.transaction_date.partial_cmp(&a.transaction_date).unwrap()
}

fn transaction_cmp_id(a: &pc_types::Transaction, b: &pc_types::Transaction) -> Ordering {
  b.user_transaction_id
    .partial_cmp(&a.user_transaction_id)
    .unwrap()
}

fn transaction_cmp_account_name(a: &pc_types::Transaction, b: &pc_types::Transaction) -> Ordering {
  let a = a.account_name.to_uppercase();
  let b = b.account_name.to_uppercase();

  a.partial_cmp(&b).unwrap()
}

fn transaction_cmp_amount(a: &pc_types::Transaction, b: &pc_types::Transaction) -> Ordering {
  let a = if a.is_credit { a.amount } else { -a.amount };
  let b = if b.is_credit { b.amount } else { -b.amount };

  a.partial_cmp(&b).unwrap()
}

fn transaction_cmp_description(a: &pc_types::Transaction, b: &pc_types::Transaction) -> Ordering {
  let a = a.description.to_uppercase();
  let b = b.description.to_uppercase();

  a.partial_cmp(&b).unwrap()
}

fn transaction_cmp_price(a: &pc_types::Transaction, b: &pc_types::Transaction) -> Ordering {
  a.price.partial_cmp(&b.price).unwrap()
}

fn transaction_cmp_quantity(a: &pc_types::Transaction, b: &pc_types::Transaction) -> Ordering {
  a.quantity.partial_cmp(&b.quantity).unwrap()
}

// TODO
// fn transaction_cmp_tags(a: &pc_types::Transaction, b: &pc_types::Transaction) -> Ordering {
// const n = r.a.get(e, "customTags.systemTags", [])
//           , i = r.a.get(a, "customTags.systemTags", [])
//           , o = r.a.get(e, "customTags.userTags", [])
//           , d = r.a.get(a, "customTags.userTags", []);
//         let u, h;
//         return 1 === n.length && 1 === i.length && Object(l.isEmpty)(o) && Object(l.isEmpty)(d) && (u = n[0] - 100001,
//         h = i[0] - 100001),
//         n.length + o.length - (i.length + d.length) || u - h || s()(e.transactionDate, c.a) - s()(a.transactionDate, c.a)
//}

// This will sort transactions exactly the way the UI does it.
pub fn transaction_cmp(a: &pc_types::Transaction, b: &pc_types::Transaction) -> Ordering {
  let comparisons = [
    transaction_cmp_status,
    transaction_cmp_date,
    transaction_cmp_id,
    transaction_cmp_account_name,
    transaction_cmp_amount,
    transaction_cmp_description,
    transaction_cmp_price,
    transaction_cmp_quantity,
    // transaction_cmp_tags,
  ];
  for cmp in comparisons {
    let o = cmp(a, b);
    if o != Ordering::Equal {
      return o;
    }
  }

  Ordering::Equal
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use std::{fs::File, io::BufReader};

  use super::*;

  #[test]
  fn test_transaction_compare() {
    let file_input = match File::open("./testdata/sort_test_input.json") {
      Ok(it) => it,
      _ => return,
    };
    let reader_input = BufReader::new(file_input);
    let mut transactions_input: Vec<pc_types::Transaction> =
      serde_json::from_reader(reader_input).unwrap();

    let file_output = match File::open("./testdata/sort_test_output.json") {
      Ok(it) => it,
      _ => return,
    };
    let reader_output = BufReader::new(file_output);
    let transactions_output: Vec<pc_types::Transaction> =
      serde_json::from_reader(reader_output).unwrap();

    transactions_input.sort_by(transaction_cmp);

    let fmt = |v: Vec<pc_types::Transaction>| -> Vec<String> {
      v.iter()
        .map(|v| {
          format!(
            "{} - {} - {} - {}",
            v.transaction_date, v.account_name, v.description, v.amount
          )
        })
        .collect()
    };

    let v_input = fmt(transactions_input);
    let v_output = fmt(transactions_output);

    // println!("{}\n{}", v_input[0], v_input[1]);
    // println!("{}\n{}", v_output[0], v_output[1]);

    assert_eq!(v_input, v_output);
  }
}
