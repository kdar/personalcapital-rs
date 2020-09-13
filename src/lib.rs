extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::{collections::HashMap, error::Error as StdError};

use async_trait::async_trait;
use cookie_store::CookieStore;
use lazy_static::lazy_static;
use percent_encoding::percent_encode;
use regex::Regex;
use reqwest::{
  self,
  header::{self, HeaderMap},
};

pub mod types;

const BASE_URL: &str = "https://home.personalcapital.com";
const IDENTIFY_USER: &str = "/api/login/identifyUser";
const CHALLENGE_SMS: &str = "/api/credential/challengeSms";
const AUTHENTICATE_SMS: &str = "/api/credential/authenticateSmsByCode";
const CHALLENGE_EMAIL: &str = "/api/credential/challengeEmail";
const AUTHENTICATE_EMAIL: &str = "/api/credential/authenticateEmailByCode";
const AUTHENTICATE_PASSWORD: &str = "/api/credential/authenticatePassword";
const USER_TRANSACTIONS: &str = "/api/transaction/getUserTransactions";
const USER_SPENDING: &str = "/api/account/getUserSpending";
const ACCOUNTS: &str = "/api/newaccount/getAccounts2";
const CATEGORIES: &str = "/api/transactioncategory/getCategories";

lazy_static! {
  static ref CSRF_RE: Regex = Regex::new(r"globals.csrf='([a-f0-9-]+)'").unwrap();
}

pub type Error = Box<dyn StdError + Send + Sync>;

#[async_trait]
pub trait TwoFactor: Send {
  async fn get_code(&mut self) -> Option<String>;
  async fn should_challenge(&mut self) -> bool;
  async fn set_status(&mut self, _success: bool) {}
}

// impl<T: TwoFactor> TwoFactor for Arc<Mutex<T>> {
//   fn get_code(&mut self) -> Option<String> {
//     let mut l = self.lock().unwrap();
//     l.get_code()
//   }

//   fn should_challenge(&mut self) -> bool {
//     let mut l = self.lock().unwrap();
//     l.should_challenge()
//   }

//   fn set_status(&mut self, success: bool) {
//     let mut l = self.lock().unwrap();
//     l.set_status(success)
//   }
// }

#[async_trait]
pub trait Store {
  type Error;
  async fn save_csrf(&mut self, csrf: String) -> Result<(), Self::Error>;
  async fn save_cookies(&mut self, cookies: Vec<u8>) -> Result<(), Self::Error>;
  async fn load_csrf(&mut self) -> Result<Option<String>, Self::Error>;
  async fn load_cookies(&mut self) -> Result<Option<Vec<u8>>, Self::Error>;
}

#[derive(Clone, Default)]
struct DefaultStore;

#[async_trait]
impl Store for DefaultStore {
  type Error = Error;

  async fn save_csrf(&mut self, _csrf: String) -> Result<(), Self::Error> {
    Ok(())
  }

  async fn save_cookies(&mut self, _cookies: Vec<u8>) -> Result<(), Self::Error> {
    Ok(())
  }

  async fn load_csrf(&mut self) -> Result<Option<String>, Self::Error> {
    Ok(None)
  }

  async fn load_cookies(&mut self) -> Result<Option<Vec<u8>>, Self::Error> {
    Ok(None)
  }
}

#[derive(Clone, Default)]
struct DefaultTwoFactor;

#[async_trait]
impl TwoFactor for DefaultTwoFactor {
  async fn should_challenge(&mut self) -> bool {
    return true;
  }

  async fn get_code(&mut self) -> Option<String> {
    use std::io::{stdin, stdout, Write};
    let mut code = String::new();
    print!("Code: ");
    let _ = stdout().flush();
    stdin()
      .read_line(&mut code)
      .expect("did not enter a correct string");

    let code = code.trim().to_string();

    if code.len() == 4 {
      Some(code)
    } else {
      None
    }
  }
}

pub struct ClientBuilder {
  two_factor: Box<dyn TwoFactor>,
  store: Box<dyn Store<Error = Error>>,
  username: Option<String>,
  password: Option<String>,
  device_name: Option<String>,
}

impl ClientBuilder {
  pub fn new() -> Self {
    ClientBuilder {
      two_factor: Box::new(DefaultTwoFactor),
      store: Box::new(DefaultStore),
      username: None,
      password: None,
      device_name: None,
    }
  }

  pub fn two_factor(&mut self, value: Box<dyn TwoFactor>) -> &mut Self {
    self.two_factor = value;
    self
  }

  pub fn store(&mut self, value: Box<dyn Store<Error = Error>>) -> &mut Self {
    self.store = value;
    self
  }

  pub fn username<V: Into<String>>(&mut self, value: V) -> &mut Self {
    self.username = Some(value.into());
    self
  }

  pub fn password<V: Into<String>>(&mut self, value: V) -> &mut Self {
    self.password = Some(value.into());
    self
  }

  pub fn device_name<V: Into<String>>(&mut self, value: V) -> &mut Self {
    self.device_name = Some(value.into());
    self
  }

  pub async fn build(&mut self) -> Result<Client, Error> {
    if self.username.is_none() {
      return Err("username must be set".into());
    }

    if self.password.is_none() {
      return Err("password must be set".into());
    }

    if self.device_name.is_none() {
      return Err("device_name must be set".into());
    }

    let mut h = HeaderMap::new();
    h.insert(header::ACCEPT, "*/*".parse()?);
    h.insert(header::USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/71.0.3578.98 Safari/537.36".parse()?);
    h.insert("adrum", "isAjax:true".parse()?);
    h.insert(header::ACCEPT_LANGUAGE, "en-US,en;q=0.9".parse()?);
    h.insert("authority", "home.personalcapital.com".parse()?);
    h.insert(header::ORIGIN, "https://home.personalcapital.com".parse()?);

    let client = reqwest::Client::builder().default_headers(h).build()?;

    // Is there a better way to do this?
    let mut tf: Box<dyn TwoFactor> = Box::new(DefaultTwoFactor);
    ::std::mem::swap(&mut self.two_factor, &mut tf);
    let mut store: Box<dyn Store<Error = Error>> = Box::new(DefaultStore);
    ::std::mem::swap(&mut self.store, &mut store);

    let cookie_store = if let Some(cookies) = store.load_cookies().await? {
      CookieStore::load_json(&cookies[..])?
    } else {
      CookieStore::default()
    };

    Ok(Client {
      client,
      csrf: String::new(),
      auth_level: types::AuthLevel::Null,
      cookie_store,
      two_factor: tf,
      store,
      username: self.username.take().unwrap(),
      password: self.password.take().unwrap(),
      device_name: self.device_name.take().unwrap(),
    })
  }
}

pub struct Client {
  client: reqwest::Client,
  csrf: String,
  auth_level: types::AuthLevel,
  cookie_store: CookieStore,
  two_factor: Box<dyn TwoFactor>,
  store: Box<dyn Store<Error = Error>>,
  username: String,
  password: String,
  device_name: String,
}

impl Client {
  async fn store_cookies(
    &mut self,
    url: reqwest::Url,
    headers: &reqwest::header::HeaderMap,
  ) -> Result<(), Error> {
    for hv in headers.get_all("set-cookie").iter() {
      if let Ok(s) = hv.to_str() {
        self.cookie_store.parse(s, &url)?;
      }
    }

    let mut buf = vec![];
    self.cookie_store.save_json(&mut buf)?;
    self.store.save_cookies(buf).await?;

    Ok(())
  }

  fn add_cookie_header(&self, headers: &mut reqwest::header::HeaderMap) {
    let header = self
      .cookie_store
      // .get_request_cookies(url)
      .iter_unexpired()
      .map(|c| {
        let name = percent_encode(c.name().as_bytes(), percent_encoding::NON_ALPHANUMERIC);
        let value = percent_encode(c.value().as_bytes(), percent_encoding::NON_ALPHANUMERIC);
        format!("{}={}", name, value)
      })
      .collect::<Vec<_>>()
      .join("; ");

    headers.insert(
      reqwest::header::COOKIE,
      reqwest::header::HeaderValue::from_bytes(header.as_bytes()).unwrap(),
    );
  }

  async fn request(&mut self, mut req: reqwest::Request) -> Result<reqwest::Response, Error> {
    self.add_cookie_header(req.headers_mut());
    let url = req.url().clone();
    let res = self.client.execute(req).await?;

    if let Err(e) = res.error_for_status_ref() {
      return Err(e.into());
    }

    self.store_cookies(url, &res.headers()).await?;
    Ok(res)
  }

  async fn request_json<T>(&mut self, req: reqwest::Request) -> Result<T, Error>
  where
    T: serde::de::DeserializeOwned,
  {
    let res = self.request(req).await?;
    let json: types::Response = res.json().await?;

    if let Some(csrf) = json.sp_header.csrf {
      self.csrf = csrf.clone();
    }

    self.auth_level = json.sp_header.auth_level;

    if let Some(errors) = json.sp_header.errors {
      let mut msg = String::new();
      msg.push_str(&errors[0].message);
      if let Some(details) = &errors[0].details {
        msg.push_str(" ");
        msg.push_str(&serde_json::to_string(&details)?);
      }
      return Err(msg.into());
    }

    let payload = json.sp_data.get();
    serde_json::from_str(payload).map_err(|e| format!("{} -> {}", e, payload).into())
  }

  async fn get_csrf(&mut self) -> Result<(), Error> {
    if let Some(csrf) = self.store.load_csrf().await? {
      self.csrf = csrf;
      return Ok(());
    }

    let req = self.client.get(BASE_URL).build()?;
    let res = self.request(req).await?;
    let body: String = res.text().await?;

    if let Some(captures) = CSRF_RE.captures(&body) {
      if let Some(csrf) = captures.get(1) {
        self.csrf = csrf.as_str().into();
        self.auth_level = types::AuthLevel::Csrf;
        self.store.save_csrf(self.csrf.clone()).await?;
        return Ok(());
      }
    }

    Err("unable to get CSRF token".into())
  }

  async fn identify_user(&mut self) -> Result<(), Error> {
    let url = format!("{}{}", BASE_URL, IDENTIFY_USER);

    let mut params = HashMap::new();
    params.insert("csrf", self.csrf.clone());
    params.insert("bindDevice", "false".into());
    params.insert("skipLinkAccount", "false".into());
    params.insert("apiClient", "WEB".into());
    params.insert("username", self.username.clone());
    params.insert("redirectTo", String::new());
    params.insert("skipFirstUse", String::new());
    params.insert("referrerId", String::new());

    let req = self.client.post(&url).form(&params).build()?;
    let json: types::IdentifyUser = self.request_json(req).await?;

    if json.user_status == types::Status::Inactive {
      return Err(format!("the username \"{}\" is inactive", self.username).into());
    }

    Ok(())
  }

  async fn two_factor_auth(&mut self) -> Result<(), Error> {
    if self.auth_level == types::AuthLevel::UserRemembered {
      return Ok(());
    }

    let (challenge_url, auth_url, auth_type) = if true {
      (
        format!("{}{}", BASE_URL, CHALLENGE_EMAIL),
        format!("{}{}", BASE_URL, AUTHENTICATE_EMAIL),
        "2",
      )
    } else {
      (
        format!("{}{}", BASE_URL, CHALLENGE_SMS),
        format!("{}{}", BASE_URL, AUTHENTICATE_SMS),
        "0",
      )
    };

    if self.two_factor.should_challenge().await {
      let mut params = HashMap::new();
      params.insert("csrf", self.csrf.clone());
      params.insert("bindDevice", "false".into());
      params.insert("challengeReason", "DEVICE_AUTH".into());
      params.insert("challengeMethod", "OP".into());
      params.insert("challengeType", auth_type.into());

      let req = self.client.post(&challenge_url).form(&params).build()?;
      self.request_json(req).await?;
    }

    if let Some(code) = self.two_factor.get_code().await {
      let mut params = HashMap::new();
      params.insert("csrf", self.csrf.clone());
      params.insert("bindDevice", "false".into());
      params.insert("challengeReason", "DEVICE_AUTH".into());
      params.insert("challengeMethod", "OP".into());
      params.insert("code", code.into());

      let req = self.client.post(&auth_url).form(&params).build()?;
      match self.request_json(req).await {
        Ok(()) => {
          self.two_factor.set_status(true).await;
        }
        Err(e) => {
          self.two_factor.set_status(false).await;
          return Err(e);
        }
      };
    }

    return Ok(());
  }

  async fn auth_password(&mut self) -> Result<(), Error> {
    let url = format!("{}{}", BASE_URL, AUTHENTICATE_PASSWORD);

    let mut params = HashMap::new();
    params.insert("csrf", self.csrf.clone());
    params.insert("bindDevice", "true".into());
    params.insert("skipLinkAccount", "false".into());
    params.insert("passwd", self.password.clone());
    params.insert("deviceName", self.device_name.clone());
    params.insert("apiClient", "WEB".into());

    let req = self.client.post(&url).form(&params).build()?;
    self
      .request_json::<types::AuthenticatePassword>(req)
      .await?;

    Ok(())
  }

  pub async fn auth(&mut self) -> Result<(), Error> {
    if self.auth_level == types::AuthLevel::SessionAuthenticated {
      return Ok(());
    }

    if self.auth_level == types::AuthLevel::Null || self.csrf.is_empty() {
      self.get_csrf().await?;
    }

    self.identify_user().await?;

    if self.auth_level == types::AuthLevel::UserIdentified {
      self.two_factor_auth().await?;
    }

    if self.auth_level == types::AuthLevel::DeviceAuthorized
      || self.auth_level == types::AuthLevel::UserRemembered
    {
      self.auth_password().await?;
    }

    match self.auth_level {
      types::AuthLevel::SessionAuthenticated => Ok(()),
      types::AuthLevel::UserIdentified => Err("awaiting challenge code".into()),
      types::AuthLevel::None => Err("could not auth".into()),
      _ => Err(
        format!(
          "unknown auth level state at end of auth(): {:?}",
          self.auth_level
        )
        .into(),
      ),
    }
  }

  pub async fn user_transactions<S: Into<String>>(
    &mut self,
    start_date: S,
    end_date: S,
  ) -> Result<types::UserTransactions, Error> {
    let url = format!("{}{}", BASE_URL, USER_TRANSACTIONS);

    let mut params = HashMap::new();
    params.insert("csrf", self.csrf.clone());
    params.insert("apiClient", "WEB".into());
    params.insert("startDate", start_date.into());
    params.insert("endDate", end_date.into());
    params.insert("lastServerChangeId", "-1".into());

    let req = self.client.post(&url).form(&params).build()?;
    let json = self.request_json(req).await?;

    Ok(json)
  }

  pub async fn user_spending(&mut self) -> Result<types::UserSpending, Error> {
    let url = format!("{}{}", BASE_URL, USER_SPENDING);

    let params = vec![
      ("csrf", self.csrf.clone()),
      ("apiClient", "WEB".into()),
      ("intervalTypes[]", "MONTH".into()),
      ("intervalTypes[]", "WEEK".into()),
      ("intervalTypes[]", "YEAR".into()),
      ("includeDetails", "true".into()),
      ("includeValues[]", "CURRENT".into()),
      ("includeValues[]", "TARGET".into()),
      ("lastServerChangeId", "-1".into()),
    ];

    let req = self.client.post(&url).form(&params).build()?;
    let json = self.request_json(req).await?;

    Ok(json)
  }

  pub async fn accounts(&mut self) -> Result<types::Accounts, Error> {
    let url = format!("{}{}", BASE_URL, ACCOUNTS);

    let params = vec![
      ("csrf", self.csrf.clone()),
      ("apiClient", "WEB".into()),
      ("lastServerChangeId", "-1".into()),
    ];

    let req = self.client.post(&url).form(&params).build()?;
    let json = self.request_json(req).await?;

    Ok(json)
  }

  pub async fn categories(&mut self) -> Result<types::Categories, Error> {
    let url = format!("{}{}", BASE_URL, CATEGORIES);

    let params = vec![
      ("csrf", self.csrf.clone()),
      ("apiClient", "WEB".into()),
      ("lastServerChangeId", "-1".into()),
    ];

    let req = self.client.post(&url).form(&params).build()?;
    let json = self.request_json(req).await?;

    Ok(json)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    env_logger::init();
    let mut c = ClientBuilder::new().build().unwrap();
    c.auth().unwrap();
  }
}
