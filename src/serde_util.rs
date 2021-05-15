use serde::{
  de::{Deserializer, Error, IntoDeserializer, Visitor},
  Deserialize,
};
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

// pub(crate) fn deserialize_milli_ts_option<'de, D: Deserializer<'de>>(
//   deserializer: D,
// ) -> Result<Option<NaiveDateTime>, D::Error> {
//   ts_milliseconds_option::deserialize(deserializer).map(|o| o.map(|d| d.naive_utc()))
// }

pub(crate) fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
  D: serde::Deserializer<'de>,
  T: serde::Deserialize<'de>,
{
  let opt = Option::<String>::deserialize(de)?;
  let opt = opt.as_ref().map(String::as_str);
  match opt {
    None | Some("") => Ok(None),
    Some(s) => T::deserialize(s.into_deserializer()).map(Some),
  }
}
