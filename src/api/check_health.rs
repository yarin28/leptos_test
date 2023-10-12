use actix_web::Responder;
use cfg_if::cfg_if;
cfg_if! {
if #[cfg(feature = "ssr")] {
}
}
pub async fn check_health() -> impl Responder {
    "the server is up!"
}
