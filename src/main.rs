extern crate lib_2048;
extern crate tokio;
extern crate futures;

use lib_2048::data::Field;

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

use commands::Command;

mod commands;

fn main() {  
    let addr = "127.0.0.1:12345".parse().unwrap();
    let tcp = TcpListener::bind(&addr).unwrap();


    let server = tcp.incoming()
        .for_each(|tcp| {
            let (reader, mut writer) = tcp.split();
            let reader = BufReader::new(reader);
            let mut field = None;
            let conn = io::lines(reader)
                .and_then(move |line| {
                    let response = handle_messages(line, &mut field);
                    Ok(response)
                }).and_then(move |mut l| {
                    writer.write_all(l.as_bytes());
                    Ok(())
                })
                .for_each(|_| ok(()))
                .map_err(|_| {
                    println!("Error");
                })
                
                .then(|_| -> FutureResult<(), ()> {
                    ok(()) });
            
            
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

fn handle_command(mut field_option: &mut Option<Field>, command: Command) -> String {
    use std::mem::swap;
    
    let mut field = None;
    swap(&mut field, &mut field_option);

    let tmp_field = field.clone();
    
    let result_field = match field {
        None => {
            match command {
                Command::New(n) => Some(Field::new(n)),
                _               => None,
            }
        },
        Some(mut field) => {
            match command {
                Command::New(n) => Some(Field::new(n)),
                Command::Right  => {
                    field.swipe_right();
                    Some(field)
                },
                Command::Left  => {
                    field.swipe_left();
                    Some(field)
                },
                Command::Up  => {
                    field.swipe_up();
                    Some(field)
                },
                Command::Down  => {
                    field.swipe_down();
                    Some(field)
                },
            }
        },
    };
    
    let mut result_field = result_field.map(|mut f| {
        match tmp_field {
            None => {
                f.insert_random();
            },
            Some(field) => {
                if field != f {
                    f.insert_random();
                }
            },       
        }
        f
    });
        
                
       
    
    let s = print_result(&result_field);
    swap(&mut result_field, &mut field_option);
    s
}

fn print_result(field: &Option<Field>) -> String{
    match field {
        None => {
            "Empty\n".to_string()
        },

        Some(field) => {
            let mut string = String::new();
            for row in &field.rows {
                let s = format!("{:?}", row.row);
                string.push_str(&s);
                string.push_str(&";");
            }
            string.push_str(&"\n");
            string
        },
    }
}

