
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
use core::panic;
use std::thread::current;

use reqwest::header::Keys;

use crate::sl::Todo;
use crate::sl::Departure;
use chrono::prelude::*;

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
    
    //println!("{:#?}",todos);

    #[derive(Debug)]
    struct info{
        id :i64,
        direction: String,
        display: String
    };

    let local_datetime: DateTime<Local> = Local::now();
    println!(" Current Time in {} is {}", "Stockholm",local_datetime);

    let mut out :Vec::<info>=Vec::new();
    for dep in todos.departures {
        //println!("{:#?}",dep);
        out.push(info{id: dep.line.id, direction: dep.direction, display: dep.display});
    
    }

    for i in out {
        println!("{} : {} : {}",i.display,i.direction,i.id);
    }



    Ok(())

}



#[tokio::test]
#[should_panic]
async fn incorrectURL() {

   let resp = reqwest::Client::new()
    .get("https://noreply.integration.sl.se/v1/sites/5502/departures?forecast=100")
    .send()
    .await;

    match resp {
        Ok(resp) => assert!(true),
        Err(err) => panic!("Error making request: {}", err),
    }
    
}

#[tokio::test]
async fn validURL() {

    let resp = reqwest::Client::new()
     .get("https://transport.integration.sl.se/v1/sites/5502/departures?forecast=100")
     .send()
     .await;
 
     match resp {
         Ok(resp) => assert!(true),
         Err(err) => panic!("Error making request: {}", err),
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