use std::{io::stdin, process};
use super::*;

pub fn uci_loop() {
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let command = &mut buf.as_str().trim();

        let cmd_name = match take_next(command) {
            Some(name) => name,
            None => continue, // Empty command
        };

        match cmd_name {
            "uci" => {
                println!("name {PKG_NAME} v{PKG_VERSION}");
                println!("author {PKG_AUTHORS}");

                // Advertise options

                println!("uciok")
            },
            "setoption" => {
                todo!()
            },
            "isready" => {
                println!("readyok")
            },
            "ucinewgame" => {
                todo!()
            },
            "position " => {
                todo!()
            },
            "go" => parse_go(command),
            "stop" => {
                todo!()
            },
            "ponderhit" => {
                todo!()
            },
            "quit" => {
                process::exit(0)
            },
            _ => println!("Unknown command")
        }
    }
}

fn take_next<'a>(command: &'a mut &str) -> Option<&'a str> {
    if command.is_empty() {
        return None
    }

    let (next, rest) = command.split_once(" ").unwrap_or_else(|| {(command, "")});

    let rest = rest.trim();

    *command = rest;

    Some(next)
}

fn parse_go(command: &mut &str) {
    
}