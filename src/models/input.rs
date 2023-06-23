use crate::models::budget::TaskBudgets;
use crate::models::goal::Goal;
use crate::models::task::Step;
use chrono::prelude::*;
use serde::Deserialize;
use std::collections::BTreeMap;

/// The front end gets a Calendar by passing a JSON data into the scheduler, via an Input object.
/// It has the requested calendar start and end, and a collection of goals that the scheduler needs to schedule.
/// On the scheduler side, the JSON is received as a ['JSValue'](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html).
/// This allows it to be deserialized into this struct.
#[derive(Deserialize, Debug)]
pub struct Input {
    #[serde(rename = "startDate")]
    pub calendar_start: NaiveDateTime,
    #[serde(rename = "endDate")]
    pub calendar_end: NaiveDateTime,
    pub goals: BTreeMap<String, Goal>,
}

#[derive(Debug, Deserialize)]
pub struct TasksToPlace {
    pub calendar_start: NaiveDateTime,
    pub calendar_end: NaiveDateTime,
    pub tasks: Vec<Step>,
    pub task_budgets: TaskBudgets,
}

#[derive(Deserialize, Debug)]
pub struct PlacedTasks {
    pub calendar_start: NaiveDateTime,
    pub calendar_end: NaiveDateTime,
    pub tasks: Vec<Step>,
}
