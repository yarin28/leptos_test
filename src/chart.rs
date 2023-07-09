use leptos::HtmlElement;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChartType {
    Timeline,
    Area,
    Bar,
    Bubble,
    Candlestick,
    Column,
    Combo,
    Gauge,
    Geo,
    Histogram,
    Radar,
    #[serde(rename = "line")]
    Line,
    Org,
    Pie,
    Scatter,
    Sparkline,
    SteppedArea,
    Table,
    Treemap,
    Waterfall,
}
#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartDataSets {
    pub background_color: Option<String>,
    //TODO: background color should be an enum
    pub bar_thickness: Option<usize>,
    pub data: Option<Vec<f32>>,
    pub label: Option<String>,
    pub fill: Option<bool>,
    //TODO: fill should be an enum
    pub border_color: Option<String>,
    //TODO: border color should be an enum
    pub tension: Option<f32>,
}
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct ChartData {
    pub labels: Option<Vec<String>>,
    pub datasets: Option<Vec<ChartDataSets>>,
}
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct ChartConfiguration {
    #[serde(rename = "type")]
    pub chart_type: Option<ChartType>,
    pub data: Option<ChartData>,
}
#[wasm_bindgen]
extern "C" {
    pub type Chart;
    #[wasm_bindgen(constructor)]
    fn new_internal(item: HtmlCanvasElement, userConfig: JsValue) -> Chart;
}

impl Chart {
    pub fn new(item: HtmlCanvasElement, userConfig: ChartConfiguration) -> Self {
        Chart::new_internal(item, serde_wasm_bindgen::to_value(&userConfig).unwrap())
    }
}
