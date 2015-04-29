#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate iron;
extern crate serde;
extern crate router;
extern crate mount;
extern crate staticfile;

use std::env;
use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::path::Path;
use std::net::Ipv4Addr;
use iron::prelude::*;
use iron::status;
use mount::Mount;
use staticfile::Static;
use serde::json;
use router::Router;

#[derive(Serialize, Deserialize)]
struct Greeting {
    msg: String
}

fn main() {

    let greeting = Arc::new(Mutex::new(Greeting { msg: "Hello".to_string() }));
    let greeting_clone = greeting.clone();

    let mut mount = Mount::new();

    let mut router = Router::new();

    router.get("/", move |r: &mut Request| hello_world(r, &greeting.lock().unwrap()));
    router.post("/set", move |r: &mut Request| set_greeting(r, &mut greeting_clone.lock().unwrap()));

    fn hello_world(_: &mut Request, greeting: &Greeting) -> IronResult<Response> {
        let payload = json::to_string(greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    fn set_greeting(request: &mut Request, greeting: &mut Greeting) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        *greeting = json::from_str(&payload).unwrap();
        Ok(Response::with((status::Ok)))
    }

    mount.mount("/api", router);
    mount.mount("/", Static::new(Path::new("www")));



    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let port = get_server_port();
    println!("Binding to {}:{}", ip, port);
    match Iron::new(mount).http((ip, port)) {
        Ok(_) => (),
        Err(err) => panic!("{}", err)
    }
    println!("Running");

}

fn get_server_port() -> u16 {
    match env::var("PORT") {
        Ok(s) => FromStr::from_str(&s[..]).unwrap_or(8080),
        Err(_) => 8080
    }
}
