use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use calculadora_distribuida::calculator;
use calculadora_distribuida::protocol::{Message, Operation, parse_message};

fn main() {
    if let Ok(address) = get_address()
        && let Ok(listener) = create_listener(&address)
    {
        run_server(listener);
    }
}

fn get_address() -> Result<String, ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("ERROR \"Se esperaba la direccion como argumento\"");
        return Err(());
    }

    Ok(String::from(&args[1]))
}

fn create_listener(address: &str) -> Result<TcpListener, String> {
    TcpListener::bind(address).map_err(|e| format!("ERROR \"No se pudo bindear: {}\"", e))
}

fn run_server(listener: TcpListener) {
    let state = Arc::new(Mutex::new(0i128));

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let st = Arc::clone(&state);
                thread::spawn(move || handle_connection(s, st));
            }
            Err(e) => eprintln!("ERROR \"{}\"", e),
        }
    }
}

fn handle_connection(stream: TcpStream, state: Arc<Mutex<i128>>) {
    let mut writer = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR \"{}\"", e);
            return;
        }
    };

    let reader = BufReader::new(stream);
    for line in reader.lines() {
        if let Err(e) = handle_line(&line, &state, &mut writer) {
            eprintln!("ERROR \"{}\"", e);
            break;
        }
    }
}

fn handle_line(
    line: &Result<String, std::io::Error>,
    state: &Arc<Mutex<i128>>,
    writer: &mut TcpStream,
) -> Result<(), String> {
    let l = line.as_ref().map_err(|e| e.to_string())?;
    match parse_message(l) {
        Ok(Message::Op(op)) => {
            let mut guard = lock_state(state, writer)?;
            apply_operation(op, &mut guard, writer)?;
        }
        Ok(Message::Get) => {
            let guard = lock_state(state, writer)?;
            send_value(&guard, writer);
        }
        Ok(_) => send_unexpected(writer),
        Err(e) => send_parse_error(&e, writer),
    }

    Ok(())
}

fn apply_operation(op: Operation, guard: &mut i128, writer: &mut TcpStream) -> Result<(), String> {
    match calculator::apply_operation(*guard, &op) {
        Ok(new_val) => {
            *guard = new_val;
            writer.write_all(b"OK\n").map_err(|e| e.to_string())?;
            Ok(())
        }
        Err(motivo) => {
            writer
                .write_all(format!("ERROR \"{}\"\n", motivo).as_bytes())
                .map_err(|e| e.to_string())?;
            Err(motivo)
        }
    }
}

fn send_value(guard: &i128, writer: &mut TcpStream) {
    let _ = writer.write_all(format!("VALUE {}\n", *guard).as_bytes());
}

fn send_parse_error(parse_err: &str, writer: &mut TcpStream) {
    let _ = writer.write_all(format!("ERROR \"{}\"\n", parse_err).as_bytes());
}

fn send_unexpected(writer: &mut TcpStream) {
    let _ = writer.write_all(b"ERROR \"unexpected message\"\n");
}

fn lock_state<'a>(
    state: &'a Arc<Mutex<i128>>,
    writer: &mut TcpStream,
) -> Result<std::sync::MutexGuard<'a, i128>, String> {
    state.lock().map_err(|_| {
        let _ = writer.write_all(b"ERROR \"Estado inaccesible\"\n");
        "Estado inaccesible".to_string()
    })
}
