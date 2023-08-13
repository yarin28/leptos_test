use crate::chart::{Chart, ChartConfiguration, ChartData, ChartDataSets, ChartType};
use anyhow::Result;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlCanvasElement};

use cfg_if::cfg_if;
cfg_if! {
if #[cfg(feature = "ssr")] {
use crate::utils::pump_water as pump_water_actually;
use reqwest;
use tracing::info;
use crate::my_scheduler::SchedulerMutex;
use actix_web::web::Data;
}
}
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        <script src="https://cdn.jsdelivr.net/npm/chart.js"/>

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
        <h1 >"Welcome to the garden control system"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <PumpWaterCheck/>
        <PumpWaterComponent/>
        </div>
        <CanvasComponent/>
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

#[server(ChangeCronString, "/api")]
pub async fn change_corn_string(new_cron_string: String) -> Result<String, ServerFnError> {
    Data
    todo!()
}
#[server(PumpWater, "/api")]
pub async fn pump_water(seconds: usize) -> Result<String, ServerFnError> {
    match pump_water_actually(seconds).await {
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
        _ => Ok("there was no error from the server and the pump worked ".to_string()),
    }
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
        class:btn-success=move || { check_pump.value().get().is_some() && !pending.get() && !check_if_empty(check_pump.value().get())}
        class:btn-info=move || { check_pump.version().get() ==0 && !pending.get() }
        class:btn-error=move || {check_pump.value().get().map(|v| v.unwrap_or("".to_string()).is_empty()).unwrap_or(false)
        && !pending.get() && check_pump.version().get() >0}
         >" click me to check the pump"</button>
             <h3>"this is the pump button"</h3>
    <p>{move || pending().then_some("waiting for response") } </p>
    <p>{move || check_pump.value().get()} </p>
        }
}

#[component]
fn PumpWaterComponent(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    let pump_water = create_action(cx, move |_| async move { pump_water(value()).await });

    // let countdown_to_zero: Action<_, ()> = create_action::<I>(cx, move |&I| async move {
    //     set_countdown(value.get());
    //     while countdown.get() != 0 {
    //         sleep(Duration::from_secs(1)).await;
    //         set_countdown(countdown.get() - 1);
    //     }
    // });

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
        class:btn-success=move || { pump_water.value().get().is_some() && !pending.get() && !check_if_empty( pump_water.value().get())}
        class:btn-info=move || { pump_water.version().get() ==0 && !pending.get() }
        class:btn-error=move || {pump_water.value().get().map(|v| v.unwrap_or("".to_string()).is_empty()).unwrap_or(false)
        && !pending.get() && pump_water.version().get() >0}
         >" click me to check the pump"</button>
    <p>{move || pending().then_some("waiting for response") } </p>
    <p>{move || pump_water.value().get()} </p>
    <p>{move || value} </p>
        }
}

#[component]
fn CanvasComponent(cx: Scope) -> impl IntoView {
    let id = create_memo::<String>(cx, |t| {
        t.cloned()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())
    });
    let canvas = view! {cx, <canvas id=id/>};
    create_effect(cx, move |_| {
        console::log_2(
            &serde_wasm_bindgen::to_value("id").unwrap(),
            &serde_wasm_bindgen::to_value(&id.get()).unwrap(),
        );
        console::log_2(
            &serde_wasm_bindgen::to_value("asdf").unwrap(),
            &serde_wasm_bindgen::to_value(
                &web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id(&id.get())
                    .is_some(),
            )
            .unwrap(),
        );
        if let Some(canvas) = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(&id.get())
        {
            let t = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
            Chart::new(
                t,
                ChartConfiguration {
                    chart_type: Some(ChartType::Line),
                    data: Some(ChartData {
                        labels: Some(vec![
                            "1".to_string(),
                            "2".to_string(),
                            "3".to_string(),
                            "4".to_string(),
                        ]),
                        datasets: Some(vec![ChartDataSets {
                            label: Some("lable1".to_string()),
                            data: Some(vec![1.1, 2.2, 3.3, 4.4]),
                            ..Default::default()
                        }]),
                    }),
                },
            );
        }
    });
    view! {cx,{canvas}}
}
