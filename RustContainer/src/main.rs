#![feature(use_extern_macros)]
#![feature(proc_macro_hygiene)]
extern crate maud;
extern crate rouille;

use maud::html;
use rouille::Response;
use rouille::router;

fn main() {
    rouille::start_server("localhost:8888", move |request| {
        router!(request,
            (GET) (/{name: String}) => {
                let name = "Lyra";
                let markup = html! {
                    p { "Hi, " (name) "!" }
                };
                return Response::html(markup.into_string());
            },
            _ => Response::empty_404()
        )
    });
}