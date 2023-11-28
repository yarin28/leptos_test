use std::collections::HashMap;

use anyhow::Result;
use leptos::*;

use cfg_if::cfg_if;

use crate::{app::get_config, utils::config::config_types};
cfg_if! {
if #[cfg(feature = "ssr")] {
use crate::utils::*;
use actix::prelude::*;
use crate::utils::low_level_handler::LowLevelHandlerCommand;
}
}
pub fn check_if_empty(value: Option<Result<String, ServerFnError>>) -> bool {
    value
        .map(|v| v.unwrap_or("".to_string()).is_empty())
        .unwrap_or(false)
}
#[server(PumpWater, "/api")]
#[instrument]
pub async fn turn_on_pin(pin_num: u8, seconds: usize) -> Result<String, ServerFnError> {
    match leptos_actix::extract(
        move |low_level_handeler: actix_web::web::Data<Addr<LowLevelHandler>>| async move {
            match low_level_handeler
                .send(LowLevelHandlerCommand {
                    pin_num,
                    message: LowLevelHandlerMessage::CloseRelayFor(seconds),
                })
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
pub fn activate_pins_component() -> impl IntoView {
    let config = expect_context::<RwSignal<Option<HashMap<String, config_types::Value>>>>();
    let activate_pins = create_resource(move || config.get(), |_| get_config());

    view! {
    <div>
        <Suspense fallback= move || view!{<p>"Loading (suspense fallback literlay)"</p>}>
        {move || {
                     activate_pins.read()
                 }}

        </Suspense>
    </div>};
}

#[component]
pub fn activate_pin_component(name: String, pin_num: usize) -> impl IntoView {
    let config = expect_context::<RwSignal<Option<HashMap<String, config_types::Value>>>>();
    // the config variable is a signal and won't be accessable until the call to the server will
    // be finished.
    let (value, set_value) = create_signal(0);
    let pump_water =
        create_action(move |_| async move { turn_on_pin(pin_num as u8, value.get()).await });

    let pending = pump_water.pending();
    view! {
    <div class="content-center">
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
         >{name}</button>
    <p class="m-0">{move || value} </p>
    <p class="m-0">{move || pending.get().then_some("waiting for response") } </p>
    <p class="m-0">{move || pump_water.value().get()} </p>
    </div>
        }
}
