use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use calculadora_distribuida::errors;
use calculadora_distribuida::protocol::{Message, parse_message};

fn main() {
    let (mut stream, file) = match init_client() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    read_file(file, &mut stream);

    // Finalmente pedir GET
    if let Err(e) = stream.write_all(b"GET\n") {
        errors::print_fatal_and_exit(&format!("Error enviando GET: {}", e));
        return;
    }

    let mut reader = BufReader::new(stream);
    let mut resp = String::new();
    if let Err(e) = reader.read_line(&mut resp) {
        errors::print_fatal_and_exit(&format!("Error leyendo VALUE: {}", e));
        return;
    }
    // Mostrar el valor al finalizar la ejecución (por stdout)
    match parse_message(resp.trim_end()) {
        Ok(Message::Value(v)) => println!("{}", v),
        Ok(Message::Err(m)) => eprintln!("ERROR \"{}\"", m),
        Ok(other) => {
            // formato no esperado
            eprintln!("ERROR \"Respuesta inesperada: {}\"", other);
        }
        Err(e) => {
            eprintln!("ERROR \"{}\"", e);
        }
    }
}

fn init_client() -> Result<(TcpStream, File), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("Se esperaba direccion y archivo como argumentos".to_string());
    }

    let addr = &args[1];
    let file_path = &args[2];
    let stream = TcpStream::connect(addr).map_err(|e| format!("No se pudo conectar: {}", e))?;
    let file = File::open(file_path).map_err(|e| format!("No se pudo abrir el archivo: {}", e))?;

    Ok((stream, file))
}

fn read_file(file: File, stream: &mut TcpStream) {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let s = l.trim();
                if s.is_empty() {
                    continue;
                }
                send_operation(s, stream);
                read_answer(stream);
            }
            Err(e) => {
                eprintln!("Error leyendo archivo: {}", e);
                return;
            }
        }
    }
}

fn send_operation(s: &str, stream: &mut TcpStream) {
    let payload = format!("OP {}\n", s);
    if let Err(e) = stream.write_all(payload.as_bytes()) {
        errors::print_fatal_and_exit(&format!("Error enviando: {}", e));
        return;
    }
}

fn read_answer(stream: &mut TcpStream) {
    let mut resp = String::new();
    let mut buff = BufReader::new(stream.try_clone().unwrap());
    if let Err(e) = buff.read_line(&mut resp) {
        errors::print_fatal_and_exit(&format!("Error leyendo respuesta: {}", e));
        return;
    }
}
