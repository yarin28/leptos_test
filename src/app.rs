use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
use reqwest;
#[cfg(feature = "ssr")]
use tracing::info;
#[cfg(feature = "ssr")]
use tracing::{event, Level};

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
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <PumpWaterCheck/>
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
pub fn check_if_empty(value: Option<Result<String, ServerFnError>>) -> bool {
    value
        .map(|v| v.unwrap_or("".to_string()).is_empty())
        .unwrap_or(false)
}

#[component]
fn PumpWaterCheck(cx: Scope) -> impl IntoView {
    let check_pump = create_action(cx, |_| async move { check_pump().await });
    let pending = check_pump.pending();
    view! {cx,<button on:click= move |ev| {
            ev.prevent_default();
            check_pump.dispatch(5);
            }
        class:warning-button =pending
        class:success-button=move || { check_pump.value().get().is_some() && pending.get() ==false && !check_if_empty(check_pump.value().get())}
        class:info-button=move || { check_pump.version().get() ==0 && pending.get() ==false }
        class:error-button=move || {check_pump.value().get().map(|v| v.unwrap_or("".to_string()).is_empty()).unwrap_or(false)
        && pending.get()==false && check_pump.version().get() >0}
         >" click me to check the pump"</button>
    <p>{move || pending().then(||"waiting for response") } </p>
    <p>{move || check_pump.value().get()} </p>
        }
}
