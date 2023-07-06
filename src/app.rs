use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
use crate::utils::pump_water as pump_water_actually;

#[cfg(feature = "ssr")]
use anyhow::Result;
#[cfg(feature = "ssr")]
use reqwest;
#[cfg(feature = "ssr")]
use tracing::info;
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <div class="card w-96 bg-base-100 shadow-xl prose">
        <h1 class="text-red-500">"Welcome to the garden control system"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <PumpWaterCheck/>
            <PumpWaterComponent/>
        </div>
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
    info!("the body is -{:?} ", body);
    Ok(body)
}

#[server(PumpWater, "/api")]
pub async fn pump_water(seconds: usize) -> Result<String, ServerFnError> {
    let res = match pump_water_actually(seconds).await {
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
        _ => Ok("there was no error from the server and the pump worked ".to_string()),
    };
    res
}

pub fn check_if_empty(value: Option<Result<String, ServerFnError>>) -> bool {
    value
        .map(|v| v.unwrap_or("".to_string()).is_empty())
        .unwrap_or(false)
}
#[component]
fn PumpWaterCheck(cx: Scope) -> impl IntoView {
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
        class:btn-warning =pending
        class:btn-success=move || { check_pump.value().get().is_some() && pending.get() ==false && !check_if_empty(check_pump.value().get())}
        class:btn-info=move || { check_pump.version().get() ==0 && pending.get() ==false }
        class:btn-error=move || {check_pump.value().get().map(|v| v.unwrap_or("".to_string()).is_empty()).unwrap_or(false)
        && pending.get()==false && check_pump.version().get() >0}
         >" click me to check the pump"</button>
    <p>{move || pending().then(||"waiting for response") } </p>
    <p>{move || check_pump.value().get()} </p>
        }
}

#[component]
fn PumpWaterComponent(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    let pump_water = create_action(
        cx,
        move |_| async move { pump_water(value().clone()).await },
    );
    //NOTE: there could be a problem if i clone the value, will check it now.
    let pending = pump_water.pending();
    view! {cx,

        <div class="hidden btn-primary btn-warning btn-success btn-error"></div>//NOTE: the
            //purpuse of the div is to include those classes in the output file, because leptos
            //calls then with a diffrent syntax then tailwind-cli can see.
            <input type="range" class="range range-primary" min="1" max="100" value="50" id="myRange" on:input=move|ev|{
                ev.prevent_default();
                set_value(event_target_value(&ev).parse().unwrap());
            }/>
        <button class="btn btn-primary" on:click= move |ev| {
            ev.prevent_default();
            pump_water.dispatch(value);
            }
        class:btn-warning =pending
        class:btn-success=move || { pump_water.value().get().is_some() && pending.get() ==false && !check_if_empty( pump_water.value().get())}
        class:btn-info=move || { pump_water.version().get() ==0 && pending.get() ==false }
        class:btn-error=move || {pump_water.value().get().map(|v| v.unwrap_or("".to_string()).is_empty()).unwrap_or(false)
        && pending.get()==false && pump_water.version().get() >0}
         >" click me to check the pump"</button>
    <p>{move || pending().then(||"waiting for response") } </p>
    <p>{move || pump_water.value().get()} </p>
    <p>{move || value} </p>
        }
}
