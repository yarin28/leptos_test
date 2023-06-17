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
        <div class="card w-96 bg-base-100 shadow-xl prose">
        <h1 class="text-red-500">"Welcome to the garden control system"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <PumpWaterCheck/>
        </div>
    }
}
#[server(CheckPump, "/check_pump")]
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
