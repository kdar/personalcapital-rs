use std::fs;

use personalcapital::types;
use serde_json;

fn main() {
  let json = fs::read_to_string("./.secret/getAccounts2.json").unwrap();
  let v: types::Response = serde_json::from_str(&json).unwrap();
  let v: types::Accounts = serde_json::from_str(v.sp_data.get()).unwrap();

  let mut taxable: f64 = 0.0;
  let mut tax_deferred: f64 = 0.0;
  let mut tax_free: f64 = 0.0;

  for account in v.accounts {
    if account.product_type != types::ProductType::Investment || account.is_exclude_from_household {
      continue;
    }

    match (account.account_type_new, account.account_type_subtype) {
      (types::AccountTypeNew::Investment, types::AccountTypeSubtype::None) => {
        taxable += account.balance
      }
      (types::AccountTypeNew::Ira, types::AccountTypeSubtype::Roth) => tax_free += account.balance,
      (types::AccountTypeNew::Ira, types::AccountTypeSubtype::Traditional) => {
        tax_deferred += account.balance
      }
      (types::AccountTypeNew::A401k, types::AccountTypeSubtype::Traditional) => {
        tax_deferred += account.balance
      }
      (types::AccountTypeNew::A401k, types::AccountTypeSubtype::Roth) => {
        tax_free += account.balance
      }
      (_, _) => {}
    };
    // println!(
    //   "{}; {:?}; {:?}; {:?}; {:?}",
    //   account.name,
    //   account.account_type_group,
    //   account.account_type,
    //   account.account_type_new,
    //   account.account_type_subtype
    // );
  }

  println!("Taxable: {}", taxable);
  println!("Tax Deferred: {}", tax_deferred);
  println!("Tax Free: {}", tax_free);
  println!("Total: {}", taxable + tax_deferred + tax_free);
}
