use leptos::*;
#[component]
pub fn PumpHelpComponent(cx: Scope) -> impl IntoView {
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
