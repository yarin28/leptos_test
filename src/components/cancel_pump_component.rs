use leptos::*;

use cfg_if::cfg_if;
cfg_if! {
if #[cfg(feature = "ssr")] {
use crate::utils::*;
use actix::prelude::*;
}
}
#[component]
pub fn CancelPumpComponent(cx: Scope) -> impl IntoView {
    let cx2 = cx;
    let cancel_pump = create_action(cx, move |_| async move { cancel_pump(cx2).await });
    let pending = cancel_pump.pending();
    view! {cx,
        <button class="btn btn-primary" on:click= move |ev| {
            ev.prevent_default();
            cancel_pump.dispatch(5);
            }
        class:btn-warning =move ||pending.get()
        class:btn-success=move || { cancel_pump.value().get().is_some() && !pending.get() && !check_if_empty(cancel_pump.value().get())}
        class:btn-info=move || { cancel_pump.version().get() ==0 && !pending.get() }
        class:btn-error=move || {cancel_pump.value().get().map(|v| v.unwrap_or("".to_string()).is_empty()).unwrap_or(false)
        && !pending.get() && cancel_pump.version().get() >0}
         >"cancel_the pump"</button>
    <p>{move || pending.get().then_some("waiting for response") } </p>
    <p>{move || cancel_pump.value().get()} </p>
        }
}
#[server(CancelPump, "/api")]
pub async fn cancel_pump(cx: Scope) -> Result<String, ServerFnError> {
    let res = leptos_actix::extract(
        cx,
        move |low_level_handeler: actix_web::web::Data<Addr<LowLevelHandler>>| async move {
            // let test: () = low_level_handeler;
            match low_level_handeler
                .send(LowLevelHandlerCommand::OpenRelayImmediately)
                .await
            {
                Ok(t) => Ok(t),
                Err(e) => Err(e),
            }
        },
    )
    .await;
    match res {
        Ok(val) => Ok(format!("the cancel worked! {val:?}")),
        // Ok(val) => val.into(),
        Err(e) => Err(leptos::ServerFnError::ServerError(format!(
            "couldn`t get the corn string, having a problem with the server{e}"
        ))),
    }
}
pub fn check_if_empty(value: Option<Result<String, ServerFnError>>) -> bool {
    value
        .map(|v| v.unwrap_or("".to_string()).is_empty())
        .unwrap_or(false)
}
