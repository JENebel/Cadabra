use std::{io::stdin, process, thread, sync::mpsc::{channel, Receiver}, time::Instant};
use super::*;

pub fn interface_loop() {
    let mut pos = Position::start_pos();
    
    // Spawn listening thread that reads input without blocking main thread
    let ui_receiver = spawn_ui_listener_thread();

    loop {
        let line = ui_receiver.recv().expect("Error reading command!");
        let command = &mut line.as_str();

        let cmd_name = match take_next(command) {
            Some(name) => name,
            None => continue, // Empty command
        };

        match cmd_name {
            // Cadabra commands
            "d" => {
                pos.pretty_print();
            },
            "x" => { // "x" added for conveniece. Does the same as UCI's quit
                quit()
            },
            "move" => {
                let moov = match take_next(command) {
                    Some(m) => m,
                    None => {
                        println!("Provide a move to make");
                        continue
                    }
                };

                if let Err(err) = pos.make_uci_move(moov) {
                    println!("{err}")
                }
            },
            "perft" => parse_perft(command, &pos),
            "bench" => {
                match take_next(command) {
                    Some("save") => run_bench(true),
                    None => run_bench(false),
                    Some(arg) => println!("Illegal parameter for benhc '{arg}'"),
                }
            },


            // UCI commands
            "uci" => {
                println!("name {PKG_NAME} {PKG_VERSION}");
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
            "go" => todo!(),//parse_go(command, &pos),
            "stop" => {
                todo!()
            },
            "ponderhit" => {
                todo!()
            },
            "quit" => {
                quit()
            },

            _ => println!("Unknown command '{cmd_name}', use 'help' command for all commands")
        }
    }
}

pub fn spawn_ui_listener_thread() -> Receiver<String> {
    let (sender, ui_receiver) = channel::<String>();

    // Spawn listening thread that reads input without blocking main thread
    thread::spawn(move || {
        loop {
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            sender.send(buf.trim().to_string()).unwrap()
        }
    });

    ui_receiver
}

pub fn take_next<'a>(command: &'a mut &str) -> Option<&'a str> {
    if command.is_empty() {
        return None
    }

    let (next, rest) = command.split_once(" ").unwrap_or_else(|| {(command, "")});

    let rest = rest.trim();

    *command = rest;

    Some(next)
}

fn quit() {
    process::exit(0)
}

fn parse_perft(command: &mut &str, pos: &Position) {
    let depth_str = match take_next(command) {
        None => {
            println!("Please provide a depth");
            return
        },
        Some(depth) => {
            depth
        },
    };

    let depth = match depth_str.parse::<u8>() {
        Ok(depth) => depth,
        Err(_) => {
            println!("Depth arg must be an integer in [0..255], got '{depth_str}'");
            return
        },
    };

    let before = Instant::now();

    println!("\n Found: {} moves at depth {depth} in {}ms\n", pos.perft::<true>(depth), before.elapsed().as_millis())
}