use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use calculadora_distribuida::calculator;
use calculadora_distribuida::protocol::{Message, Operation, parse_message};

fn main() {
    let address = match get_address() {
        Ok(addr) => addr,
        Err(_) => return,
    };

    let listener = match create_listener(&address) {
        Ok(l) => l,
        Err(_) => return,
    };

    run_server(listener);
}

fn get_address() -> Result<String, ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("ERROR \"Se esperaba la direccion como argumento\"");
        return Err(());
    }
    Ok(String::from(&args[1]))
}

fn create_listener(address: &str) -> Result<TcpListener, ()> {
    match TcpListener::bind(address) {
        Ok(l) => Ok(l),
        Err(e) => {
            eprintln!("ERROR \"No se pudo bindear: {}\"", e);
            Err(())
        }
    }
}

fn run_server(listener: TcpListener) {
    let state = Arc::new(Mutex::new(0i128));

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let st = Arc::clone(&state);
                thread::spawn(move || handle_connection(s, st));
            }
            Err(e) => {
                eprintln!("ERROR \"{}\"", e);
            }
        }
    }
}

fn handle_connection(stream: TcpStream, state: Arc<Mutex<i128>>) {
    let writer = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR \"{}\"", e);
            return;
        }
    };
    let mut writer = writer;
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(l) => {
                parse_line(&l, &state, &mut writer);
            }
            Err(e) => {
                eprintln!("ERROR \"{}\"", e);
                return;
            }
        }
    }
}

fn parse_line(l: &str, state: &Arc<Mutex<i128>>, writer: &mut TcpStream) {
    match parse_message(&l) {
        Ok(Message::Op(op)) => {
            let mut guard = match state.lock() {
                Ok(g) => g,
                Err(_) => {
                    let _ = writer.write_all(b"ERROR \"Estado inaccesible\"\n");
                    return;
                }
            };
            apply_operation(op, &mut guard, writer);
        }
        Ok(Message::Get) => {
            let guard = match state.lock() {
                Ok(g) => g,
                Err(_) => {
                    let _ = writer.write_all(b"ERROR \"Estado inaccesible\"\n");
                    return;
                }
            };
            let msg = format!("VALUE {}\n", *guard);
            if let Err(e) = writer.write_all(msg.as_bytes()) {
                eprintln!("ERROR \"{}\"", e);
                return;
            }
        }
        Ok(_) => {
            // OK / VALUE / ERROR no esperados como input por cliente; enviar error
            let _ = writer.write_all(b"ERROR \"unexpected message\"");
        }
        Err(parse_err) => {
            let msg = format!("ERROR \"{}\"\n", parse_err);
            if let Err(e) = writer.write_all(msg.as_bytes()) {
                eprintln!("ERROR \"{}\"", e);
                return;
            }
        }
    }
}

fn apply_operation(op: Operation, guard: &mut i128, writer: &mut TcpStream) {
    match calculator::apply_operation(*guard, &op) {
        Ok(new_val) => {
            *guard = new_val;
            if let Err(e) = writer.write_all(b"OK\n") {
                eprintln!("ERROR \"{}\"", e);
                return;
            }
        }
        Err(motivo) => {
            let msg = format!("ERROR \"{}\"\n", motivo);
            if let Err(e) = writer.write_all(msg.as_bytes()) {
                eprintln!("ERROR \"{}\"", e);
                return;
            }
        }
    }
}
