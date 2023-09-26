use anyhow::Result;
use leptos::*;

use cfg_if::cfg_if;
cfg_if! {
if #[cfg(feature = "ssr")] {
use crate::utils::*;
use actix::prelude::*;
}
}
pub fn check_if_empty(value: Option<Result<String, ServerFnError>>) -> bool {
    value
        .map(|v| v.unwrap_or("".to_string()).is_empty())
        .unwrap_or(false)
}
#[server(PumpWater, "/api")]
pub async fn pump_water(cx: Scope, seconds: usize) -> Result<String, ServerFnError> {
    tracing::event!(
        tracing::Level::INFO,
        "inside the server function - water pump"
    );
    match leptos_actix::extract(
        cx,
        move |low_level_handeler: actix_web::web::Data<Addr<LowLevelHandler>>| async move {
            tracing::event!(tracing::Level::INFO, "inside the leptos_actix::extract");
            // let test: () = low_level_handeler;
            match low_level_handeler
                .send(LowLevelHandlerCommand::CloseRelayFor(seconds))
                .await
            {
                Ok(t) => {
                    tracing::event!(
                        tracing::Level::INFO,
                        "calling the low level handeler returnd {t:?}"
                    );
                    Ok(t)
                }
                Err(e) => {
                    tracing::event!(
                        tracing::Level::ERROR,
                        "calling the low level handeler returnd {e}"
                    );
                    Err(e)
                }
            }
        },
    )
    .await
    {
        Ok(val) => Ok(format!("the pump recived the msessage {val:?}")),
        // Ok(val) => val.into(),
        Err(e) => {
            tracing::event!(
                tracing::Level::ERROR,
                "there was an error in the pump, reciving the message{}",
                e
            );
            Err(leptos::ServerFnError::ServerError(format!(
                "couldn`t get the corn string, having a problem with the server{e}"
            )))
        }
    }
}

#[component]
pub fn PumpWaterComponent(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    let pump_water = create_action(
        cx,
        move |_| async move { pump_water(cx, value.get()).await },
    );

    let pending = pump_water.pending();
    view! {cx,

        <div class="hidden btn-primary btn-warning btn-success btn-error"></div>//NOTE: the
            //purpuse of the div is to include those classes in the output file, because leptos
            //calls then with a diffrent syntax then tailwind-cli can see.
            <input type="range" class="range range-primary" min="1" max="100" value="50" id="myRange" on:input=move|ev|{
                ev.prevent_default();
                set_value.set(event_target_value(&ev).parse().unwrap());
            }/>
        <button class="btn btn-primary" on:click= move |ev| {
            ev.prevent_default();
            pump_water.dispatch(value);
            }
        class:btn-warning =move ||pending.get()
        class:btn-success=move || { pump_water.value().get().is_some() && !pending.get() && !check_if_empty( pump_water.value().get())}
        class:btn-info=move || { pump_water.version().get() ==0 && !pending.get() }
        class:btn-error=move || {pump_water.value().get().map(|v| v.unwrap_or("".to_string()).is_empty()).unwrap_or(false)
        && !pending.get() && pump_water.version().get() >0}
         >" pump water"</button>
    <p>{move || value} </p>
    <p>{move || pending.get().then_some("waiting for response") } </p>
    <p>{move || pump_water.value().get()} </p>
        }
}
