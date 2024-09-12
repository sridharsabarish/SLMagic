
use reqwest;

fn fetchData(url: &str) {
     let resp = match reqwest::blocking::get(url) {
         Ok(resp) => resp.text().unwrap(),
         Err(err) => panic!("Error: {}", err)
     };
    println!("{}", resp);

}

fn main(){
    let url ="https://transport.integration.sl.se/v1/sites/5502/departures?forecast=10";
    fetchData(url);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn test_invalid_url() {
        fetchData("SADASSSAD")
    }


    #[test]
    fn test_valid_url() {
        fetchData("https://transport.integration.sl.se/v1/sites/5502/departures?forecast=10")
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