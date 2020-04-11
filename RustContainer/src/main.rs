#[macro_use]
extern crate tera;
extern crate rouille;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;

use std::collections::HashMap;
use rouille::Response;
use rouille::router;
use serde_json::value::{to_value, Value};
use std::error::Error;
use tera::{Context, Result, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);
        tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

pub fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(&s).unwrap())
}

fn main() {
    let mut context = Context::new();
    context.insert("username", &"Bob");
    context.insert("numbers", &vec![1, 2, 3]);
    context.insert("show_all", &false);
    context.insert("bio", &"<script>alert('pwnd');</script>");

    // A one off template
    Tera::one_off("hello", &Context::new(), true).unwrap();

    rouille::start_server("0.0.0.0:8888", move|request|{
                router!(request,
                    (GET) (/) => {
                        // If the request's URL is `/`, we jump here.
                        // This block builds a `Response` object that redirects to the `/hello/world`.
                        // If the request's URL is `/`, we jump here.
                        //// This block builds a `Response` object that redirects to the `/hello/world`.
                        let mut res = "".to_string(); 
                        match TEMPLATES.render("users/profile.html", &context) {
                            Ok(s) => { res.push_str(&s); println!("{:?}", s);},
                            Err(e) => {
                                println!("Error: {}", e);
                                let mut cause = e.source();
                                while let Some(e) = cause {
                                    println!("Reason: {}", e);
                                    cause = e.source();
                                }
                            }
                        };
                        rouille::Response::html(&res);
                        //rouille::Response::text("/hello/world")
                    },
        
                    (GET) (/hello/world) => {
                        // If the request's URL is `/hello/world`, we jump here.
                        println!("hello world");
        
                        // Builds a `Response` object that contains the "hello world" text.
                        rouille::Response::text("hello world")
                    },
        
                    (GET) (/panic) => {
                        // If the request's URL is `/panic`, we jump here.
                        //
                        // This block panics. Fortunately rouille will automatically catch the panic and
                        // send back a 500 error message to the client. This prevents the server from
                        // closing unexpectedly.
                        panic!("Oops!")
                    },
        
                    (GET) (/{id: u32}) => {
                        // If the request's URL is for example `/5`, we jump here.
                        //
                        // The `router!` macro will attempt to parse the identfier (eg. `5`) as a `u32`. If
                        // the parsing fails (for example if the URL is `/hello`), then this block is not
                        // called and the `router!` macro continues looking for another block.
                        println!("u32 {:?}", id);
        
                        // For the same of the example we return an empty response with a 400 status code.
                        rouille::Response::empty_400()
                    },
        
                    (GET) (/{id: String}) => {
                        // If the request's URL is for example `/foo`, we jump here.
                        //
                        // This route is similar to the previous one, but this time we have a `String`.
                        // Parsing into a `String` never fails.
                        println!("String {:?}", id);
        
                        // Builds a `Response` object that contains "hello, " followed with the value
                        // of `id`.
                        rouille::Response::text(format!("hello, {}", id))
                    },
        
                    // The code block is called if none of the other blocks matches the request.
                    // We return an empty response with a 404 status code.
                    _ => rouille::Response::empty_404()
                )
    });
}
