use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use calculadora_distribuida::protocol::{Message, parse_message};

fn main() {
    if let Err(e) = run_client() {
        eprintln!("{}", e);
    }
}

fn run_client() -> Result<(), String> {
    let (mut stream, file) = init_client()?;
    read_file(file, &mut stream)?;
    get_final_value(&mut stream)?;
    Ok(())
}

fn init_client() -> Result<(TcpStream, File), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("Se esperaba direccion y archivo como argumentos".to_string());
    }

    let stream = TcpStream::connect(&args[1]).map_err(|e| format!("No se pudo conectar: {}", e))?;
    let file = File::open(&args[2]).map_err(|e| format!("No se pudo abrir el archivo: {}", e))?;
    Ok((stream, file))
}

fn read_file(file: File, stream: &mut TcpStream) -> Result<(), String> {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.map_err(|e| format!("Error leyendo archivo: {}", e))?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        send_operation(line, stream)?;
        read_answer(stream)?;
    }

    Ok(())
}

fn send_operation(s: &str, stream: &mut TcpStream) -> Result<(), String> {
    let payload = format!("OP {}\n", s);
    stream
        .write_all(payload.as_bytes())
        .map_err(|e| format!("Error enviando: {}", e))
}

fn read_answer(stream: &mut TcpStream) -> Result<(), String> {
    let mut resp = String::new();
    let mut buff = BufReader::new(stream);
    buff.read_line(&mut resp)
        .map_err(|e| format!("Error leyendo respuesta: {}", e))?;

    Ok(())
}

fn get_final_value(stream: &mut TcpStream) -> Result<(), String> {
    stream
        .write_all(b"GET\n")
        .map_err(|e| format!("Error enviando GET: {}", e))?;

    let mut reader = BufReader::new(stream);
    let mut resp = String::new();
    reader
        .read_line(&mut resp)
        .map_err(|e| format!("Error leyendo VALUE: {}", e))?;

    match parse_message(resp.trim_end()) {
        Ok(Message::Value(v)) => println!("{}", v),
        Ok(Message::Err(m)) => eprintln!("ERROR \"{}\"", m),
        Ok(other) => eprintln!("ERROR \"Respuesta inesperada: {}\"", other),
        Err(e) => eprintln!("ERROR \"{}\"", e),
    }

    Ok(())
}
