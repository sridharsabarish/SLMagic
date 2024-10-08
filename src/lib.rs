mod sl;
use core::panic;
use std::thread::current;

use reqwest::header::Keys;

use crate::sl::Todo;
use crate::sl::Departure;
use chrono::prelude::*;



pub async fn getSLOutput() -> Result<(),reqwest::Error> {
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