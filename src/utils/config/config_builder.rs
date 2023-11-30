use crate::utils::config::config_types::*;
use anyhow::bail;
use anyhow::Result;
use config::{Config, FileStoredFormat, Format, Map, ValueKind};
use rlua::TablePairs;
use std::collections::HashMap;
use std::io::prelude::*;

use lazy_static::lazy_static;
use std::sync::RwLock;
lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(config_build().unwrap());
}
pub fn config_build() -> anyhow::Result<Config, config::ConfigError> {
    let mut config_file_content: String = String::new();
    std::fs::File::open("config.lua")
        .unwrap()
        .read_to_string(&mut config_file_content)
        .unwrap();
    Config::builder()
        .add_source(config::File::from_str(&config_file_content, LuaTable))
        .build()
}

#[derive(Debug, Clone)]
pub struct LuaTable;

impl Format for LuaTable {
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, config::Value>, Box<dyn std::error::Error + Send + Sync>> {
        let mut result = HashMap::new();
        let lua = rlua::Lua::new();
        lua.context(|lua_ctx| {
            let config_lua_table = lua_ctx.load(text).eval().unwrap();
            result.insert(
                uri.unwrap_or(&"lua".to_string()).to_string(),
                lua_to_config_value(config_lua_table).unwrap(),
            );
        });
        Ok(result)
    }
}

// As strange as it seems for config sourced from a string, legacy demands its sacrifice
// It is only required for File source, custom sources can use Format without caring for extensions
static MY_FORMAT_EXT: Vec<&'static str> = vec![];
impl FileStoredFormat for LuaTable {
    fn file_extensions(&self) -> &'static [&'static str] {
        &MY_FORMAT_EXT
    }
}

fn lua_to_config_value(lua_value: rlua::Value) -> Result<config::Value> {
    let uri = "lua".to_string();
    Ok(match lua_value {
        rlua::Value::Table(table) => {
            let pairs: TablePairs<rlua::Value, rlua::Value> = table.pairs();
            let map = pairs
                .map(|pair| {
                    let (key, value) = pair?;
                    let key = match key {
                        rlua::Value::String(name) => name.to_str()?.to_string(),
                        rlua::Value::Integer(i) => i.to_string(),
                        _ => {
                            {
                                tracing::event!(tracing::Level::ERROR,"error with the config table, please check the syntax of the lua config table");
                                     bail!(rlua::Error::FromLuaConversionError { from: value.type_name(), to: value.type_name(), message: Some( "bad syntax".to_string() ) })
                            }
                        },
                    };
                    Ok((key, lua_to_config_value(value)?))
                })
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .collect::<HashMap<_, _>>();
            config::Value::new(Some(&uri), map)
        }
        rlua::Value::Nil => config::Value::new(None, config::ValueKind::Nil),
        rlua::Value::Boolean(val) => config::Value::new(None, val),
        rlua::Value::Integer(val) => config::Value::new(None, val),
        rlua::Value::Number(val) => config::Value::new(None, val),
        rlua::Value::String(val) => config::Value::new(None, val.to_str()?.to_string()),
        rlua::Value::LightUserData(_) => todo!(),
        rlua::Value::Function(_) => todo!(),
        rlua::Value::Thread(_) => todo!(),
        rlua::Value::UserData(_) => todo!(),
        rlua::Value::Error(_) => todo!(),
    })
}
// i want to pass the config struct to the client without installing the config crate on the
// client, so i have created a common struct that they can both use
pub fn config_value_to_public_config_value(origin: config::Value) -> Result<config_types::Value> {
    Ok(match origin.kind {
        config::ValueKind::Table(table) => {
            let value = table
                .into_iter()
                .map(|pair| {
                    let (key, value) = pair;
                    Ok((key, config_value_to_public_config_value(value)?))
                })
                .collect::<Result<Vec<(String, config_types::Value)>>>()?
                .into_iter()
                .collect::<HashMap<_, _>>();
            config_types::Value {
                origin: None,
                kind: value.into(),
            }
        }
        ValueKind::Nil => config_types::Value {
            origin: None,
            kind: config_types::ValueKind::Nil,
        },
        ValueKind::Boolean(val) => config_types::Value {
            origin: None,
            kind: config_types::ValueKind::Boolean(val),
        },

        ValueKind::I64(val) => config_types::Value {
            origin: None,
            kind: config_types::ValueKind::I64(val),
        },
        ValueKind::I128(val) => config_types::Value {
            origin: None,
            kind: config_types::ValueKind::I128(val),
        },

        ValueKind::U64(val) => config_types::Value {
            origin: None,
            kind: config_types::ValueKind::U64(val),
        },

        ValueKind::U128(val) => config_types::Value {
            origin: None,
            kind: config_types::ValueKind::U128(val),
        },

        ValueKind::Float(val) => config_types::Value {
            origin: None,
            kind: config_types::ValueKind::Float(val),
        },

        ValueKind::String(val) => config_types::Value {
            origin: None,
            kind: config_types::ValueKind::String(val),
        },

        ValueKind::Array(val) => {
            tracing::event!(
                tracing::Level::ERROR,
                "still dident implemeted the array option, please use table"
            );
            config_types::Value {
                origin: None,
                kind: config_types::ValueKind::Nil,
            }
        }
    })
}

pub fn get_value_from_settings_object_to_client(query_string: &str) -> Result<config_types::Value> {
    config_value_to_public_config_value(
        SETTINGS
            .read()
            .unwrap()
            .get_table(query_string)
            .unwrap()
            .into(),
    )
}

use config::Value;

use super::config_types;
#[test]
fn test_config_values_correctness() {
    let mut config = config_build();
    let mut map = config
        .as_mut()
        .unwrap()
        .clone()
        .get_table("lua.gpio_table")
        .unwrap();
    assert!(config
        .as_mut()
        .unwrap()
        .get_string("lua.seconds_to_pump_water")
        .is_ok());
    assert!(map.keys().count() != 0);
    assert!(!map
        .iter_mut()
        .map(|gpio_pin_hash_map| {
            let (_key, value) = gpio_pin_hash_map;
            value.clone().into_table().unwrap().get("name").is_some()
                && value
                    .clone()
                    .into_table()
                    .unwrap()
                    .get("gpio_pin")
                    .is_some()
                && value
                    .clone()
                    .into_table()
                    .unwrap()
                    .get("gpio_type")
                    .is_some()
                && value
                    .clone()
                    .into_table()
                    .unwrap()
                    .get("active_seconds")
                    .is_some()
                && value
                    .clone()
                    .into_table()
                    .unwrap()
                    .get("cron_string")
                    .is_some()
        })
        .collect::<Vec<bool>>()
        .contains(&false));
    dbg!(&map);
}