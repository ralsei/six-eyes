use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Label {
    pub position: HashMap<String, f32>,
    pub txt: String,
}

#[derive(Debug, Deserialize)]
pub struct CubeMessage {
    pub id: String,
    pub dims: Vec<String>,
    pub labels: Vec<Label>,
}

#[derive(Debug, Deserialize)]
pub struct DisplayGoal {
    pub dims: Vec<String>,
    pub labels: Vec<Label>,
    pub context: String,
    pub cubes: Vec<CubeMessage>,
}

#[derive(Debug, Deserialize)]
pub enum Message {
    DisplayGoal(DisplayGoal),
}
