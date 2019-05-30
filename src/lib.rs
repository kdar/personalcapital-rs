#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::error::Error;

use cookie_store::CookieStore;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{
  self,
  header::{self, HeaderMap},
};
use serde_json::Value;
use url::percent_encoding::{percent_encode, USERINFO_ENCODE_SET};

const BASE_URL: &str = "https://home.personalcapital.com";
const IDENTIFY_USER: &str = "/api/login/identifyUser";
const CHALLENGE_SMS: &str = "/api/credential/challengeSms";
const AUTHENTICATE_SMS: &str = "/api/credential/authenticateSmsByCode";
const CHALLENGE_EMAIL: &str = "/api/credential/challengeEmail";
const AUTHENTICATE_EMAIL: &str = "/api/credential/authenticateEmailByCode";
const AUTHENTICATE_PASSWORD: &str = "/api/credential/authenticatePassword";
const USER_TRANSACTIONS: &str = "/api/transaction/getUserTransactions";

lazy_static! {
  static ref CSRF_RE: Regex = Regex::new(r"globals.csrf='([a-f0-9-]+)'").unwrap();
}

pub trait TwoFactor {
  fn get_code(&self) -> String;
}

#[derive(Clone, Default)]
struct DefaultTwoFactor;

impl TwoFactor for DefaultTwoFactor {
  fn get_code(&self) -> String {
    use std::io::{stdin, stdout, Write};
    let mut code = String::new();
    print!("Code: ");
    let _ = stdout().flush();
    stdin()
      .read_line(&mut code)
      .expect("did not enter a correct string");

    code.trim().to_string()
  }
}

#[derive(Debug, PartialEq)]
enum AuthLevel {
  Null, // initial state
  Csrf, // fake auth level signifying we got the csrf token

  // Personal Capital auth levels:
  UserRemembered,
  UserIdentified,
  DeviceAuthorized,
  SessionAuthenticated,
  None,
}

impl From<&str> for AuthLevel {
  fn from(s: &str) -> Self {
    // These are all the auth levels actually returned by
    // Personal Capital.
    match s {
      "USER_REMEMBERED" => AuthLevel::UserRemembered,
      "USER_IDENTIFIED" => AuthLevel::UserIdentified,
      "DEVICE_AUTHORIZED" => AuthLevel::DeviceAuthorized,
      "SESSION_AUTHENTICATED" => AuthLevel::SessionAuthenticated,
      "NONE" => AuthLevel::None,
      _ => panic!("unknown auth level: {:?}", s),
    }
  }
}

pub struct ClientBuilder {
  two_factor: Box<TwoFactor>,
  username: Option<String>,
  password: Option<String>,
  device_name: Option<String>,
}

impl ClientBuilder {
  pub fn new() -> Self {
    ClientBuilder {
      two_factor: Box::new(DefaultTwoFactor),
      username: None,
      password: None,
      device_name: None,
    }
  }

  pub fn two_factor(&mut self, value: Box<TwoFactor>) -> &mut Self {
    self.two_factor = value;
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

  pub fn build(&mut self) -> Result<Client, Box<Error>> {
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
    let mut tf: Box<TwoFactor> = Box::new(DefaultTwoFactor);
    ::std::mem::swap(&mut self.two_factor, &mut tf);

    Ok(Client {
      client,
      csrf: String::new(),
      auth_level: AuthLevel::Null,
      cookie_store: CookieStore::default(),
      two_factor: tf,
      username: self.username.take().unwrap(),
      password: self.password.take().unwrap(),
      device_name: self.device_name.take().unwrap(),
    })
  }
}

pub struct Client {
  client: reqwest::Client,
  csrf: String,
  auth_level: AuthLevel,
  cookie_store: CookieStore,
  two_factor: Box<TwoFactor>,
  username: String,
  password: String,
  device_name: String,
}

impl Client {
  fn store_cookies(
    &mut self,
    url: reqwest::Url,
    headers: &reqwest::header::HeaderMap,
  ) -> Result<(), Box<Error>> {
    for hv in headers.get_all("set-cookie").iter() {
      if let Ok(s) = hv.to_str() {
        self.cookie_store.parse(s, &url)?;
      }
    }

    Ok(())
  }

  fn add_cookie_header(&self, headers: &mut reqwest::header::HeaderMap) {
    let header = self
      .cookie_store
      // .get_request_cookies(url)
      .iter_unexpired()
      .map(|c| {
        let name = percent_encode(c.name().as_bytes(), USERINFO_ENCODE_SET);
        let value = percent_encode(c.value().as_bytes(), USERINFO_ENCODE_SET);
        format!("{}={}", name, value)
      })
      .collect::<Vec<_>>()
      .join("; ");

    headers.insert(
      reqwest::header::COOKIE,
      reqwest::header::HeaderValue::from_bytes(header.as_bytes()).unwrap(),
    );
  }

  fn request(&mut self, mut req: reqwest::Request) -> Result<reqwest::Response, Box<Error>> {
    self.add_cookie_header(req.headers_mut());
    let url = req.url().clone();
    let res = self.client.execute(req)?;

    if let Err(e) = res.error_for_status_ref() {
      return Err(e.into());
    }

    self.store_cookies(url, &res.headers())?;
    Ok(res)
  }

  fn request_json(&mut self, req: reqwest::Request) -> Result<Value, Box<Error>> {
    let mut res = self.request(req)?;
    let json: Value = res.json()?;

    if let Some(csrf) = json["spHeader"]["csrf"].as_str() {
      self.csrf = csrf.into();
    }

    if let Some(auth_level) = json["spHeader"]["authLevel"].as_str() {
      self.auth_level = auth_level.into();
    }

    if let Some(errors) = json["spHeader"]["errors"].as_array() {
      let mut msg = String::new();
      msg.push_str(&errors[0]["message"].to_string());
      if let Some(details) = errors[0].get("details") {
        msg.push_str(" ");
        msg.push_str(&details.to_string());
      }
      return Err(msg.into());
    }

    Ok(json)
  }

  fn get_csrf(&mut self) -> Result<(), Box<Error>> {
    let req = self.client.get(BASE_URL).build()?;
    let mut res = self.request(req)?;
    let body = res.text()?;

    if let Some(captures) = CSRF_RE.captures(&body) {
      if let Some(csrf) = captures.get(1) {
        self.csrf = csrf.as_str().into();
        self.auth_level = AuthLevel::Csrf;
        return Ok(());
      }
    }

    Err("unable to get CSRF token".into())
  }

  fn identify_user(&mut self) -> Result<(), Box<Error>> {
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
    let json = self.request_json(req)?;

    if json["spData"]["userStatus"] == "INACTIVE" {
      return Err(format!("the username \"{}\" is inactive", self.username).into());
    }

    Ok(())
  }

  fn two_factor_auth(&mut self) -> Result<(), Box<Error>> {
    if self.auth_level == AuthLevel::UserRemembered {
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

    let mut params = HashMap::new();
    params.insert("csrf", self.csrf.clone());
    params.insert("bindDevice", "false".into());
    params.insert("challengeReason", "DEVICE_AUTH".into());
    params.insert("challengeMethod", "OP".into());
    params.insert("challengeType", auth_type.into());

    let req = self.client.post(&challenge_url).form(&params).build()?;
    self.request_json(req)?;

    let code = self.two_factor.get_code();

    let mut params = HashMap::new();
    params.insert("csrf", self.csrf.clone());
    params.insert("bindDevice", "false".into());
    params.insert("challengeReason", "DEVICE_AUTH".into());
    params.insert("challengeMethod", "OP".into());
    params.insert("code", code.into());

    let req = self.client.post(&auth_url).form(&params).build()?;
    self.request_json(req)?;

    return Ok(());
  }

  fn auth_password(&mut self) -> Result<(), Box<Error>> {
    let url = format!("{}{}", BASE_URL, AUTHENTICATE_PASSWORD);

    let mut params = HashMap::new();
    params.insert("csrf", self.csrf.clone());
    params.insert("bindDevice", "true".into());
    params.insert("skipLinkAccount", "false".into());
    params.insert("passwd", self.password.clone());
    params.insert("deviceName", self.device_name.clone());
    params.insert("apiClient", "WEB".into());

    let req = self.client.post(&url).form(&params).build()?;
    self.request_json(req)?;

    Ok(())
  }

  pub fn auth(&mut self) -> Result<(), Box<Error>> {
    if self.auth_level == AuthLevel::Null || self.csrf.is_empty() {
      self.get_csrf()?;
    }

    self.identify_user()?;

    if self.auth_level == AuthLevel::UserIdentified || self.auth_level == AuthLevel::UserRemembered
    {
      self.two_factor_auth()?;
    }

    if self.auth_level == AuthLevel::DeviceAuthorized {
      self.auth_password()?;
    }

    match self.auth_level {
      AuthLevel::SessionAuthenticated => Ok(()),
      AuthLevel::None => Err("could not auth".into()),
      _ => Err(
        format!(
          "unknown auth level state at end of auth(): {:?}",
          self.auth_level
        )
        .into(),
      ),
    }
  }

  pub fn user_transactions(&mut self) -> Result<Value, Box<Error>> {
    let url = format!("{}{}", BASE_URL, USER_TRANSACTIONS);

    let mut params = HashMap::new();
    params.insert("csrf", self.csrf.clone());
    params.insert("apiClient", "WEB".into());
    params.insert("startDate", "2019-05-01".into());
    params.insert("endDate", "2019-05-28".into());
    params.insert("lastServerChangeId", "-1".into());

    let req = self.client.post(&url).form(&params).build()?;
    let json = self.request_json(req)?;

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
