use std::io::prelude::*;
use std::{collections::HashMap, fs::File};
// use anyhow::Result;
use config::{Config, FileStoredFormat, Format, Map, ValueKind};
use rlua::{Context, FromLua, Function, Lua, MetaMethod, RegistryKey, Result, Table, UserData};
use rlua_serde;
use rlua_table_derive::FromLuaTable;
use serde::{de, Deserialize, Serialize};
use tracing::event;

pub fn config_build() {
    let mut config_file_content: String = String::new();
    std::fs::File::open("config.lua")
        .unwrap()
        .read_to_string(&mut config_file_content)
        .unwrap();
    // println!("this is the config text - {config_file_content}");
    let config = Config::builder()
        .add_source(config::File::from_str(&config_file_content, MyFormat))
        .build();

    // matth config {
    //     Ok(cfg) => println!("A config: {:#?}", cfg),
    //     Err(e) => println!("An error: {}", e),
    // }
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

fn get_lua_table_with_struct<T>(table_name_in_lua: &str, lua_ctx: &Context) -> T
where
    T: Default + FromLuaTable,
{
    let result: T;
    let table = match lua_ctx.globals().get(table_name_in_lua) {
        Ok(table) => {
            event!(
                tracing::Level::DEBUG,
                "got the {table_name_in_lua:} variable from lua and the value is ->{:?}",
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
// a small explantaion on the lifetime-
// i promise using the lifetime 'a  that both the lua table and the lua context have the same
// "lifetime"
fn from_lua_table_to_hash_map<'a>(
    lua_value: rlua::Value<'a>,
    rust_table: &mut HashMap<config::Value, config::Value>,
    lua_ctx: Context<'a>,
) -> Result<String> {
    // let lua_value: rlua::Value<'a> = <rlua::Value<'a>>::from_lua(lua_value, lua_ctx).unwrap();
    // dbg!(lua_value);
    Ok("good".to_string())
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
        let mut gpio_config_from_lua = GpioConfig::default();
        let mut gpio2_config: HashMap<String, Table> = HashMap::new();
        let mut gpio_list: Vec<GpioConfig>;
        // let mut lua_config: HashMap<rlua::String, Value> = HashMap::new();
        let mut lua_config: HashMap<config::Value, config::Value> = HashMap::new();
        println!("inside parse");

        lua.context(|lua_ctx| {
            let config: Table = lua_ctx.load(text).eval().unwrap();
            println!("tins is config return structure{:?}", config);
            // lua_config = FromLua::from_lua(rlua::Value::Table(config), lua_ctx).unwrap();
            let res = from_lua_table_to_hash_map(
                rlua::Value::Table(config),
                &mut lua_config,
                lua_ctx.clone(),
            );
            match res {
                Ok(_) => {}
                Err(_) => event!(
                    tracing::Level::ERROR,
                    "there was an error with from_lua_table_to_hash_map"
                ),
            };

            // config
            //     .pairs::<String, rlua::Value>()
            //     .for_each(|pair| { println!("this is pair - {pair:?}");
            // if pair.});
            // let globals:  =
            println!("insidecontext");
            // lua_ctx
            //     .globals()
            //     .pairs::<String, rlua::Value>()
            //     .for_each(|pair| println!("this is pair - {pair:?}"));
            // result.insert("luaconf", Value::new(uri, ValueKind::Table(globals)));
            // gpio_config_from_lua = get_lua_table_with_struct::<GpioConfig>("gpio_table", &lua_ctx);
            // gpio_list = get_lua_table_with_struct("gpio_table2", &lua_ctx);
        });
        event!(
            tracing::Level::DEBUG,
            "this is the config ->{:?}",
            &gpio_config_from_lua
        );
        dbg!(&gpio2_config);
        let json_string = serde_json::to_string(&gpio_config_from_lua)?;
        // dbg!(&json_string);
        // let lookup: HashMap<String, Value> = serde_json::from_str(&json_string)?;
        // dbg!(&lookup);
        // result.insert(
        //     "pump".to_string(),
        //     config::Value::new(uri, ValueKind::Table(lookup)), //TODO: the struct will
        //                                                        //have to become a hashmap
        // );
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
