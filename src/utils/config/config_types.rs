use crate::utils::config::error::ConfigError;
use crate::utils::config::error::Unexpected;
use anyhow::Result;
use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Value {
    pub origin: Option<String>,

    pub kind: ValueKind,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ValueKind {
    Nil,
    Boolean(bool),
    I64(i64),
    I128(i128),
    U64(u64),
    U128(u128),
    Float(f64),
    String(String),
    Table(Map<String, Value>),
    Array(Array),
}

pub type Array = Vec<Value>;
pub type Table = Map<String, Value>;
pub type Map<K, V> = HashMap<K, V>;

impl<T> From<Map<String, T>> for ValueKind
where
    T: Into<Value>,
{
    fn from(values: Map<String, T>) -> Self {
        let t = values.into_iter().map(|(k, v)| (k, v.into())).collect();
        Self::Table(t)
    }
}

impl<T> From<Vec<T>> for ValueKind
where
    T: Into<Value>,
{
    fn from(values: Vec<T>) -> Self {
        Self::Array(values.into_iter().map(T::into).collect())
    }
}

impl Default for ValueKind {
    fn default() -> Self {
        Self::Nil
    }
}
impl Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::String(ref value) => write!(f, "{}", value),
            Self::Boolean(value) => write!(f, "{}", value),
            Self::I64(value) => write!(f, "{}", value),
            Self::I128(value) => write!(f, "{}", value),
            Self::U64(value) => write!(f, "{}", value),
            Self::U128(value) => write!(f, "{}", value),
            Self::Float(value) => write!(f, "{}", value),
            Self::Nil => write!(f, "nil"),
            Self::Table(ref table) => write!(f, "{{ {} }}", {
                table
                    .iter()
                    .map(|(k, v)| format!("{} => {}, ", k, v))
                    .collect::<String>()
            }),
            Self::Array(ref array) => write!(f, "{:?}", {
                array.iter().map(|e| format!("{}, ", e)).collect::<String>()
            }),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl<T> From<T> for Value
where
    T: Into<ValueKind>,
{
    fn from(value: T) -> Self {
        Self {
            origin: None,
            kind: value.into(),
        }
    }
}

impl Value {
    /// If the `Value` is a Table, returns the associated Map.
    // FIXME: Should this not be `try_into_*` ?
    pub fn into_table(self) -> Result<Map<String, Self>> {
        match self.kind {
            ValueKind::Table(value) => Ok(value),

            // Cannot convert
            ValueKind::Float(value) => {
                Err(
                    ConfigError::invalid_type(self.origin, Unexpected::Float(value), "a map")
                        .into(),
                )
            }
            ValueKind::String(value) => {
                Err(ConfigError::invalid_type(self.origin, Unexpected::Str(value), "a map").into())
            }
            ValueKind::I64(value) => {
                Err(ConfigError::invalid_type(self.origin, Unexpected::I64(value), "a map").into())
            }
            ValueKind::I128(value) => {
                Err(ConfigError::invalid_type(self.origin, Unexpected::I128(value), "a map").into())
            }
            ValueKind::U64(value) => {
                Err(ConfigError::invalid_type(self.origin, Unexpected::U64(value), "a map").into())
            }
            ValueKind::U128(value) => {
                Err(ConfigError::invalid_type(self.origin, Unexpected::U128(value), "a map").into())
            }
            ValueKind::Boolean(value) => {
                Err(ConfigError::invalid_type(self.origin, Unexpected::Bool(value), "a map").into())
            }
            ValueKind::Nil => {
                Err(ConfigError::invalid_type(self.origin, Unexpected::Unit, "a map").into())
            }
            ValueKind::Array(_) => {
                Err(ConfigError::invalid_type(self.origin, Unexpected::Seq, "a map").into())
            }
        }
    }

    pub fn into_int(self) -> std::result::Result<i64> {
        match self.kind {
            ValueKind::I64(value) => Ok(value),
            ValueKind::I128(value) => value.try_into().map_err(|_| {
                ConfigError::invalid_type(
                    self.origin,
                    Unexpected::I128(value),
                    "an signed 64 bit or less integer",
                )
            }),
            ValueKind::U64(value) => value.try_into().map_err(|_| {
                ConfigError::invalid_type(
                    self.origin,
                    Unexpected::U64(value),
                    "an signed 64 bit or less integer",
                )
            }),
            ValueKind::U128(value) => value.try_into().map_err(|_| {
                ConfigError::invalid_type(
                    self.origin,
                    Unexpected::U128(value),
                    "an signed 64 bit or less integer",
                )
            }),

            ValueKind::String(ref s) => {
                match s.to_lowercase().as_ref() {
                    "true" | "on" | "yes" => Ok(1),
                    "false" | "off" | "no" => Ok(0),
                    _ => {
                        s.parse().map_err(|_| {
                            // Unexpected string
                            ConfigError::invalid_type(
                                self.origin.clone(),
                                Unexpected::Str(s.clone()),
                                "an integer",
                            )
                        })
                    }
                }
            }

            ValueKind::Boolean(value) => Ok(if value { 1 } else { 0 }),
            ValueKind::Float(value) => Ok(value.round() as i64),

            // Unexpected type
            ValueKind::Nil => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Unit,
                "an integer",
            )),
            ValueKind::Table(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Map,
                "an integer",
            )),
            ValueKind::Array(_) => Err(ConfigError::invalid_type(
                self.origin,
                Unexpected::Seq,
                "an integer",
            )),
        }
    }
}
