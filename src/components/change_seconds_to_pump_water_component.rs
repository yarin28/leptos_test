use anyhow::Result;
use leptos::{html::Input, *};
use web_sys::SubmitEvent;

use cfg_if::cfg_if;

use crate::components::set_and_display_generic_component::SetAndDisplayComponent;
cfg_if! {
if #[cfg(feature = "ssr")] {
use crate::my_scheduler::SchedulerMutex;
use tracing::{event, Level};
}
}

#[component]
pub fn ChangeSecondsToPumpWaterComponent() -> impl IntoView {
    let call_action = create_action(move |seconds: &String| {
        let seconds = seconds.clone().parse::<usize>().unwrap();
        async move { change_seconds_to_pump_water(seconds).await }
    });
    let stable = create_resource(
        || (),
        move |_| async move { get_seconds_to_pump_water().await },
    );
    view! {
        <SetAndDisplayComponent
            component_name="Change pump duration".to_string()
            call_action=call_action
            stable=stable
            submit_button_description="change".to_string()
            value_description="current seconds".to_string()/>
    }
}
#[server(GetSecondsToPumpWater, "/api")]
#[instrument]
pub async fn get_seconds_to_pump_water() -> Result<String, ServerFnError> {
    event!(Level::INFO, "inside the get_seconds_to_pump_water ");
    match leptos_actix::extract(
        move |scheduler_mutex: actix_web::web::Data<SchedulerMutex>| async move {
            event!(tracing::Level::INFO, "inside get_seconds_to_pump_water");
            scheduler_mutex
                .scheduler
                .lock()
                .await
                .config
                .seconds_to_pump_water
                .clone()
                .to_string()
        },
    )
    .await
    {
        Ok(val) => {
            event!(
                Level::INFO,
                "all good with the get_seconds_to_pump_water and the reutrn value is - {val:?}"
            );
            Ok(val)
        }
        // Ok(val) => val.into(),
        Err(e) => {
            event!(Level::ERROR,"there was an error with the get_seconds_to_pump_water and the reutrn error is - {e:?}");
            Err(leptos::ServerFnError::ServerError(
                "couldn`t get the corn string, having a problem with the server".to_string(),
            ))
        }
    }
}
#[server(ChangeSecondsToPumpWater, "/api")]
pub async fn change_seconds_to_pump_water(new_seconds: usize) -> Result<String, ServerFnError> {
    leptos_actix::extract(move |scheduler: actix_web::web::Data<SchedulerMutex>| {
        let new_seconds2 = new_seconds;
        async move { scheduler.change_seconds_to_pump_water(new_seconds2).await }
    })
    .await?
    .map_err(|_| ServerFnError::ServerError("couldn`t change the cron string".to_string()))?;
    Ok("the function worked".to_string())
}
