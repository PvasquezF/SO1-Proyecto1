#![feature(use_extern_macros)]
#![feature(proc_macro_hygiene)]
extern crate maud;
extern crate rouille;
extern crate redis;

use maud::html;
use rouille::Response;
use rouille::router;
use std::env;
use std::fs;
use futures::{Future, Stream};
use reqwest::r#async::{Client, Decoder};
use std::mem;

// use reqwest::r#async::{Client, Decoder};

fn main() {
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