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

async fn check_pump() {
    let text = reqwest::get("https://fakerapi.it/api/v1/custom?fname=firstName")
        .await
        .unwrap()
        .text()
        .await;
    match text {
        Ok(text) => log!("{}: rendering names", text),
        Err(e) => log!("there was an error with the sending"),
    };
}

#[component]
fn PumpWater(cx: Scope) -> impl IntoView {
    // let check_pump = move |_| log!("{}: rendering Small", "gaga");
    let (text, set_text) = create_signal(cx, "");
    let async_data = create_resource(cx, text, |value| async move { check_pump().await });
    view! {cx,<button on:click= async_data>" click me to check the pump"</button>}
}
