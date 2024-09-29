use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct Todo {
    pub departures: Vec<Departure>,

    pub stop_deviations: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Departure {
    pub destination: String,

    pub direction_code: i64,

    pub direction: String,

    pub state: String,

    pub display: String,

    pub scheduled: String,

    pub expected: String,

    pub journey: Journey,

    pub stop_area: StopArea,

    pub stop_point: StopPoint,

    pub line: Line,

    pub deviations: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Journey {
    pub id: i64,

    pub state: String,

    pub prediction_state: Option<String>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Line {
    pub id: i64,

    pub designation: String,

    pub transport_mode: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct StopArea {
    pub id: i64,

    pub name: String,

    #[serde(rename = "type")]
    pub stop_area_type: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct StopPoint {
    pub id: i64,

    pub name: String,
}
