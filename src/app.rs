use crate::components::cancel_pump_component::CancelPumpComponent;
use crate::components::canvas_component::CanvasComponent;
use crate::components::change_cron_string_component::ChangeCronStringComponent;
use crate::components::change_seconds_to_pump_water_component::ChangeSecondsToPumpWaterComponent;
use crate::components::pump_help_component::PumpHelpComponent;
use crate::components::pump_water_check_component::PumpWaterCheck;
use crate::components::pump_water_copmpnent::PumpWaterComponent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use cfg_if::cfg_if;
cfg_if! {
if #[cfg(feature = "ssr")] {
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
        <Title text="Garden Pi"/>

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
            <h2 >"Welcome to the garden control system"</h2>
            <ChangeCronStringComponent/>
            <ChangeSecondsToPumpWaterComponent/>
            <PumpWaterComponent/>
            <PumpWaterCheck/>
            <CancelPumpComponent/>
            < PumpHelpComponent/ >
        </div>
        <CanvasComponent/>
    }
}
