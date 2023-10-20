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
#[instrument]
pub async fn pump_water(seconds: usize) -> Result<String, ServerFnError> {
    match leptos_actix::extract(
        move |low_level_handeler: actix_web::web::Data<Addr<LowLevelHandler>>| async move {
            // let test: () = low_level_handeler;
            match low_level_handeler
                .send(LowLevelHandlerCommand::CloseRelayFor(seconds))
                .await
            {
                Ok(t) => Ok(t),
                Err(e) => Err(e),
            }
        },
    )
    .await
    {
        Ok(val) => Ok(format!("the pump recived the msessage {val:?}")),
        // Ok(val) => val.into(),
        Err(e) => Err(leptos::ServerFnError::ServerError(format!(
            "couldn`t get the corn string, having a problem with the server{e}"
        ))),
    }
}

#[component]
pub fn PumpWaterComponent() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let pump_water = create_action(move |_| async move { pump_water(value.get()).await });

    let pending = pump_water.pending();
    view! {

        <div class="hidden btn-primary btn-warning btn-success btn-error"></div>//NOTE: the
            //purpuse of the div is to include those classes in the output file, because leptos
            //calls then with a diffrent syntax then tailwind-cli can see.
            <input type="range" class="range range-primary" min="1" max="100" value="50" id="myRange" on:input=move|ev|{
                ev.prevent_default();
                set_value.set(event_target_value(&ev).parse().unwrap());
                //the unwrap cant fail because the input type is range and it can output only
                //numbers between 1-100
            }/>
        <button class="btn btn-primary m-0" on:click= move |ev| {
            ev.prevent_default();
            pump_water.dispatch(value);
            }
        class:btn-warning =move ||pending.get()
        class:btn-success=move || { pump_water.value().get().is_some() && !pending.get() && !check_if_empty( pump_water.value().get())}
        class:btn-info=move || { pump_water.version().get() ==0 && !pending.get() }
        class:btn-error=move || {pump_water.value().get().map(|v| v.unwrap_or("".to_string()).is_empty()).unwrap_or(false)
        && !pending.get() && pump_water.version().get() >0}
         >" pump water"</button>
    <p class="m-0">{move || value} </p>
    <p class="m-0">{move || pending.get().then_some("waiting for response") } </p>
    <p class="m-0">{move || pump_water.value().get()} </p>
        }
}
