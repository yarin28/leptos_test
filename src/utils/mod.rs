use cfg_if::cfg_if;
cfg_if! {
if #[cfg(feature = "ssr")] {
pub mod low_level_handler;
pub use low_level_handler::LowLevelHandler;
pub use low_level_handler::LowLevelHandlerCommand;
pub mod configure_logger;
pub mod config_builder;
}
}
