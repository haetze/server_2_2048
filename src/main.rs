#![allow(unused_must_use)]

extern crate lib_2048;
extern crate tokio;
extern crate futures;

use lib_2048::data::Field;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

use futures::future::ok;

use std::io::BufReader;
use std::env;
use std::net::SocketAddr;

use commands::Command;

mod commands;

const DEFAULT_PORT: u16 = 4343;

fn main() {
    // Reading arguments for Port to run on
    let port_requested: u16 = match env::args().skip(1).next() {
        Some(p) => match p.parse() {
            Ok(port) => port,
            Err(_)   => DEFAULT_PORT,
        },
        None    => DEFAULT_PORT,
    };
    println!("Running on Port: {}", port_requested);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], port_requested));
    let tcp = TcpListener::bind(&addr).unwrap();

    println!("Server running");
    let mut connection_count = 0;
    // Server Future
    let server = tcp.incoming()
        .for_each(move |tcp| {
            // Adds one to the connection counter
            connection_count = connection_count + 1;
            // Copys the current state over in a local variable
            let current_connection_number = connection_count;
            println!("Connection #{} opened", connection_count);
            let (reader, mut writer) = tcp.split();
            let reader = BufReader::new(reader);
            let mut field = None;
            
            // Connection Future
            // Basically a remote REPL
            // or RREPL
            let conn = io::lines(reader)
                .and_then(move |line| {
                    let response = handle_messages(line, &mut field);
                    ok(response)
                }).and_then(move |l| {
                    writer.write_all(l.as_bytes())
                })
                .for_each(|_| ok(())) // Collects the whole stream til the end
                .and_then(move |_| {
                    // Prints that the collection is closed
                    // This works because for_each only returns when stream
                    // Is completely handled, so only when Stream is done
                    println!("Connection #{} closed", current_connection_number);
                    ok(())
                })
                .map_err(|_| {
                    println!("Error");
                });
                

            tokio::spawn(conn);
            
            Ok(())
        })
        .map_err(|err| {
            println!("server error {:?}", err);
        });
    
    // Start the runtime and spin up the server
    tokio::run(server);
}


fn handle_messages(command: String, mut field: &mut Option<Field>) -> String{

    // Translates received String to Command
    match command.trim() {
        "right" => handle_command(&mut field, Command::Right),
        "left"  => handle_command(&mut field, Command::Left),
        "up"    => handle_command(&mut field, Command::Up),
        "down"  => handle_command(&mut field, Command::Down),
        "exit"  => return "".to_string(),
        other   => {
            let commands: Vec<&str> = other.split_whitespace().collect();
            if commands.len() == 0 { return print_result(field);}
            if let "new" = commands[0] {
                let scale = match commands[1].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => 4,
                };
                
                handle_command(&mut field, Command::New(scale))
            } else {
                "Unsupported Command\n".to_string()
            }
        },
    }

    
}

fn handle_command(mut field: &mut Option<Field>, command: Command) -> String {
    use std::mem::swap;

    // Clones Field for later comparison
    // in case of invalid move
    let mut tmp_field = field.clone();

    // If command is "New Command"
    // Field is gonna be set
    // Only match will filter None 's
    // Because of map
    match command {
        Command::New(n) => tmp_field = Some(Field::new(n)),
        _               => (),
    };

    // Executes Command
    let execute_command_field = tmp_field.map(|mut inner_field| {
        match command {
            Command::New(_) => inner_field,
            Command::Right  => {
                inner_field.swipe_right();
                inner_field
            },
            Command::Left  => {
                inner_field.swipe_left();
                inner_field
            },
            Command::Up  => {
                inner_field.swipe_up();
                inner_field
            },
            Command::Down  => {
                inner_field.swipe_down();
                inner_field
            },
        }
    });

    // Compares to old state
    // If equal then nothing happend because of the Command
    // and no new number is added
    let mut result_field = execute_command_field.map(|mut inner_field| {
        match field {
            None => {
                inner_field.insert_random();
            },
            Some(field) => {
                if field != &mut inner_field {
                    inner_field.insert_random();
                }
            },       
        };
        inner_field
    });

       
    swap(&mut result_field, &mut field);
    print_result(&field)
    
}


// Function that takes a optional Field and return the
// String representing it.
fn print_result(field: &Option<Field>) -> String{
    match field {
        None => {
            "Empty\n".to_string()
        },

        Some(field) => {
            let mut string = String::new();
            //Print each row
            for row in &field.rows {
                let s = format!("{:?}", row.row);
                // Add row to result String
                // Add delimiter (;) to String
                string.push_str(&s);
                string.push_str(&";");
            }
            string.push_str(&"\n");
            string
        },
    }
}

