#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate reqwest;
extern crate chrono;
extern crate serialize;
use reqwest::Error;

extern crate rouille;
extern crate redis;

use rouille::Response;
use rouille::router;
use std::env;
use std::fs;
use chrono::prelude::*;
use redis::Commands;
use serialize::{Decodable, Encodable, json};
// use reqwest::r#async::{Client, Decoder};
#[derive(Deserialize, Debug)]
struct Data {
    cpu: Cpu
}

#[derive(Deserialize, Debug)]
struct Cpu {
    read: f64
}

#[derive(Serialize, Debug)]
struct RedisData{
	Valor:  String,
	Tiempo: String
}

fn main() -> Result<(), Error>{
    let request_url = format!("http://35.208.41.153:8080");
    let mut response = reqwest::get(&request_url)?;

    let data: Data = response.json()?;
    let utc: DateTime<Utc> = Utc::now();
    let redisSend: RedisData = RedisData{Valor: data.cpu.read.to_string(), Tiempo: utc.format("%Y-%m-%d %H:%M:%S").to_string()};
    println!("{:?}", redisSend);
    save(redisSend);
    rouille::start_server("0.0.0.0:8888", move |request| {
        router!(request,
            (GET) (/{name: String}) => {
                let name = "Lyra";
                let contents = fs::read_to_string("Template/index.html")
                .expect("Something went wrong reading the file");
                return Response::html(contents);
            },
            _ => Response::empty_404()
        )
    });
    Ok(())
}

fn save(red: RedisData) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://http://35.208.41.153:6379")?;
    let mut con = client.get_connection()?;
    let _ : () = con.lpush("cpu", json::encode(red))?;
    Ok(())
}