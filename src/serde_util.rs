use serde::de::{Deserializer, Error, Visitor};
use std::fmt::{Formatter, Result as FResult};

struct F64;

impl<'de> Visitor<'de> for F64 {
  type Value = f64;

  fn expecting(&self, f: &mut Formatter) -> FResult {
    f.write_str("f64 as a number or string")
  }

  fn visit_f64<E: Error>(self, id: f64) -> Result<Self::Value, E> {
    Ok(id)
  }

  fn visit_str<E: Error>(self, s: &str) -> Result<Self::Value, E> {
    s.parse().map_err(Error::custom)
  }
}

pub(crate) fn deserialize_f64_option<'de, D: Deserializer<'de>>(
  deserializer: D,
) -> Result<Option<f64>, D::Error> {
  deserializer.deserialize_any(F64).map(Some).or(Ok(None))
}
