#![feature(proc_macro_hygiene, decl_macro)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use std::io::{self, BufRead, Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};use std::thread;
use std::time;

extern crate multiqueue;

use hex::FromHex;

type CSN = [u8; 8];
type PACS = [u8; 5];



fn convert(strings: Vec<&str>) -> Result<(CSN, PACS), hex::FromHexError> {
    let csn = CSN::from_hex(strings[0]);
    let csn = match csn {
        Ok(v) => v,
        Err(e) => {
            return Err(e);
        },
    };

    let pacs = PACS::from_hex(strings[1]);
    let pacs = match pacs {
        Ok(v) => v,
        Err(e) => {
            return Err(e)
        },
    };

    Ok((csn, pacs))
}

fn handle_client(mut stream: TcpStream, r: multiqueue::BroadcastReceiver<u64>) {
    loop {
        let message = match r.try_recv() {
            Ok(val) => {
                val
            },
            Err(_) => {
                continue;
            },
        };

        stream.write(&message.to_string()[..].as_bytes());
        stream.write("\n".as_bytes());
    }
}


fn main() {
    let stdin = io::stdin();
    let (s, r) = multiqueue::broadcast_queue::<u64>(10);
    thread::spawn(move|| {
        for line in stdin.lock().lines() {
            let text = line.unwrap();
    
            let strings: Vec<&str> = text.split("\t").take(2).collect();
    
            if strings.len() != 2 {
                println!("not two tab separated elements");
                continue;
            };
            let data = convert(strings).unwrap();
            
            let mut hasher = DefaultHasher::new();
            data.hash(&mut hasher);
            let hash = hasher.finish();

            s.try_send(hash);
        }
    });

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let r = r.add_stream();
                thread::spawn(move|| {
                    handle_client(stream, r)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}