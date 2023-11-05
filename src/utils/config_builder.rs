use anyhow::bail;
use anyhow::Result;
use config::{Config, FileStoredFormat, Format, Map};
use rlua::TablePairs;
use std::collections::HashMap;
use std::io::prelude::*;

pub fn config_build() -> anyhow::Result<Config, config::ConfigError> {
    let mut config_file_content: String = String::new();
    std::fs::File::open("config.lua")
        .unwrap()
        .read_to_string(&mut config_file_content)
        .unwrap();
    let config = Config::builder()
        .add_source(config::File::from_str(&config_file_content, LuaTable))
        .build();

    match &config {
        Ok(cfg) => {
            println!("A config: {:#?}", cfg);
        }
        Err(e) => println!("An error: {}", e),
    }
    config
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
    // fn lua_to_config_value(lua_value: rlua::Value) -> Result<config::Value, rlua::Error> {
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
#[test]
fn name() {
    unimplemented!();
}
