// the types are taken from - [Types Github]: https://github.com/DefinitelyTyped/DefinitelyTyped/blob/master/types/chart.js/index.d.ts
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScaleType {
    Category,
    Linear,
    Logarithmic,
    Time,
    RadialLinear,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PositionType {
    Left,
    Right,
    Top,
    Buttom,
    ChartArea,
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
pub struct ChartScales {
    #[serde(rename = "type")]
    pub chart_scale_type: Option<ScaleType>,
    pub position: Option<PositionType>,
}
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct ScatterData {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub r: Option<f64>,
    pub t: Option<f64>,
}
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct ChartOptions {}
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
    pub fn new(item: HtmlCanvasElement, user_config: ChartConfiguration) -> Self {
        Chart::new_internal(item, serde_wasm_bindgen::to_value(&user_config).unwrap())
    }
}
