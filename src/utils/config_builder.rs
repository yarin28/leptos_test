use std::io::prelude::*;
use std::{collections::HashMap, fs::File};
// use anyhow::Result;
use config::{Config, FileStoredFormat, Format, Map, Value, ValueKind};
use rlua::{Context, Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};
use rlua_table_derive::FromLuaTable;
use serde::{de, Serialize};
use tracing::event;

pub fn config_build() {
    let mut config_file_content: String = String::new();
    std::fs::File::open("config.lua")
        .unwrap()
        .read_to_string(&mut config_file_content)
        .unwrap();
    println!("this is the config text - {config_file_content}");
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

trait FromLuaTable {
    fn from_lua_table(table: &rlua::Table) -> Self;
}

#[derive(Debug, Clone)]
pub struct MyFormat;

fn get_lua_table_with_struct<T: Default, FromLuaTable>(
    table_name_in_lua: &str,
    lua_ctx: Context,
) -> T {
    trait FromLuaTable {
        fn from_lua_table(table: &rlua::Table) -> Self;
    }
    let lua = rlua::Lua::new();
    let mut result = T::default();
    let table = match lua_ctx.globals().get(table_name_in_lua) {
        Ok(table) => {
            event!(
                tracing::Level::DEBUG,
                "got the gpio_table variable from lua and the value is ->{:?}",
                &table
            );
            T::from_lua_table(&table)
        }
        Err(e) => {
            event!(
                tracing::Level::ERROR,
                "coulden`t the gpio_table variable from lua and the error is ->{:?}",
                &e
            );
            T::default()
        }
    };
    result = table;
    result
}

impl Format for MyFormat {
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> std::result::Result<Map<String, config::Value>, Box<dyn std::error::Error + Send + Sync>>
    {
        // Let's assume our format is somewhat malformed, but this is fine
        // In real life anything can be used here - nom, serde or other.
        //
        // For some more real-life examples refer to format implementation within the library code
        //
        let mut result = Map::new();
        let lua = rlua::Lua::new();
        let mut config_from_lua = GpioConfig::default();

        lua.context(|lua_ctx| {
            lua_ctx.load(text).exec().unwrap();

            let table = match lua_ctx.globals().get("gpio_table") {
                Ok(table) => {
                    event!(
                        tracing::Level::DEBUG,
                        "got the gpio_table variable from lua and the value is ->{:?}",
                        &table
                    );
                    GpioConfig::from_lua_table(&table)
                }
                Err(e) => {
                    event!(
                        tracing::Level::ERROR,
                        "coulden`t the gpio_table variable from lua and the error is ->{:?}",
                        &e
                    );
                    GpioConfig::default()
                }
            };
            config_from_lua = table;
        });
        event!(
            tracing::Level::DEBUG,
            "this is the config ->{:?}",
            &config_from_lua
        );
        let json_string = serde_json::to_string(&config_from_lua)?;
        dbg!(&json_string);
        let lookup: HashMap<String, Value> = serde_json::from_str(&json_string)?;
        dbg!(&lookup);
        result.insert(
            "pump".to_string(),
            Value::new(uri, ValueKind::Table(lookup)), //TODO: the struct will
                                                       //have to become a hashmap
        );
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
