
#![feature(use_extern_macros)]
#![feature(proc_macro_hygiene)]
extern crate maud;
#[macro_use] extern crate rouille;

use maud::html;
use rouille::Response;

fn main() {
    rouille::start_server("localhost:8888", move |request| {
        router!(request,
            (GET) (/{name: String}) => {
                html! {
                    h1 { "Hello, " (name) "!" }
                    p { "Nice to meet you!" }
                }
            },
            _ => Response::empty_404()
        )
    });
}