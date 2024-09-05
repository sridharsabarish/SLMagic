
use reqwest;

fn main() {
     let resp = match reqwest::blocking::get("https://transport.integration.sl.se/v1/sites/5502/departures?forecast=10") {
         Ok(resp) => resp.text().unwrap(),
         Err(err) => panic!("Error: {}", err)
     };
    println!("{}", resp)

}
