
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

mod sl;
use crate::sl::Todo;

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