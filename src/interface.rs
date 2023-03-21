use std::{io::stdin, process, thread, sync::mpsc::{channel, Receiver}, time::Instant};
use super::*;

pub fn interface_loop() {
    let mut pos = Position::start_pos();
    
    // Spawn listening thread that reads input without blocking main thread
    let ui_receiver = spawn_ui_listener_thread();

    loop {
        let line = ui_receiver.recv().expect("Error receiving ui command!");
        let mut command = line.as_str();

        let cmd_name = match take_next(&mut command) {
            Some(name) => name,
            None => continue, // Empty command
        };

        match cmd_name {
            // Cadabra specific commands
            "help" => {
                todo!()
            },
            "d" => {
                println!("{}", pos);
            },
            "fen" => {
                println!("{}", pos.fen_string());
            },
            "x" => { // "x" added for conveniece. Does the same as UCI's quit
                quit()
            },
            "move" => {
                let moov = match take_next(&mut command) {
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
            "eval" => {
                println!("Heuristic value: {}", pos.evaluate())
            },
            "zobrist" => {
                println!("Zobrist hash:: {:x}", pos.zobrist_hash)
            }
            "perft" => {
                parse_perft(&mut command, &pos)
            },
            "bench" => {
                match take_next(&mut command) {
                    Some("save") => run_bench(true),
                    None => run_bench(false),
                    Some(arg) => println!("Illegal parameter for bench '{arg}'. Only 'save' is supported"),
                }
            }


            // UCI commands
            "uci" => {
                println!("name {PKG_NAME} {PKG_VERSION}");
                println!("author {PKG_AUTHORS}");

                // Advertise options

                println!("uciok")
            }
            "setoption" => {
                todo!()
            }
            "isready" => {
                println!("readyok")
            }
            "ucinewgame" => {
                todo!()
            }
            "position" => {
                match parse_position(&mut command) {
                    Ok(res) => pos = res,
                    Err(err) => println!("{err}"),
                }
            }
            "go" => {
                parse_go(&mut command, &pos)
            },
            "stop" => {
                todo!()
            },
            "ponderhit" => {
                todo!()
            },
            "quit" => {
                quit()
            },
            "debug" => {
                match take_next(&mut command) {
                    Some("on") => run_bench(true),
                    Some("off") => run_bench(false),
                    _ => println!("Debug can be 'on' or 'off'"),
                }
            }

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

pub fn take_next_u8<'a>(command: &'a mut &str) -> Option<u8> {
    let depth_str = match take_next(command) {
        None => {
            return None
        },
        Some(depth) => {
            depth
        },
    };

    match depth_str.parse::<u8>() {
        Ok(depth) => Some(depth),
        Err(_) => {
            None
        },
    }
}

fn quit() {
    process::exit(0)
}

fn parse_position(command: &mut &str) -> Result<Position, String> {
    match take_next(command) {
        Some("startpos") => Ok(Position::start_pos()),
        Some("fen") => {
            todo!()
        },
        _ => Err(format!("Illegal position command"))
    }
}

fn parse_go(command: &mut &str, _pos: &Position) {
    match take_next(command) {
        Some("depth") => {
            let _depth = match take_next_u8(command) {
                Some(d) => d,
                None => {
                    println!("Illegal go command");
                    return
                },
            };

            //search(pos, depth)
        },
        _ => println!("Illegal go command")
    }
}

fn parse_perft(command: &mut &str, pos: &Position) {
    let depth = match take_next_u8(command) {
        Some(d) => d,
        None => {
            println!("Illegal perft command");
            return
        },
    };

    let before = Instant::now();

    println!("\n Found: {} moves at depth {depth} in {}ms\n", pos.perft::<true>(depth), before.elapsed().as_millis())
}