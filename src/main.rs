use clap::Parser;
use reqwest::{self, Response};
// figure out how the option args syntax works
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// url address of the api
    #[arg(short)]
    link: Option<String>,
    /// request type
    #[arg(short, default_value_t = String::new())]
    get: String,
    /// count to run
    #[arg(short, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        let cmp = args.link.as_deref().unwrap();
        println!("{:?}", cmp);
        if args.get.contains("get") || args.get.contains("Get") {
            let resp = get_req(cmp);
            println!("{:?}", resp);
        }
    }
}
#[tokio::main]
async fn get_req(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

//add post support with json
//transfer 25 into a fn
//update the instruction on usage
//lines 8 11 14
