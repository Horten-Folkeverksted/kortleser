use std::sync::{Arc, Mutex};

use crypto::digest::Digest;
use crypto::sha2::Sha256;



use std::io::{self, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use hex::FromHex;


fn main() -> std::io::Result<()> {
    let streams = Arc::new(Mutex::new(Vec::<TcpStream>::new()));


    let listener = TcpListener::bind("0.0.0.0:3333")?;
    let streams2 = streams.clone();
    thread::spawn(move || {
        for stream in listener.incoming() {
            streams2.lock().unwrap().push(stream.unwrap());
        }
    
    });
    
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;

        let mut hasher = Sha256::new();
        hasher.input_str(&line);

        let hash = hasher.result_str();

        for mut stream in streams.lock().unwrap().iter() {
            stream.write(hash.as_bytes())?;
            stream.write(b"\n")?;
        }
    };
    
    Ok(())
}