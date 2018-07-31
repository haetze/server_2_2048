extern crate tokio;
extern crate futures;

use tokio::io;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::Lines;
use tokio::io::WriteHalf;
use tokio::prelude::*;

use futures::future::FutureResult;
use futures::future::ok;

use std::io::BufReader;
use std::io::BufRead;





fn main() {  
    let addr = "127.0.0.1:12345".parse().unwrap();
    let tcp = TcpListener::bind(&addr).unwrap();


    let server = tcp.incoming()
        .for_each(|tcp| {
            let (reader, mut writer) = tcp.split();
            let reader = BufReader::new(reader);
            
            let conn = io::lines(reader)
                .and_then(|line| {
                    println!("{}", line);
                    Ok(line)
                }).and_then(move |mut line| {
                    line.push_str("\n");
                    writer.write_all(line.as_bytes());
                    Ok(())
                })
                .for_each(|_| ok(()))
                .map_err(|_| {
                    println!("Error");
                })
                
                .then(|_| -> FutureResult<(), ()> { ok(()) });
            
            
            tokio::spawn(conn);
            
            Ok(())
        })
        .map_err(|err| {
            println!("server error {:?}", err);
        });
    
    // Start the runtime and spin up the server
    tokio::run(server);
}

