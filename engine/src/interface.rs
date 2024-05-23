use std::{io::stdin, process, thread, sync::mpsc::{channel, Receiver}, time::Instant, str::FromStr};

use super::*;

pub fn interface_loop() {
    let mut pos = Position::start_pos();
    
    // Spawn listening thread that reads input without blocking main thread
    let ui_receiver = spawn_ui_listener_thread();

    let mut settings = Settings::default();

    let mut current_search: Search = Search::new(settings);

    loop {
        let line = wait_for_input(&ui_receiver);
        let mut command = line.as_str().trim();

        let cmd_name = match take_next(&mut command) {
            Some(name) => name,
            None => continue, // Empty command
        };

        match cmd_name {
            // UCI commands
            "uci" => {
                println!("id name {PKG_NAME} {PKG_VERSION}");
                println!("id author {PKG_AUTHORS}");
                println!();

                // Advertise options
                println!("option name Hash type spin default 16 min 1 max 1048576");
                println!("option name Threads type spin default 1 min 1 max 255");
                println!("option name Clear Hash type button");

                // Apply modified settings
                current_search = Search::new(settings);

                println!("uciok")
            },
            "setoption" => {
                if current_search.is_running() {
                    println!("Cannot change options while a search is running");
                    continue;
                }

                if command == "name Clear Hash" {
                    current_search.tt.clear();
                    continue;
                }

                match parse_set_option(&mut command, settings) {
                    Ok(n_settings) => {
                        settings = n_settings;
                        current_search.update_settings(settings)
                    },
                    Err(err) => println!("{err}"),
                }
            },
            "isready" => {
                println!("readyok")
            },
            "ucinewgame" => {
                if current_search.is_running() {
                    println!("Cannot start a new game while a search is running");
                }
                current_search.tt.clear()
            },
            "position" => {
                match parse_position(&mut command) {
                    Ok(n_pos) => pos = n_pos,
                    Err(err) => println!("{err}"),
                }
            },
            "go" => {
                if current_search.is_running() {
                    println!("A search is already running");
                    continue
                }

                let meta = match parse_go(&mut command, pos) {
                    Ok(c) => c,
                    Err(err) => {
                        println!("{err}");
                        continue
                    },
                };

                let search = current_search.clone();
                thread::spawn(move || {
                    search.start(pos, meta, true, CONST_EVALUATOR);
                });
            },
            "stop" => {
                current_search.stop()
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
            "x" => { // "x" added for conveniece. Does the same as UCI's 'quit'
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
                println!("Heuristic value: {}", pos.evaluate(CONST_EVALUATOR))
            },
            "threefold" => {
                println!("{}", pos.rep_table.is_in_3_fold_rep(&pos))
            },
            "insufficient" => {
                println!("{}", pos.is_insufficient_material())
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
            },
            "cleartt" => {
                current_search.tt.clear();
            },
            "fillrate" => {
                println!("Fill rate: {:.2}%", current_search.tt.fill_rate() * 100.0)
            },
            
            _ => println!("Unknown command '{cmd_name}', use 'help' command for all commands")
        }
    }
}

fn parse_set_option(mut command: &str, mut settings: Settings) -> Result<Settings, String> {
    match take_next(&mut command) {
        Some("name") => (),
        _ => return Err("Expected 'name' in command after 'setoption'".to_string()),
    }
    match take_next(&mut command) {
        Some("Hash") => match take_next(&mut command) {
            Some("value") => match take_next_num(&mut command) {
                Some(megabytes) => {
                    if (megabytes & (megabytes - 1)) != 0 || megabytes < 1 {
                        return Err("Transposition size must be a positive power of 2".to_string());
                    }
                    settings.transposition_table_mb = megabytes;
                },
                _ => return Err("No value provided for Hash option".to_string())
            },
            _ => return Err("Expected 'value' after 'Hash'".to_string())
        },
        Some("Threads") => match take_next(&mut command) {
            Some("value") => match take_next_num(&mut command) {
                Some(threads) => {
                    if threads < 1 {
                        return Err("Threads must be at least 1".to_string());
                    }
                    settings.threads = threads;
                },
                _ => return Err("No value provided for Threads option".to_string())
            },
            _ => return Err("Expected 'value' after 'Threads'".to_string())
        },
        Some(unknown) => return Err(format!("Unknown option name '{unknown}'")),
        None => return Err("No option name provided".to_string()),
    }

    Ok(settings)
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

    let (next, rest) = command.split_once(' ').unwrap_or((command, ""));

    let rest = rest.trim();

    *command = rest;

    Some(next)
}

pub fn take_next_num<T: FromStr>(command: &mut &str) -> Option<T> {
    let depth_str = match take_next(command) {
        None => {
            return None
        },
        Some(depth) => {
            depth
        },
    };

    match depth_str.parse::<T>() {
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

fn parse_go(command: &mut &str, pos: Position) -> Result<SearchArgs, String> {
    if command == &"" {
        return Err("Empty go command".to_string())
    }

    let mut max_depth: Option<u8> = None;
    let mut ponder = false;
    let mut infinite = false;
    let mut time: Option<u128> = None;
    let mut inc: Option<u128> = None;
    let mut movestogo: Option<u8> = None;
    let mut nodes: Option<u128> = None;
    let mut movetime: Option<u128> = None;

    while let Some(arg) = take_next(command) {
        match arg {
            "depth" => {
                max_depth = match take_next_num(command) {
                    Some(d) => Some(d),
                    None => {
                        return Err(format!("Illegal go depth"));
                    },
                };
            },

            "ponder" => ponder = true,
            "infinite" => infinite = true,

            "wtime" => match take_next_num(command) {
                Some(t) => if pos.active_color.is_white() {
                    time = Some(t);
                },
                None => {
                    return Err(format!("Illegal go wtime"));
                }
            },
            "btime" => match take_next_num(command) {
                Some(t) => if pos.active_color.is_black() {
                    time = Some(t);
                },
                None => {
                    return Err(format!("Illegal go btime"));
                }
            },
            "winc" => match take_next_num(command) {
                Some(i) => if pos.active_color.is_white() {
                    inc = Some(i);
                },
                None => {
                    return Err(format!("Illegal go winc"));
                }
            },
            "binc" => match take_next_num(command) {
                Some(i) => if pos.active_color.is_black() {
                    inc = Some(i);
                },
                None => {
                    return Err(format!("Illegal go binc"));
                }
            },

            "movestogo" => movestogo = match take_next_num(command) {
                Some(m) => Some(m),
                None => {
                    return Err(format!("Illegal go movestogo"));
                },
            },
            "mate" => unimplemented!(),
            "movetime" => movetime = match take_next_num(command) {
                Some(m) => Some(m),
                None => {
                    return Err(format!("Illegal go movetime"));
                },
            },
            "nodes" => nodes = match take_next_num(command) {
                Some(n) => Some(n),
                None => {
                    return Err(format!("Illegal go nodes"));
                },
            },
            
            _ => return Err(format!("Illegal go argument: {arg}"))
        }
    }

    SearchArgs::new(max_depth, ponder, infinite, time, inc, movestogo, nodes, movetime)
}

fn parse_perft(command: &mut &str, pos: &Position) {
    let depth = match take_next_num(command) {
        Some(d) => d,
        None => {
            println!("Illegal perft command");
            return
        },
    };

    let before = Instant::now();
    println!("\n Found: {} moves at depth {depth} in {}ms\n", pos.perft::<true>(depth), before.elapsed().as_millis())
}