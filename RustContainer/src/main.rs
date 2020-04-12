#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate reqwest;
use reqwest::Error;

extern crate rouille;
extern crate redis;

use rouille::Response;
use rouille::router;
use std::env;
use std::fs;

// use reqwest::r#async::{Client, Decoder};
#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

fn main() -> Result<(), Error>{
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;

    let users: Vec<User> = response.json()?;
    println!("{:?}", users);
    Ok(());
    rouille::start_server("0.0.0.0:8888", move |request| {
        router!(request,
            (GET) (/{name: String}) => {
                let name = "Lyra";
                let contents = fs::read_to_string("Template/index.html")
                .expect("Something went wrong reading the file");
                //let body = reqwest::get("http://35.208.41.153:8080")
                //.await?
                //.text()
                //.await?;
//
                //println!("body = {:?}", body);
                return Response::html(contents);
            },
            _ => Response::empty_404()
        )
    });
}

// fn do_something() -> redis::RedisResult<()> {
//     let client = redis::Client::open("redis://http://35.208.41.153:6379")?;
//     let mut con = client.get_connection()?;
// 
//     let _ : () = con.lpush("cpu",val)?;
//     /* do something here */
// 
//     Ok(())
// }