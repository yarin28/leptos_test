use crate::chart::{Chart, ChartConfiguration, ChartData, ChartDataSets, ChartType};
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlCanvasElement};

use cfg_if::cfg_if;
cfg_if! {
if #[cfg(feature = "ssr")] {
}
}

#[allow(unused_braces)]
#[component]
pub fn CanvasComponent(cx: Scope) -> impl IntoView {
    let id = create_memo::<String>(cx, |t| {
        t.cloned()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())
    });
    let my_canvas = view! {cx, <canvas id=move ||id.get()/>};
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
