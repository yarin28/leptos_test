#![feature(async_closure)]
use crate::chart::{Chart, ChartConfiguration, ChartData, ChartDataSets, ChartType};
use anyhow::Result;
use leptos::{html::Input, *};
use leptos_meta::*;
use leptos_router::*;
use tokio::sync::mpsc;
use tracing::span;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlCanvasElement, SubmitEvent};

use cfg_if::cfg_if;
cfg_if! {
if #[cfg(feature = "ssr")] {
use reqwest;
use tracing::info;
use crate::my_scheduler::SchedulerMutex;
use crate::utils::*;
use actix::prelude::*;
}
}

// pub async fn get_mutex_scheduler(
//     scheduler: actix_web::web::Data<SchedulerMutex>,
// ) -> Result<SchedulerMutex> {
//     todo!();
// }
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
    view! { cx,
              <div class="card w-96 bg-base-100 shadow-xl prose flex flex-col justify-evenly items-center">
            <h1 >"Welcome to the garden control system"</h1>
            <ChangeCronStringComponent/>
            <PumpWaterComponent/>
            <PumpWaterCheck/>
            <CancelPumpComponent/>
    < PumpHelpComponent/ >
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

#[server(CancelPump, "/api")]
pub async fn cancel_pump(cx: Scope) -> Result<String, ServerFnError> {
    tracing::event!(tracing::Level::INFO, "inside the cancel pump");
    match leptos_actix::extract(
        cx,
        move |low_level_handeler: actix_web::web::Data<Addr<LowLevelHandler>>| async move {
            tracing::event!(tracing::Level::INFO, "inside the leptos_actix::extract");
            // let test: () = low_level_handeler;
            match low_level_handeler
                .send(LowLevelHandlerCommand::OpenRelayImmediately)
                .await
            {
                Ok(t) => {
                    tracing::event!(
                        tracing::Level::INFO,
                        "calling the low level handeler returnd {t:?}"
                    );
                    Ok(t)
                }
                Err(e) => {
                    tracing::event!(
                        tracing::Level::ERROR,
                        "calling the low level handeler returnd {e}"
                    );
                    Err(e)
                }
            }
        },
    )
    .await
    {
        Ok(val) => Ok(format!("the cancel worked! {val:?}")),
        // Ok(val) => val.into(),
        Err(e) => {
            tracing::event!(
                tracing::Level::ERROR,
                "there was an error in ther cancel pump function{}",
                e
            );
            Err(leptos::ServerFnError::ServerError(format!(
                "couldn`t get the corn string, having a problem with the server{e}"
            )))
        }
    }
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
#[server(PumpWater, "/api")]
pub async fn pump_water(cx: Scope, seconds: usize) -> Result<String, ServerFnError> {
    tracing::event!(
        tracing::Level::INFO,
        "inside the server function - water pump"
    );
    match leptos_actix::extract(
        cx,
        move |low_level_handeler: actix_web::web::Data<Addr<LowLevelHandler>>| async move {
            tracing::event!(tracing::Level::INFO, "inside the leptos_actix::extract");
            // let test: () = low_level_handeler;
            match low_level_handeler
                .send(LowLevelHandlerCommand::CloseRelayFor(seconds))
                .await
            {
                Ok(t) => {
                    tracing::event!(
                        tracing::Level::INFO,
                        "calling the low level handeler returnd {t:?}"
                    );
                    Ok(t)
                }
                Err(e) => {
                    tracing::event!(
                        tracing::Level::ERROR,
                        "calling the low level handeler returnd {e}"
                    );
                    Err(e)
                }
            }
        },
    )
    .await
    {
        Ok(val) => Ok(format!("the cancel worked! {val:?}")),
        // Ok(val) => val.into(),
        Err(e) => {
            tracing::event!(
                tracing::Level::ERROR,
                "there was an error in ther cancel pump function{}",
                e
            );
            Err(leptos::ServerFnError::ServerError(format!(
                "couldn`t get the corn string, having a problem with the server{e}"
            )))
        }
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
         >"test server internet conactivity"</button>
    <p>{move || pending().then_some("waiting for response") } </p>
    <p>{move || check_pump.value().get()} </p>
        }
}

#[component]
fn PumpWaterComponent(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    let pump_water = create_action(cx, move |_| async move { pump_water(cx, value()).await });

    let mut countdown_value = value.get();
    countdown_value = 1000;
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
                countdown_value = value.get();
                pump_water.dispatch(value);
                }
            class:btn-warning =pending
            class:btn-success=move || { pump_water.value().get().is_some() && !pending.get() && !check_if_empty( pump_water.value().get())}
            class:btn-info=move || { pump_water.version().get() ==0 && !pending.get() }
            class:btn-error=move || {pump_water.value().get().map(|v| v.unwrap_or("".to_string()).is_empty()).unwrap_or(false)
            && !pending.get() && pump_water.version().get() >0}
             >" pump water"</button>
        <p>{move || value} </p>
        <p>{move || pending().then_some("waiting for response") } </p>
        <p>{move || pump_water.value().get()} </p>
        <div>
        <span class="countdown font-mono text-6xl">
      <span style="--value:{countdown_value}"></span>
    </span>
    </div>
            }
}

#[allow(unused_braces)]
#[component]
fn CanvasComponent(cx: Scope) -> impl IntoView {
    let id = create_memo::<String>(cx, |t| {
        t.cloned()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())
    });
    let my_canvas = view! {cx, <canvas id=id/>};
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
    view! {cx,{my_canvas}}
}
#[component]
fn ChangeCronStringComponent(cx: Scope) -> impl IntoView {
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
        let value = input_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_cron_string(value);
        call_action.dispatch(cron_string.get());
    };
    view! {cx,
        <form on:submit=on_submit
            class="flex flex-col items-center">
        <input type="text"
            value=cron_string
            node_ref=input_element
            class="input w-full max-w-xs  input-ghost input-bordered input-primary"
        />
        <input type="submit" value="Send new cron string" class="btn btn-primary btn-outline"/>
    </form>
    <p>"current cron string is: " {cron_string}</p>
    }
}
#[component]
fn CancelPumpComponent(cx: Scope) -> impl IntoView {
    let cx2 = cx;
    let cancel_pump = create_action(cx, move |_| async move { cancel_pump(cx2).await });
    let pending = cancel_pump.pending();
    view! {cx,
        <button class="btn btn-primary" on:click= move |ev| {
            ev.prevent_default();
            cancel_pump.dispatch(5);
            }
        class:btn-warning =pending
        class:btn-success=move || { cancel_pump.value().get().is_some() && !pending.get() && !check_if_empty(cancel_pump.value().get())}
        class:btn-info=move || { cancel_pump.version().get() ==0 && !pending.get() }
        class:btn-error=move || {cancel_pump.value().get().map(|v| v.unwrap_or("".to_string()).is_empty()).unwrap_or(false)
        && !pending.get() && cancel_pump.version().get() >0}
         >"cancel_the pump"</button>
    <p>{move || pending().then_some("waiting for response") } </p>
    <p>{move || cancel_pump.value().get()} </p>
        }
}
#[component]
fn StatComponet(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button

    view! { cx,
                <div class="stat flex flex-row-reverse items-center justify-evenly">
      <div class="stat-figure text-secondary">
        <div class="avatar online">
          <div class="w-16 rounded-full">
            <img class="m-0" src="icons/lion-svgrepo-com.svg" />
          </div>
        </div>
      </div>
      <div>
      <div class="stat-value">"86%"</div>
      <div class="stat-title">"Tasks done"</div>
      <div class="stat-desc text-secondary">"31 tasks remaining"</div>
      </div>
    </div>
          }
}
#[component]
fn PumpHelpComponent(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button

    view! { cx,
        <div class="backdrop-blur-sm collapse ">
      <input type="checkbox" />
      <div class="collapse-title text-xl font-medium">
    " Click me to show/hide help menu "
      </div>
      <div class="collapse-content">
      <h3>"Hello! this is the pump control platform"</h3>
        <h4>"there are 3 buttons to chose from"</h4>
        <ul>
        <li>" to check if the pump has internet connection press the CHECK INTERNET button, "</li>
        <li>"to activate the pump manually slide the slider to the desired amount of seconds and press the PUMP WATER button"</li>
        <li>"to change the schedule string press the CHANGE SCHEDULE STRING after inserting the string"</li>
        </ul>
      </div>
    </div>
              }
}
