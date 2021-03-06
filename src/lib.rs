extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::{collections::HashMap, error::Error as StdError, io::Write};

use async_trait::async_trait;
use cookie_store::CookieStore;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{
  self,
  header::{self, HeaderMap},
};
use thiserror::Error;

mod serde_util;
pub mod types;

const BASE_URL: &str = "https://home.personalcapital.com";
const IDENTIFY_USER: &str = "/api/login/identifyUser";
const QUERY_SESSION: &str = "/api/login/querySession";
// const CHALLENGE_SMS: &str = "/api/credential/challengeSms";
// const AUTHENTICATE_SMS: &str = "/api/credential/authenticateSmsByCode";
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

pub type SyncError = Box<dyn StdError + Send + Sync>;

#[derive(Error, Debug)]
pub enum Error {
  #[error("two factor required")]
  TwoFactorRequired,
  #[error("awaiting two factor code")]
  AwaitingTwoFactorCode,
  #[error("login failed")]
  LoginFailed,
  #[error("call login() first")]
  CallLogin,
  #[error("session is invalid")]
  SessionInvalid,
  #[error("username not set")]
  UsernameNotSet,
  #[error("password not set")]
  PasswordNotSet,
  #[error("device name not set")]
  DeviceNameNotSet,
  #[error("unable to get CSRF token")]
  CrsfToken,
  #[error("username {0} is inactive")]
  InactiveUser(String),
  #[error("reqwest error")]
  Reqwest(#[from] reqwest::Error),
  #[error("cookie store error")]
  CookieStore(#[from] cookie_store::CookieError),
  #[error("personal capital error: {0}")]
  PersonalCapital(String),
  #[error("serde_json error")]
  SerdeJson(#[from] serde_json::error::Error),
  #[error("serde_json error: {0}; around `{}`", {
    let v = .1.lines().nth(.0.line()-1).unwrap();
    let start = (.0.column()-1) - 100;
    // if start < 0 {
    //   start = 0;
    // }
    let mut end = (.0.column()-1) + 100;
    if end >= v.len() {
      end = v.len()-1;
    }

    &v[start..end]
  })]
  SerdeJsonContext(serde_json::error::Error, String),
  #[error(transparent)]
  Other(#[from] SyncError),
}

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
  type Error = SyncError;

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

pub struct ClientBuilder {
  store: Box<dyn Store<Error = SyncError>>,
  username: Option<String>,
  password: Option<String>,
  device_name: Option<String>,
}

impl ClientBuilder {
  pub fn new() -> Self {
    ClientBuilder {
      store: Box::new(DefaultStore),
      username: None,
      password: None,
      device_name: None,
    }
  }

  pub fn store(&mut self, value: Box<dyn Store<Error = SyncError>>) -> &mut Self {
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
      return Err(Error::UsernameNotSet);
    }

    if self.password.is_none() {
      return Err(Error::PasswordNotSet);
    }

    if self.device_name.is_none() {
      return Err(Error::DeviceNameNotSet);
    }

    let mut h = HeaderMap::new();
    h.insert(header::ACCEPT, "*/*".parse().unwrap());
    h.insert(
      header::USER_AGENT,
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:80.0) Gecko/20100101 Firefox/80.0"
        .parse()
        .unwrap(),
    );
    h.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
    h.insert(header::ACCEPT_LANGUAGE, "en-US,en;q=0.5 ".parse().unwrap());
    // h.insert("authority", "home.personalcapital.com".parse().unwrap());
    h.insert(header::ORIGIN, BASE_URL.parse().unwrap());

    // let p = reqwest::redirect::Policy::custom(|attempt| {
    //   if attempt.previous().len() > 5 {
    //     attempt.error("too many redirects")
    //   } else if attempt.url().host_str() == Some("home.personalcapital.com") {
    //     // This will happen when you try to authenticate with a previous session
    // cookie     // and the session has expired. Normally you think it'd return
    // JSON to tell you     // it's expired but Personal Capital decides to take
    // a POST request and turn it     // into a homepage redirect.
    //     attempt.error(Error::SessionExpired)
    //   } else {
    //     attempt.follow()
    //   }
    // });
    let client = reqwest::Client::builder()
      .default_headers(h)
      .connection_verbose(true)
      // .redirect(p)
      .build()?;

    // Is there a better way to do this?
    let mut store: Box<dyn Store<Error = SyncError>> = Box::new(DefaultStore);
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
      store,
      username: self.username.take().unwrap(),
      password: self.password.take().unwrap(),
      device_name: self.device_name.take().unwrap(),
      last_server_change_id: -1,
    })
  }
}

pub struct Client {
  client: reqwest::Client,
  csrf: String,
  auth_level: types::AuthLevel,
  cookie_store: CookieStore,
  store: Box<dyn Store<Error = SyncError>>,
  username: String,
  password: String,
  device_name: String,
  last_server_change_id: i64,
}

impl Client {
  async fn store_cookies(
    &mut self,
    url: reqwest::Url,
    headers: &reqwest::header::HeaderMap,
  ) -> Result<(), Error> {
    for hv in headers.get_all(header::SET_COOKIE).iter() {
      if let Ok(s) = hv.to_str() {
        // Don't set CloudFlare cookies, since they expire the second you retrieve them
        // and they cause the cookie store to throw an error.
        if s.contains("__cfduid") || s.contains("__cflb") {
          continue;
        }

        self.cookie_store.parse(s, &url)?;
      }
    }

    let mut buf = vec![];
    // We can't use save_json() here because the cookie store will not save
    // non-persistent cookies. We want to persist all cookies so that we can
    // continue grabbing data after a restart without having to login again.
    // self.cookie_store.save_json(&mut buf)?;
    for cookie in self.cookie_store.iter_any() {
      writeln!(&mut buf, "{}", serde_json::to_string(&cookie)?).unwrap();
    }
    self.store.save_cookies(buf).await?;

    Ok(())
  }

  fn add_cookie_header(&self, headers: &mut reqwest::header::HeaderMap) {
    let header = self
      .cookie_store
      .iter_unexpired()
      .map(|c| {
        // let name = percent_encode(c.name().as_bytes(),
        // percent_encoding::NON_ALPHANUMERIC); let value =
        // percent_encode(c.value().as_bytes(), percent_encoding::NON_ALPHANUMERIC);
        format!("{}={}", c.name(), c.value())
      })
      .collect::<Vec<_>>()
      .join("; ");

    headers.insert(
      reqwest::header::COOKIE,
      reqwest::header::HeaderValue::from_bytes(header.as_bytes()).unwrap(),
    );
  }

  async fn request(&mut self, mut req: reqwest::Request) -> Result<reqwest::Response, Error> {
    // println!("\x1b[0;92m{:?} - {:?}\x1b[0;0m", req, self.auth_level);
    // if let Some(b) = req.body() {
    //   println!("{:?}", String::from_utf8_lossy(b.as_bytes().unwrap()));
    // }
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
    let res = match self.request(req).await {
      Ok(v) => v,
      // Err(Error::Reqwest(e)) => {
      //   if let Some(v) = e.source() {
      //     if let Some(Error::SessionExpired) = v.downcast_ref() {
      //       return Err(Error::SessionExpired);
      //     }
      //   }
      //   return Err(Error::Reqwest(e));
      // }
      Err(e) => {
        return Err(e);
      },
    };

    let text = res.text().await?;
    // println!("\x1b[0;34m{}\x1b[0;0m", text);
    let json: types::Response = serde_json::from_str(&text)?;

    if let Some(csrf) = json.sp_header.csrf {
      self.csrf = csrf.clone();
    }

    // We just logged out.
    if self.auth_level == types::AuthLevel::SessionAuthenticated
      && json.sp_header.auth_level != types::AuthLevel::SessionAuthenticated
    {
      return Err(Error::SessionInvalid);
    }

    self.auth_level = json.sp_header.auth_level;

    // if let Some(changes) = json.sp_header.sp_data_changes {
    //   for change in changes {
    //     if change.server_change_id > self.last_server_change_id {
    //       self.last_server_change_id = change.server_change_id;
    //     }
    //   }
    // }

    // if json.sp_header.sp_header_version > self.last_server_change_id {
    //   self.last_server_change_id = json.sp_header.sp_header_version;
    // }

    if let Some(errors) = json.sp_header.errors {
      if errors[0].code == 202 {
        return Err(Error::SessionInvalid);
      }

      let mut msg = String::new();
      msg.push_str(&errors[0].message);
      if let Some(details) = &errors[0].details {
        msg.push_str(" ");
        msg.push_str(&serde_json::to_string(&details).unwrap());
      }

      return Err(Error::PersonalCapital(msg.into()));
    }

    let payload = json.sp_data.get();
    serde_json::from_str(payload).map_err(|e| Error::SerdeJsonContext(e, payload.to_string()))
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

    Err(Error::CrsfToken)
  }

  async fn identify_user(&mut self) -> Result<(), Error> {
    let url = format!("{}{}", BASE_URL, IDENTIFY_USER);

    let mut params = HashMap::new();
    params.insert("csrf", self.csrf.clone());
    params.insert("bindDevice", "false".into());
    params.insert("skipLinkAccount", "true".into());
    params.insert("apiClient", "WEB".into());
    params.insert("username", self.username.clone());
    params.insert("redirectTo", String::new());
    params.insert("skipFirstUse", String::new());
    params.insert("referrerId", String::new());

    let req = self.client.post(&url).form(&params).build()?;
    let json: types::IdentifyUser = self.request_json(req).await?;

    if json.user_status == types::Status::Inactive {
      return Err(Error::InactiveUser(self.username.clone()));
    }

    Ok(())
  }

  pub async fn two_factor_challenge(&mut self) -> Result<(), Error> {
    if self.auth_level == types::AuthLevel::UserRemembered {
      return Ok(());
    }

    if self.auth_level != types::AuthLevel::UserIdentified {
      return Err(Error::CallLogin);
    }

    let challenge_url = format!("{}{}", BASE_URL, CHALLENGE_EMAIL);

    let mut params = HashMap::new();
    params.insert("csrf", self.csrf.clone());
    params.insert("bindDevice", "false".into());
    params.insert("challengeReason", "DEVICE_AUTH".into());
    params.insert("challengeMethod", "OP".into());
    params.insert("apiClient", "WEB".into());

    let req = self.client.post(&challenge_url).form(&params).build()?;
    self.request_json(req).await?;

    Ok(())
  }

  pub async fn two_factor_auth(&mut self, code: &str) -> Result<(), Error> {
    let auth_url = format!("{}{}", BASE_URL, AUTHENTICATE_EMAIL);

    if self.auth_level != types::AuthLevel::UserIdentified {
      return Err(Error::CallLogin);
    }

    let mut params = HashMap::new();
    params.insert("csrf", self.csrf.clone());
    params.insert("bindDevice", "false".into());
    params.insert("challengeReason", "DEVICE_AUTH".into());
    params.insert("challengeMethod", "OP".into());
    params.insert("code", code.into());
    params.insert("apiClient", "WEB".into());

    let req = self.client.post(&auth_url).form(&params).build()?;
    match self.request_json(req).await {
      Ok(()) => {
        // TODO: possibly set state here
      },
      Err(e) => {
        // TODO: possibly set state here
        return Err(e);
      },
    };

    Ok(())
  }

  pub async fn auth_password(&mut self) -> Result<(), Error> {
    if self.auth_level != types::AuthLevel::UserRemembered
      && self.auth_level != types::AuthLevel::DeviceAuthorized
    {
      return Err(Error::TwoFactorRequired);
    }

    let url = format!("{}{}", BASE_URL, AUTHENTICATE_PASSWORD);

    let mut params = HashMap::new();
    params.insert("csrf", self.csrf.clone());
    params.insert("bindDevice", "true".into());
    params.insert("skipLinkAccount", "false".into());
    params.insert("passwd", self.password.clone());
    params.insert("deviceName", self.device_name.clone());
    params.insert("apiClient", "WEB".into());
    params.insert("redirectTo", String::new());
    params.insert("skipFirstUse", String::new());
    params.insert("referrerId", String::new());
    params.insert("username", self.username.clone().into());

    let req = self.client.post(&url).form(&params).build()?;
    self
      .request_json::<types::AuthenticatePassword>(req)
      .await?;

    match self.auth_level {
      types::AuthLevel::SessionAuthenticated | types::AuthLevel::UserRemembered => Ok(()),
      types::AuthLevel::UserIdentified => Err(Error::AwaitingTwoFactorCode),
      types::AuthLevel::None => Err(Error::LoginFailed),
      _ => {
        Err(Error::Other(
          format!(
            "unknown auth level state at end of auth(): {:?}",
            self.auth_level
          )
          .into(),
        ))
      },
    }
  }

  pub async fn login(&mut self) -> Result<(), Error> {
    if self.auth_level == types::AuthLevel::SessionAuthenticated {
      return Ok(());
    }

    if self.auth_level == types::AuthLevel::Null || self.csrf.is_empty() {
      self.get_csrf().await?;
    }

    self.identify_user().await?;
    self.auth_password().await?;

    Ok(())
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
    params.insert(
      "lastServerChangeId",
      format!("{}", self.last_server_change_id),
    );

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
      (
        "lastServerChangeId",
        format!("{}", self.last_server_change_id),
      ),
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
      (
        "lastServerChangeId",
        format!("{}", self.last_server_change_id),
      ),
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
      (
        "lastServerChangeId",
        format!("{}", self.last_server_change_id),
      ),
    ];

    let req = self.client.post(&url).form(&params).build()?;
    let json = self.request_json(req).await?;

    Ok(json)
  }

  pub async fn query_session(&mut self) -> Result<types::QuerySession, Error> {
    let url = format!("{}{}", BASE_URL, QUERY_SESSION);

    let params = vec![
      ("csrf", self.csrf.clone()),
      ("apiClient", "WEB".into()),
      (
        "lastServerChangeId",
        format!("{}", self.last_server_change_id),
      ),
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
