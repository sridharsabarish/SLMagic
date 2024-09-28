
// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::slmagic;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: slmagic = serde_json::from_str(&json).unwrap();
// }

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


#[tokio::main]


async fn main() -> Result<(),reqwest::Error> {
   
    let resp = reqwest::Client::new()
    .get("https://transport.integration.sl.se/v1/sites/5502/departures?forecast=100")
    .send()
    .await;

    // Some  Error Handling
    let todos: Todo = match resp {
        Ok(resp) => match resp.json().await {
            Ok(json) => json,
            Err(err) => {
                eprintln!("Error parsing JSON: {}", err);
                return Err(err.into());
            }
        },
        Err(err) => {
            eprintln!("Error making request: {}", err);
            panic!()
        }
    };

    
    println!("{:#?}",todos);
    Ok(())


    //print!(todos[0].departures[0].display);
    

}


// use reqwest;

/// The `fetchData` function is a helper function that takes a URL as a parameter and performs an HTTP
/// GET request to that URL using the `reqwest` crate. Here's a breakdown of what the function does:
/// The `fetchData` function is a helper function that takes a URL as a parameter and performs an HTTP
/// GET request to that URL using the `reqwest` crate. Here's a breakdown of what the function does:
fn fetchData(url: &str)  {
     let resp = match reqwest::blocking::get(url) {
         Ok(resp) => resp.text().unwrap(),
         Err(resp) => panic!("Error")
     };
    

     if resp == "" {
         panic!("Empty Response");
     }
    println!("{}", resp);
   

}

// fn main(){
//     let url ="https://transport.integration.sl.se/v1/sites/5502/departures?forecast=100";
//     fetchData(url);
// }


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn test_invalid_url() {
        fetchData("https://transport.integration.sls.se/v1/sites/a5502/departures?forecast=10")
    }



}


/*

Todo :

Some Logic for finding the  route with least wait time .
1) Use the API call to find the departure from Point A to Point B
2) Get the wait times if any between the two departures
3) Find the route with least travel Time and Departures.
4) Do additional API calls from neighbouring stations if needed.
5) Choose the one with least waiting time (Less than 10 minutes) and also least travel time.

*/