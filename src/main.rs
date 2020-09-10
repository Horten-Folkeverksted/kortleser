#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::State;


use std::io::{self, BufRead};
use std::thread;
use std::time;

use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
struct Stuff {
    scanned: Vec<String>
}

type SharedState = Arc<Mutex<Stuff>>;

fn main() {

    let state = SharedState::default();

    {
        let state = state.clone();
        thread::spawn(move|| {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                state.lock().unwrap().scanned.push(line.unwrap());
            }
        });
    }

    rocket::ignite()
    .mount("/", routes![index])
    .manage(state)
    .launch();

}

#[get("/")]
fn index(state: State<SharedState>) -> String {
    format!("{:?}", state.lock().unwrap().scanned)
}