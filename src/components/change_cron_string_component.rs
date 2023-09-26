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
pub fn ChangeCronStringComponent(cx: Scope) -> impl IntoView {
    let call_action = create_action(cx, move |cron_string: &String| {
        let cron_string = cron_string.clone();
        async move { change_corn_string(cx, cron_string).await }
    });
    let stable = create_resource(cx, || (), move |_| async move { get_cron_string(cx).await });
    let server_cron_string = stable
        .read(cx)
        .map(|val| {
            val.expect("there was en error whth ther server cron string")
            // .expect("there was en error whth ther server cron string")
        })
        .unwrap_or("there was en error whth ther server cron string".to_string());
    let (cron_string, set_cron_string) = create_signal(cx, server_cron_string);

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
        set_cron_string.set(value);
        call_action.dispatch(cron_string.get());
    };
    view! {cx,
        <form on:submit=on_submit
            class="flex flex-col items-center">
        <input type="text"
            value=move ||cron_string.get()
            node_ref=input_element
            class="input w-full max-w-xs  input-ghost input-bordered input-primary"
        />
        <input type="submit" value="Send new cron string" class="btn btn-primary btn-outline"/>
    </form>
    <p>"current cron string is: " {move ||cron_string.get()}</p>
    }
}
#[server(ChangeCronString, "/api")]
pub async fn change_corn_string(
    cx: Scope,
    new_cron_string: String,
) -> Result<String, ServerFnError> {
    leptos_actix::extract(
        cx,
        move |scheduler: actix_web::web::Data<SchedulerMutex>| {
            let new_cron_string = new_cron_string.clone();
            async move { scheduler.change_cron_string(new_cron_string).await }
        },
    )
    .await?
    .map_err(|_| ServerFnError::ServerError("couldn`t change the cron string".to_string()))?;
    Ok("the function worked".to_string())
}
#[server(GetCronString, "/api")]
pub async fn get_cron_string(cx: Scope) -> Result<String, ServerFnError> {
    match leptos_actix::extract(
        cx,
        move |scheduler_mutex: actix_web::web::Data<SchedulerMutex>| async move {
            scheduler_mutex
                .scheduler
                .lock()
                .await
                .water_pump_job_curret_corn_string
                .clone()
        },
    )
    .await
    {
        Ok(val) => Ok(val),
        // Ok(val) => val.into(),
        Err(e) => {
            tracing::event!(
                tracing::Level::ERROR,
                "there was an error in getting the cron string from the scheduler struct {}",
                e
            );
            Err(leptos::ServerFnError::ServerError(
                "couldn`t get the corn string, having a problem with the server".to_string(),
            ))
        }
    }
}
