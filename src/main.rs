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
        let message = match r.try_recv() {                                 // Recieve a message maybe, shitty library doesnt have a blocking one...
            Ok(val) => {
                val
            },
            Err(e) => {
                println!("Error: {}", e)
                continue;                                                   // If we didnt recieve anything I guess we just try again lol
            },
        };

        stream.write(&message.to_string()[..].as_bytes());                  // stream write takes a slice of u8s, but our hash is a u64. essentially turning it back to a string now
        stream.write("\n".as_bytes());                                      // dont even want to attempt adding a char to the slice...
    }
}                                                                           // If you havent been paying attention, architecture is as follows:
                                                                            //  card data -> keypresses over emulated usb kb -> read as str from a buffer
                                                                            //    -> converted to byte arrays -> hashed -> converted to String -> forced into a str slice -> into a byte array.
fn main() {
    let stdin = io::stdin();
    // This number is 10 because ¯\_(ツ)_/¯
    let (s, r) = multiqueue::broadcast_queue::<u64>(10);
    thread::spawn(move|| {
        for line in stdin.lock().lines() {
            let text = line.unwrap();                                       // Read the line
            let strings: Vec<&str> = text.split("\t").take(2).collect();    // Split into its elements
            if strings.len() != 2 {                                         // Check if its two and not one or zero...
                println!("not two tab separated elements");
                continue;
            };
            let data = convert(strings).unwrap();                           // convert the strings to byte arrays because ???
            
            let mut hasher = DefaultHasher::new();                          // Hash it using some internal hashing mechanism I dont know anything about
            data.hash(&mut hasher);                                         // Probably reversible for all I know
            let hash = hasher.finish();

            s.try_send(hash);                                               // Send it with some weird channel library that let me do SPMC with broadcasting
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