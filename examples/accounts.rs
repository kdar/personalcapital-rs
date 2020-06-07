use std::env;
use std::error::Error;

use personalcapital;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  let mut client = personalcapital::ClientBuilder::new()
    .username(env::var("PC_USERNAME")?)
    .password(env::var("PC_PASSWORD")?)
    .device_name(env::var("PC_DEVICE_NAME")?)
    .build()?;
  client.auth()?;
  println!("{:#?}", client.accounts()?);

  Ok(())
}
