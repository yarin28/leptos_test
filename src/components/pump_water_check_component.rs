use leptos::*;

use cfg_if::cfg_if;
cfg_if! {
if #[cfg(feature = "ssr")] {
use reqwest;
use tracing::info;
}
}
pub fn check_if_empty(value: Option<Result<String, ServerFnError>>) -> bool {
    value
        .map(|v| v.unwrap_or("".to_string()).is_empty())
        .unwrap_or(false)
}
#[component]
pub fn PumpWaterCheck(cx: Scope) -> impl IntoView {
    let check_pump = create_action(cx, |_| async move { check_pump().await });
    let pending = check_pump.pending();
    view! {cx,

        <div class="hidden btn-primary btn-warning btn-success btn-error"></div>//NOTE: the
            //purpuse of the div is to include those classes in the output file, because leptos
            //calls then with a diffrent syntax then tailwind-cli can see.
        <button class="btn btn-primary" on:click= move |ev| {
            ev.prevent_default();
            check_pump.dispatch(5);
            }
        class:btn-warning =move ||pending.get()
        class:btn-success=move || { check_pump.value().get().is_some() && !pending.get() && !check_if_empty(check_pump.value().get())}
        class:btn-info=move || { check_pump.version().get() ==0 && !pending.get() }
        class:btn-error=move || {check_pump.value().get().map(|v| v.unwrap_or("".to_string()).is_empty()).unwrap_or(false)
        && !pending.get() && check_pump.version().get() >0}
         >"test server internet conactivity"</button>
    <p>{move || pending.get().then_some("waiting for response") } </p>
    <p>{move || check_pump.value().get()} </p>
        }
}
#[server(CheckPump, "/api")]
pub async fn check_pump() -> Result<String, ServerFnError> {
    let body = reqwest::get("http://fakerapi.it/api/v1/custom?fname=firstName")
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .text()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(body)
}
