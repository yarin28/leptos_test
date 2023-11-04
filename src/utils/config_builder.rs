use std::io::prelude::*;
use std::{collections::HashMap, fs::File};
// use anyhow::Result;
use config::{Config, FileStoredFormat, Format, Map, ValueKind};
use rlua::{Context, FromLua, Function, Lua, MetaMethod, RegistryKey, Result, Table, UserData};
// use rlua_serde;
use rlua_table_derive::FromLuaTable;
use serde::{de, Deserialize, Serialize};
use tracing::event;

pub fn config_build() {
    let mut config_file_content: String = String::new();
    std::fs::File::open("config.lua")
        .unwrap()
        .read_to_string(&mut config_file_content)
        .unwrap();
    let config = Config::builder()
        .add_source(config::File::from_str(&config_file_content, MyFormat))
        .build();

    match config {
        Ok(cfg) => println!("A config: {:#?}", cfg),
        Err(e) => println!("An error: {}", e),
    }
}
#[derive(Serialize, Default, Clone, FromLuaTable, Debug)]
pub struct GpioConfig {
    name: String,
    gpio_pin: usize,
    gpio_type: String,
    active_seconds: usize,
    cron_string: String,
}

// #[derive(Serialize, Default, Clone, Deserialize, Debug)]
// pub struct ConfigTable {
//     table: HashMap<config::Value, config::Value>,
// }
trait FromLuaTable {
    fn from_lua_table(table: &rlua::Table) -> Self;
}

#[derive(Debug, Clone)]
pub struct MyFormat;

impl Format for MyFormat {
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> std::result::Result<Map<String, config::Value>, Box<dyn std::error::Error + Send + Sync>>
    {
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
impl FileStoredFormat for MyFormat {
    fn file_extensions(&self) -> &'static [&'static str] {
        &MY_FORMAT_EXT
    }
}

fn lua_to_config_value(lua_value: rlua::Value) -> Result<config::Value> {
    Ok(match lua_value {
        rlua::Value::Table(table) => {
            let pairs = table.pairs();
            let map = pairs
                .map(|pair| {
                    let (key, value) = pair?;
                    let key = match key {
                        rlua::Value::String(name) => name.to_str()?.to_string(),
                        rlua::Value::Integer(i) => i.to_string(),
                        _ => return Err(todo!()),
                    };
                    Ok((key, lua_to_config_value(value)?))
                })
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .collect::<HashMap<_, _>>();
            config::Value::new(None, map)
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
