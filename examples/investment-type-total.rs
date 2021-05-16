use std::fs;

use personalcapital::pc_types;
use serde_json;

fn main() {
  let json = fs::read_to_string("./.secret/getAccounts2.json").unwrap();
  let v: pc_types::Response = serde_json::from_str(&json).unwrap();
  let v: pc_types::Accounts = serde_json::from_str(v.sp_data.get()).unwrap();

  let mut taxable: f64 = 0.0;
  let mut tax_deferred: f64 = 0.0;
  let mut tax_free: f64 = 0.0;

  for account in v.accounts {
    if account.product_type != pc_types::ProductType::Investment
      || account.is_exclude_from_household
    {
      continue;
    }

    match (account.account_type_new, account.account_type_subtype) {
      (pc_types::AccountTypeNew::Investment, pc_types::AccountTypeSubtype::None) => {
        taxable += account.balance.unwrap()
      },
      (pc_types::AccountTypeNew::Ira, pc_types::AccountTypeSubtype::Roth) => {
        tax_free += account.balance.unwrap()
      },
      (pc_types::AccountTypeNew::Ira, pc_types::AccountTypeSubtype::Traditional) => {
        tax_deferred += account.balance.unwrap()
      },
      (pc_types::AccountTypeNew::A401k, pc_types::AccountTypeSubtype::Traditional) => {
        tax_deferred += account.balance.unwrap()
      },
      (pc_types::AccountTypeNew::A401k, pc_types::AccountTypeSubtype::Roth) => {
        tax_free += account.balance.unwrap()
      },
      (..) => {},
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
