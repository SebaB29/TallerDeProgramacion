use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use calculadora_distribuida::calculator;
use calculadora_distribuida::protocol::{Message, Operation, parse_message};

/// Punto de entrada del servidor.
///
/// Obtiene la dirección desde los argumentos de línea de comando y ejecuta
/// el servidor TCP.
fn main() {
    if let Ok(address) = get_address()
        && let Ok(listener) = create_listener(&address)
    {
        run_server(listener);
    }
}

/// Obtiene la dirección del servidor desde los argumentos de línea de comando.
///
/// Retorna `Ok(String)` con la dirección o `Err(())` si no se proporcionó.
fn get_address() -> Result<String, ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("ERROR \"Se esperaba la direccion como argumento\"");
        return Err(());
    }

    Ok(String::from(&args[1]))
}

/// Crea un `TcpListener` en la dirección proporcionada.
///
/// # Errores
/// Retorna `Err(String)` si no se puede bindear la dirección.
fn create_listener(address: &str) -> Result<TcpListener, String> {
    TcpListener::bind(address).map_err(|e| format!("ERROR \"No se pudo bindear: {}\"", e))
}

/// Ejecuta el bucle principal del servidor.
///
/// - Acepta conexiones entrantes.
/// - Para cada conexión, crea un hilo que maneja el cliente.
/// - Mantiene un estado compartido seguro entre hilos.
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

/// Maneja una conexión individual de cliente.
///
/// - Lee líneas enviadas por el cliente.
/// - Procesa cada línea usando `handle_line`.
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
        }
    }
}

/// Procesa una línea recibida del cliente.
///
/// - `line`: línea recibida del cliente.
/// - `state`: referencia al estado compartido entre clientes.
/// - `writer`: stream para responder al cliente.
///
/// Retorna `Ok(())` si se procesó correctamente o `Err(String)` si hubo error.
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
        Err(_) => send_parse_error("parsing error", writer),
    }

    Ok(())
}

/// Aplica una operación matemática sobre el estado.
///
/// - `op`: operación a aplicar.
/// - `guard`: referencia mutable al estado.
/// - `writer`: stream para enviar la respuesta al cliente.
///
/// Retorna `Ok(())` si se aplicó con éxito o `Err(String)` si hubo error.
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

/// Envía el valor actual del estado al cliente.
fn send_value(guard: &i128, writer: &mut TcpStream) {
    let _ = writer.write_all(format!("VALUE {}\n", *guard).as_bytes());
}

/// Envía un mensaje de error de parseo al cliente.
fn send_parse_error(parse_err: &str, writer: &mut TcpStream) {
    let _ = writer.write_all(format!("ERROR \"{}\"\n", parse_err).as_bytes());
    eprintln!("ERROR \"{}\"", parse_err);
}

/// Envía un mensaje de error por mensaje inesperado.
fn send_unexpected(writer: &mut TcpStream) {
    let _ = writer.write_all(b"ERROR \"unexpected message\"\n");
}

/// Bloquea el estado compartido para su uso seguro.
///
/// Retorna un `MutexGuard` sobre el estado o `Err(String)` si no se puede acceder.
fn lock_state<'a>(
    state: &'a Arc<Mutex<i128>>,
    writer: &mut TcpStream,
) -> Result<std::sync::MutexGuard<'a, i128>, String> {
    state.lock().map_err(|_| {
        let _ = writer.write_all(b"ERROR \"Estado inaccesible\"\n");
        "Estado inaccesible".to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    /// Arranca el servidor en un hilo y devuelve la dirección
    fn start_server() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap(); // puerto aleatorio
        let addr = listener.local_addr().unwrap().to_string();

        thread::spawn(move || {
            run_server(listener);
        });

        // Pequeña pausa para que el servidor esté escuchando
        thread::sleep(Duration::from_millis(50));
        addr
    }

    #[test]
    fn test_server_ok_response() {
        let addr = start_server();
        let mut stream = TcpStream::connect(addr).unwrap();

        // Enviar operación válida
        stream.write_all(b"OP + 10\n").unwrap();

        let mut reader = BufReader::new(&mut stream);
        let mut response = String::new();
        reader.read_line(&mut response).unwrap();

        assert_eq!(response.trim(), "OK");
    }

    #[test]
    fn test_server_value_response() {
        let addr = start_server();
        let mut stream = TcpStream::connect(addr).unwrap();
        let mut reader = BufReader::new(&mut stream);

        // Enviar operación
        reader.get_mut().write_all(b"OP + 5\n").unwrap();
        let mut resp1 = String::new();
        reader.read_line(&mut resp1).unwrap();
        assert_eq!(resp1.trim(), "OK");

        // Enviar GET usando la misma referencia
        reader.get_mut().write_all(b"GET\n").unwrap();
        let mut resp2 = String::new();
        reader.read_line(&mut resp2).unwrap();
        assert_eq!(resp2.trim(), "VALUE 5");
    }

    #[test]
    fn test_server_error_response() {
        let addr = start_server();
        let mut stream = TcpStream::connect(addr).unwrap();

        // Enviar operación inválida (división por cero)
        stream.write_all(b"OP / 0\n").unwrap();

        let mut reader = BufReader::new(&mut stream);
        let mut response = String::new();
        reader.read_line(&mut response).unwrap();

        assert!(response.trim().starts_with("ERROR"));
    }
}
