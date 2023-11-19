use anyhow::Result;
use leptos::{html::Input, *};
use web_sys::SubmitEvent;

use cfg_if::cfg_if;
cfg_if! {
if #[cfg(feature = "ssr")] {
}
}
#[component]
pub fn SetAndDisplayComponent(
    call_action: Action<String, Result<String, ServerFnError>>,
    stable: Resource<(), Result<String, ServerFnError>>,
    component_name: String,
    submit_button_description: String,
    value_description: String,
    // server_api_function: fn(Scope) -> Result<String, ServerFnError>,
) -> impl IntoView {
    let seconds = stable
        .get()
        .map(|val| {
            val.expect("there was en error whth ther server cron string")
            // .expect("there was en error whth ther server cron string")
        })
        .unwrap_or("there was en error whth ther server cron string".to_string());
    let (seconds_value, set_seconds_value) = create_signal(seconds);
    let input_element: NodeRef<Input> = create_node_ref();
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
    view! {
    <div class="stats glass text-primary-content">

        <div class="stat">
            <form on:submit=on_submit>
                    <div class="stat-title text-black text-lg">{move || component_name.clone()}</div>
                    // <div class="stat-title text-black text-lg">{move || component_name.clone()}</div>
                        <div class="stat-value">
                        <input type="text"
                            value=move ||seconds_value.get()
                            node_ref=input_element
                            // class="input w-full max-w-xs m-0  input-ghost input-bordered input-primary"
                            class="input w-full max-w-xs m-0 input-ghost  input-bordered input-primary font-bold text-2xl"
                        />
                        </div>
                    <div class="stat-actions flex flex-row justify-between">
                        <input type="submit" class="btn btn-sm btn-success" value=move|| submit_button_description.clone()/>
                        <p class="m-0">{move||value_description.clone()}"->" {move||seconds_value.get()}</p>
                    </div>
            </form>
        </div>
    </div>
    }
}
