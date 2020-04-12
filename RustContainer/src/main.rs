#![feature(use_extern_macros)]
#![feature(proc_macro_hygiene)]
extern crate maud;
extern crate rouille;

use maud::html;
use rouille::Response;
use rouille::router;
use std::env;
use std::fs;

fn main() {
    rouille::start_server("0.0.0.0:8888", move |request| {
        router!(request,
            (GET) (/{name: String}) => {
                let name = "Lyra";
                let markup = html! {
                    p { "Hi, " (name) "!" }
                };
                let contents = fs::read_to_string("templates/base.html")
                .expect("Something went wrong reading the file");

                println!("With text:\n{}", contents);
                return Response::html(markup.into_string());
            },
            _ => Response::empty_404()
        )
    });
}