use anyhow::Result;
use leptos::{html::Input, *};
use web_sys::SubmitEvent;

use cfg_if::cfg_if;
cfg_if! {
if #[cfg(feature = "ssr")] {
use crate::my_scheduler::SchedulerMutex;
}
}
#[component]
pub fn SetAndDisplayComponent(
    cx: Scope,
    call_action: Action<String, Result<String, ServerFnError>>,
    stable: Resource<(), Result<String, ServerFnError>>,
    component_name: String,
    submit_button_description: String,
    value_description: String,
    // server_api_function: fn(Scope) -> Result<String, ServerFnError>,
) -> impl IntoView {
    let seconds = stable
        .read(cx)
        .map(|val| {
            val.expect("there was en error whth ther server cron string")
            // .expect("there was en error whth ther server cron string")
        })
        .unwrap_or("there was en error whth ther server cron string".to_string());
    let (seconds_value, set_seconds_value) = create_signal(cx, seconds);
    let input_element: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_seconds_value.set(value);
        call_action.dispatch(seconds_value.get());
    };
    view! {cx,
        <form on:submit=on_submit
            class="flex flex-row items-center">
            <h1>{move||component_name.clone()}</h1>
        <input type="text"
            value=move ||seconds_value.get()
            node_ref=input_element
            class="input w-full max-w-xs  input-ghost input-bordered input-primary"
        />
        <input type="submit" value=move|| submit_button_description.clone()
        class="btn btn-primary btn-outline"/>
    </form>
    <p>{move||value_description.clone()}"->" {move||seconds_value.get()}</p>
    }
}

#[component]
pub fn ChangeSecondsToPumpWaterComponent(cx: Scope) -> impl IntoView {
    let call_action = create_action(cx, move |seconds: &String| {
        let seconds = seconds.clone().parse::<usize>().unwrap();
        async move { change_seconds_to_pump_water(cx, seconds).await }
    });
    let stable = create_resource(
        cx,
        || (),
        move |_| async move { get_seconds_to_pump_water(cx).await },
    );
    view! {cx,
        <SetAndDisplayComponent
            component_name="green".to_string()
            call_action=call_action stable=stable
            submit_button_description="submit desc".to_string()
            value_description="value_description".to_string()/>
    }
}
#[component]
pub fn ChangeSecondsToPumpWaterComponentOld(cx: Scope) -> impl IntoView {
    let call_action = create_action(cx, move |seconds: &String| {
        let seconds = seconds.clone().parse::<usize>().unwrap();
        async move { change_seconds_to_pump_water(cx, seconds).await }
    });
    let stable = create_resource(
        cx,
        || (),
        move |_| async move { get_seconds_to_pump_water(cx).await },
    );
    let seconds = stable
        .read(cx)
        .map(|val| {
            val.expect("there was en error whth ther server cron string")
            // .expect("there was en error whth ther server cron string")
        })
        .unwrap_or("there was en error whth ther server cron string".to_string());
    let (seconds_value, set_seconds_value) = create_signal(cx, seconds);

    let input_element: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_seconds_value.set(value);
        call_action.dispatch(seconds_value.get());
    };
    view! {cx,
        <form on:submit=on_submit
            class="flex flex-row items-center">
        <input type="text"
            value=move ||seconds_value.get()
            node_ref=input_element
            class="input w-full max-w-xs  input-ghost input-bordered input-primary"
        />
        <input type="submit" value="how long every pump last" class="btn btn-primary btn-outline"/>
    </form>
    <p>"how long every pump last: " {move||seconds_value.get()}</p>
    }
}
#[server(GetSecondsToPumpWater, "/api")]
pub async fn get_seconds_to_pump_water(cx: Scope) -> Result<String, ServerFnError> {
    match leptos_actix::extract(
        cx,
        move |scheduler_mutex: actix_web::web::Data<SchedulerMutex>| async move {
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
        Ok(val) => Ok(val),
        // Ok(val) => val.into(),
        Err(_e) => Err(leptos::ServerFnError::ServerError(
            "couldn`t get the corn string, having a problem with the server".to_string(),
        )),
    }
}
#[server(ChangeSecondsToPumpWater, "/api")]
pub async fn change_seconds_to_pump_water(
    cx: Scope,
    new_seconds: usize,
) -> Result<String, ServerFnError> {
    leptos_actix::extract(
        cx,
        move |scheduler: actix_web::web::Data<SchedulerMutex>| {
            let new_seconds2 = new_seconds;
            async move { scheduler.change_seconds_to_pump_water(new_seconds2).await }
        },
    )
    .await?
    .map_err(|_| ServerFnError::ServerError("couldn`t change the cron string".to_string()))?;
    Ok("the function worked".to_string())
}
