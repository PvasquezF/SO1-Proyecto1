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
                                return rouille::Response::html(&s);
                                let mut cause = e.source();
                                while let Some(e) = cause {
                                    println!("Reason: {}", e);
                                    cause = e.source();
                                }
                            }
                        };
                        return rouille::Response::html(&res);
                    }
                )
    });
}
