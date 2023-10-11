use leptos::*;

use cfg_if::cfg_if;
cfg_if! {
if #[cfg(feature = "ssr")] {
}
}
#[server(CheckHealth, "/api", "GetJson", "CheckHealth1")]
pub async fn check_health() -> Result<String, ServerFnError> {
    Ok("the server is up!".to_string())
}
