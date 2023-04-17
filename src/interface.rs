use std::{io::stdin, process, thread, sync::{mpsc::{channel, Receiver}}, time::Instant};

use super::*;

pub fn interface_loop() {
    let mut pos = Position::start_pos();
    
    // Spawn listening thread that reads input without blocking main thread
    let ui_receiver = spawn_ui_listener_thread();

    let settings = Settings::default();

    let current_search: Search = Search::new(settings);

    loop {
        let line = wait_for_input(&ui_receiver);
        let mut command = line.as_str().trim();

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
                parse_perft(&mut command, &pos);
            },
            "bench" => {
                match take_next(&mut command) {
                    Some("save") => run_bench(true),
                    None => run_bench(false),
                    Some(arg) => println!("Illegal parameter for bench '{arg}'. Only 'save' is supported"),
                }
            },
            "legal" => {
                for m in pos.generate_moves() {
                    println!(" {m}")
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
                current_search.new_game()
            }
            "position" => {
                match parse_position(&mut command) {
                    Ok(res) => pos = res,
                    Err(err) => println!("{err}"),
                }
            }
            "go" => {
                if current_search.is_running() {
                    println!("A search is already running");
                    continue
                }

                let context = match parse_go(&mut command) {
                    Ok(c) => c,
                    Err(err) => {
                        println!("{err}");
                        continue
                    },
                };

                current_search.start(pos, context);
            },
            "stop" => {
                current_search.stop(false)
            },
            "ponderhit" => {
                todo!()
            },
            "quit" => {
                quit()
            },
            "debug" => {
                match take_next(&mut command) {
                    Some("on") => println!("Debug setting does nothing"),
                    Some("off") => println!("Debug setting does nothing"),
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

fn wait_for_input(ui_receiver: &Receiver<String>) -> String {
    ui_receiver.recv().expect("Error receiving ui command!")
}

fn parse_position(command: &mut &str) -> Result<Position, String> {
    let mut split = command.split("moves");
    let mut pos_str = match split.next() {
        Some(pos_str) => pos_str.trim(),
        None => return Err(format!("No argument provided for position command")),
    };

    let mut pos = match take_next(&mut pos_str) {
        Some("startpos") => Position::start_pos(),
        Some("fen") => Position::from_fen(pos_str)?,
        _ => return Err(format!("Illegal position argument"))
    };

    if let Some(mut move_args) = split.next() {
        move_args = move_args.trim();
        while let Some(moov) = take_next(&mut move_args) {
            pos.make_uci_move(moov)?;
        }
    }

    Ok(pos)
}

fn parse_go(command: &mut &str) -> Result<SearchMeta, String> {
    match take_next(command) {
        Some("depth") => {
            let depth = match take_next_u8(command) {
                Some(d) => d,
                None => {
                    return Err(format!("Illegal go depth"));
                },
            };

            Ok(SearchMeta::new(depth))
        },
        _ => Err(format!("Illegal go argument"))
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