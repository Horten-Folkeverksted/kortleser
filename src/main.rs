#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::State;
use rocket_contrib::json::Json;

use serde::{Serialize, Deserialize};

use std::io::{self, BufRead};
use std::thread;
use std::time;

use std::sync::{Arc, Mutex};

use std::collections::HashSet;

type CSN = [u8; 8];
type PACS = [u8; 5];

#[derive(Debug, Default, Deserialize)]
struct Stuff {
    scanned: HashSet<(CSN, PACS)>
}

type SharedState = Arc<Mutex<Stuff>>;

use hex::FromHex;

fn main() {

    let state = SharedState::default();

    {
        let state = state.clone();
        thread::spawn(move|| {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let list = &mut state.lock().unwrap().scanned;
                let text = line.unwrap();

                let strings: Vec<&str> = text.split("\t").take(2).collect();

                if strings.len() != 2 {
                  println!("not two tab separated elements");
                  continue;
                };

                let csn = CSN::from_hex(strings[0]);
                let csn = match csn {
                    Ok(v) => v,
                    Err(e) => {
                        println!("CSN: {}", e); 
                        continue;
                    },
                };

                let pacs = PACS::from_hex(strings[1]);
                let pacs = match pacs {
                    Ok(v) => v,
                    Err(e) => {
                        println!("PACS: {}", e); 
                        continue;
                    },
                };
                
                if list.contains(&(csn, pacs)) {
                    list.remove(&(csn, pacs));
                }
                else {
                    list.insert((csn, pacs));
                }
            }
        });
    }

    rocket::ignite()
    	.mount("/api", routes![current])
    	.manage(state)
    	.launch();

}

#[get("/current")]
fn current(_state: State<SharedState>) -> Json<HashSet<(CSN, PACS)>> {
    let state =  _state.clone();
    let state = state.lock().unwrap().scanned.clone();

    Json(state)
}
