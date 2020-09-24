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

use clap::{Arg, App};

fn main() -> std::io::Result<()> {
    let matches = App::new("kortleser")
        .version("0.3.0")
        .author("Daniel Olsen <daniel.olsen@folkeverkstedet.com>")
        .about("Overcomplicated nc -u <host> <port>")
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .takes_value(true)
            .required(true)
            .help("ip or domain name of server"))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .takes_value(true)
            .required(true)
            .help("UDP Port to connect to"))
        .get_matches();

    let socket = UdpSocket::bind("127.0.0.1:0")?;

    socket.connect((matches.value_of("host").unwrap(), matches.value_of("port").unwrap_or("3333").parse::<u16>().unwrap())).expect("connecting to socket failed");

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

        match socket.send(hash.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                println!("Couldn't send data: {}", e);
                continue;
            },
        };
        match socket.send(b"\n") {
            Ok(_) => (),
            Err(e) => {
                println!("Couldn't send newline {}", e);
                continue;
            },
        };
    };
    
    Ok(())
}
