// kortleser reads information from stdin and sends it to connected tcp clients
// Copyright (C) 2020  Daniel Løvbrøtte Olsen

// This program is free software: you can redistribute it and/or modify 
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>. 

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
