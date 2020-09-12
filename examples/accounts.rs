use std::env;
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use async_trait::async_trait;
use personalcapital;

struct Store;

#[async_trait]
impl personalcapital::Store for Store {
  type Error = Box<dyn Error + Send + Sync + 'static>;

  async fn save_csrf(&mut self, csrf: String) -> Result<(), Self::Error> {
    let mut path = env::temp_dir();
    path.push("__pc_csrf");
    let mut f = fs::File::create(path)?;
    f.write_all(csrf.as_bytes())?;
    Ok(())
  }

  async fn save_cookies(&mut self, cookies: Vec<u8>) -> Result<(), Self::Error> {
    let mut path = env::temp_dir();
    path.push("__pc_cookies");
    let mut f = fs::File::create(path)?;
    f.write_all(&cookies)?;
    Ok(())
  }

  async fn load_csrf(&mut self) -> Result<Option<String>, Self::Error> {
    let mut path = env::temp_dir();
    path.push("__pc_csrf");
    if !path.is_file() {
      return Ok(None);
    }

    let mut f = fs::File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    if contents.is_empty() {
      Ok(None)
    } else {
      Ok(Some(contents))
    }
  }

  async fn load_cookies(&mut self) -> Result<Option<Vec<u8>>, Self::Error> {
    let mut path = env::temp_dir();
    path.push("__pc_cookies");
    if !path.is_file() {
      return Ok(None);
    }

    let mut f = fs::File::open(path)?;
    let mut contents = vec![];
    f.read_to_end(&mut contents)?;

    if contents.is_empty() {
      Ok(None)
    } else {
      Ok(Some(contents))
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  let store = Box::new(Store);
  let mut client = personalcapital::ClientBuilder::new()
    .username(env::var("PC_USERNAME")?)
    .password(env::var("PC_PASSWORD")?)
    .store(store)
    .device_name(env::var("PC_DEVICE_NAME")?)
    .build()
    .await?;
  client.auth().await?;
  println!("{:#?}", client.accounts().await?);

  Ok(())
}
