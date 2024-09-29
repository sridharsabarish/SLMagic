use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct Todo {
    departures: Vec<Departure>,

    stop_deviations: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Departure {
    destination: String,

    direction_code: i64,

    direction: String,

    state: String,

    display: String,

    scheduled: String,

    expected: String,

    journey: Journey,

    stop_area: StopArea,

    stop_point: StopPoint,

    line: Line,

    deviations: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Journey {
    id: i64,

    state: String,

    prediction_state: Option<String>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Line {
    id: i64,

    designation: String,

    transport_mode: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct StopArea {
    id: i64,

    name: String,

    #[serde(rename = "type")]
    stop_area_type: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct StopPoint {
    id: i64,

    name: String,
}
