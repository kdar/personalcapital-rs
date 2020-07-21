use std::env;
use std::error::Error;

use personalcapital;

struct Store;
impl personalcapital::Store for Store {
  type Error = Box<dyn Error>;
  fn save_csrf(&mut self, csrf: String) -> Result<(), Self::Error> {
    Ok(())
  }
  fn save_cookies(&mut self, cookies: Vec<u8>) -> Result<(), Self::Error> {
    Ok(())
  }
  fn load_csrf(&mut self) -> Result<Option<String>, Self::Error> {
    Ok(None)
  }
  fn load_cookies(&mut self) -> Result<Option<Vec<u8>>, Self::Error> {
    Ok(None)
  }
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  let store = Store;
  let mut client = personalcapital::ClientBuilder::new()
    .username(env::var("PC_USERNAME")?)
    .password(env::var("PC_PASSWORD")?)
    .store(store)
    .device_name(env::var("PC_DEVICE_NAME")?)
    .build()?;
  client.auth()?;
  println!("{:#?}", client.accounts()?);

  Ok(())
}
