use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use calculadora_distribuida::protocol::{Message, parse_message};

/// Punto de entrada del cliente.
/// Ejecuta el cliente y maneja errores generales.
fn main() {
    if let Err(e) = run_client() {
        eprintln!("{}", e);
    }
}

/// Ejecuta la lógica principal del cliente.
///
/// - Inicializa la conexión con el servidor y abre el archivo de operaciones.
/// - Envía todas las operaciones del archivo al servidor.
/// - Solicita el valor final al servidor y lo imprime.
///
/// Retorna `Ok(())` si todo fue exitoso, o `Err(String)` con un mensaje de error.
fn run_client() -> Result<(), String> {
    let (mut stream, file) = init_client()?;
    read_file(file, &mut stream)?;
    get_final_value(&mut stream)?;
    Ok(())
}

/// Inicializa la conexión con el servidor y abre el archivo de operaciones.
///
/// Retorna un tuple `(TcpStream, File)` si tiene éxito, o `Err(String)` con
/// un mensaje de error descriptivo.
///
/// # Errores
/// - Si no se reciben los argumentos correctos.
/// - Si no se puede conectar al servidor.
/// - Si no se puede abrir el archivo.
fn init_client() -> Result<(TcpStream, File), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("Se esperaba direccion y archivo como argumentos".to_string());
    }

    let stream = TcpStream::connect(&args[1]).map_err(|e| format!("No se pudo conectar: {}", e))?;
    let file = File::open(&args[2]).map_err(|e| format!("No se pudo abrir el archivo: {}", e))?;
    Ok((stream, file))
}

/// Lee cada línea del archivo y la envía al servidor como operación.
///
/// Ignora líneas vacías.
///
/// # Errores
/// Retorna `Err(String)` si ocurre un error al leer el archivo o al enviar
/// la operación al servidor.
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

/// Envía una operación al servidor.
///
/// # Parámetros
/// - `s`: operación en formato `<operador> <numero>`.
/// - `stream`: stream TCP conectado al servidor.
///
/// # Errores
/// Retorna `Err(String)` si ocurre un error al enviar los datos.
fn send_operation(s: &str, stream: &mut TcpStream) -> Result<(), String> {
    let payload = format!("OP {}\n", s);
    stream
        .write_all(payload.as_bytes())
        .map_err(|e| format!("Error enviando: {}", e))
}

/// Lee la respuesta inmediata del servidor tras una operación.
///
/// - Si la respuesta es `OK`, no hace nada.
/// - Si la respuesta es `ERROR`, lo muestra por `stderr`.
/// - Si la respuesta es inesperada o no se puede parsear, también lo muestra por `stderr`.
///
/// # Errores
/// Retorna `Err(String)` solo si ocurre un error de E/S al leer la respuesta.
/// Los errores reportados por el servidor **no interrumpen la ejecución**
/// y se imprimen por `stderr`.
fn read_answer(stream: &mut TcpStream) -> Result<(), String> {
    let mut resp = String::new();
    let mut buff = BufReader::new(stream);
    buff.read_line(&mut resp)
        .map_err(|e| format!("Error leyendo respuesta: {}", e))?;

    match parse_message(resp.trim_end()) {
        Ok(Message::Ok) => { /* no hacer nada, todo bien */ }
        Ok(Message::Err(m)) => eprintln!("ERROR \"{}\"", m),
        Ok(other) => eprintln!("ERROR \"Respuesta inesperada: {}\"", other),
        Err(e) => eprintln!("ERROR \"{}\"", e),
    }

    Ok(())
}

/// Solicita el valor final al servidor y lo imprime.
///
/// # Errores
/// Retorna `Err(String)` si ocurre un error al enviar el mensaje GET o
/// al leer la respuesta.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    /// Levanta un servidor TCP que responde con el mensaje dado
    fn start_mock_server(response: &'static str) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap().to_string();

        thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut reader = BufReader::new(&mut stream);
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                let _ = stream.write_all(response.as_bytes());
            }
        });

        addr
    }

    #[test]
    fn test_send_operation_and_read_answer() {
        let addr = start_mock_server("OK\n");

        let mut stream = TcpStream::connect(addr).unwrap();
        send_operation("+ 1", &mut stream).unwrap();
        read_answer(&mut stream).unwrap();
    }

    #[test]
    fn test_get_final_value_value() {
        let addr = start_mock_server("VALUE 42\n");
        let mut stream = TcpStream::connect(addr).unwrap();
        get_final_value(&mut stream).unwrap();
    }

    #[test]
    fn test_get_final_value_error() {
        let addr = start_mock_server("ERROR \"Operacion invalida\"\n");
        let mut stream = TcpStream::connect(addr).unwrap();
        get_final_value(&mut stream).unwrap();
    }
}
