
use reqwest;

fn fetchData(url: &str) {
     let resp = match reqwest::blocking::get(url) {
         Ok(resp) => resp.text().unwrap(),
         Err(err) => panic!("Error: {}", err)
     };
    println!("{}", resp);

}

fn main(){

    fetchData("https://transport.integration.sl.se/v1/sites/5502/departures?forecast=10");
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_validURL() {
    //     assert!(fetchData("https://transport.integration.sl.se/v1/sites/5502/departures?forecast=10"));
    // }

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