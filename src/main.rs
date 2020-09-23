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

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::io::{self, BufRead,};
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let mut socket = UdpSocket::bind("127.0.0.1:0")?;

    socket.connect("10.100.100.1:3333").expect("connecting to socket failed");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
          Ok(v) => v,
          Err(e) => {
            println!("Error getting line from stdin: {}", e);
            continue;
          }
        };

        let mut hasher = Sha256::new();
        hasher.input_str(&line);

        let hash = hasher.result_str();

        socket.send(hash.as_bytes()).expect("couldn't send");
        socket.send(b"\n").expect("couldn't send");  
    };
    
    Ok(())
}
