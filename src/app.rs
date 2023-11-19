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
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {

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
                    <Route path="" view=| |view! {<HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
                    <p class="text-2xl font-bold" >"Welcome to the garden control system"</p>
    <div class="flex flex-col m-px w-fit ">
        <div class="flex flex-row ">
            <div class="w-3/4">
            <ChangeCronStringComponent/>
            </div>

            <div class="w-1/4">
                        <PumpWaterCheck/>
            </div>
        </div>
        <div class="col-span-2 row-span-2">
                    <ChangeSecondsToPumpWaterComponent/>
        </div>
        <div class="flex flex-row">
            <div class="col-span-4 col-start-1 row-start-7">
                        <CancelPumpComponent/>
            </div>
            <div class="col-span-3 row-span-2 col-start-1 row-start-5">
                        <PumpWaterComponent/>
            </div>
        </div>
        <div class="col-span-2 row-span-3 col-start-3 row-start-2">
                    < PumpHelpComponent/ >
        </div>
    </div>
                <div class="card bg-base-100 shadow-xl bg-transparent  flex flex-col justify-evenly items-center transparant">
                </div>
                <CanvasComponent/>
            }
}
