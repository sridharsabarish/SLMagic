mod sl;
use core::panic;
use std::thread::current;

use reqwest::header::Keys;

use crate::sl::Todo;
use crate::sl::Departure;
use chrono::prelude::*;

#[tokio::main]

async fn main()
{
    SLMagic::getSLOutput().await;
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