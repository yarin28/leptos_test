use leptos::{
    ev::{Event, MouseEvent},
    *,
};
use leptos_meta::*;
use leptos_router::*;
use reqwest;
use std::error::Error;

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
        <PumpWater/>
    }
}

#[server(CheckPump, "/api")]
pub async fn check_pump() -> Result<(), ServerFnError> {
    let body = reqwest::get("http://fakerapi.it/api/v1/custom?fname=firstName").await;
    match body {
        Ok(b) => {
            match b.text().await {
                Ok(text) => log!("{}: rendering names", text),
                Err(_e) => log!("there was an error with the sending"),
            };
        }
        Err(e) => log!("{:?}", e),
    }
    Ok(())
}

#[component]
fn PumpWater(cx: Scope) -> impl IntoView {
    view! {cx,<button on:click= move |_| {
        spawn_local(async{
            check_pump().await.unwrap();
        })
    } >" click me to check the pump"</button>}
}
