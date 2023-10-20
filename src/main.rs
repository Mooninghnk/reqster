use reqwest;
use std::env;
// add extra function for handleing optional input POST/GET
fn main() {
    println!("enter a url to fetch");
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    //checks for get keyword in os args if matched run the corisponding function
    if args[0] == "get" {
        let res = fetch_get(args[1].as_str());
        match res {
            Ok(x) => println!("{:?}", x),
            Err(err) => println!("{:?}", err),
        }
        //checks for post keyword in os args if matched runs the coresponding fucntioin
    } else if args[0] == "post" {
        let res = fetch_post(args[1].as_str(), String::new());
        match res {
            Ok(x) => println!("{:?}", x),
            Err(err) => println!("{:?}", err),
        }
    }
}

// add support for error handleing and fix the return type
#[tokio::main]
async fn fetch_get(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

//seperated post function to handle post requests
#[tokio::main]
async fn fetch_post(url: &str, bd: String) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(url).body(bd).send().await?;
    Ok(res)
}

//* wrtie a parser to parse the 3rd part of the args witch is going to be be parsed streight into json
//so we can actaully make  a real post request with sending out data
//automaticly parse the data and call the json on body