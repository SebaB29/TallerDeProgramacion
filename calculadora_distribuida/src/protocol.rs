use crate::operator::Operator;
use std::fmt;

/// Representa una operación aritmética que se enviará al servidor.
#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    pub op: Operator,
    pub operand: u8,
}

/// Representa los distintos tipos de mensajes que pueden enviarse o recibirse.
#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    Op(Operation),
    Get,
    Ok,
    Err(String),
    Value(i128),
}

impl fmt::Display for Message {
    /// Convierte un `Message` en su representación textual.
    ///
    /// - `Message::Ok` → "OK"
    /// - `Message::Err(m)` → "ERROR \"m\""
    /// - `Message::Value(v)` → "VALUE v"
    /// - Otros mensajes no imprimen nada.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Ok => write!(f, "OK"),
            Message::Err(m) => write!(f, "ERROR \"{}\"", m),
            Message::Value(v) => write!(f, "VALUE {}", v),
            _ => Ok(()),
        }
    }
}

/// Parsea un mensaje textual recibido desde el servidor o cliente.
///
/// # Parámetros
/// - `line`: el mensaje como cadena de texto.
///
/// # Retorno
/// - `Ok(Message)` si el mensaje es válido.
/// - `Err(String)` si el mensaje no cumple con el formato esperado.
pub fn parse_message(line: &str) -> Result<Message, String> {
    let s = line.trim();
    if s.is_empty() {
        return Err("Empty message".to_string());
    }

    if s == "GET" {
        return Ok(Message::Get);
    }
    if s == "OK" {
        return Ok(Message::Ok);
    }
    if let Some(rest) = s.strip_prefix("OP ") {
        return parse_op(rest);
    }
    if let Some(rest) = s.strip_prefix("ERROR ") {
        return parse_error(rest);
    }
    if let Some(rest) = s.strip_prefix("VALUE ") {
        return parse_value(rest);
    }

    Err("Mensaje desconocido".to_string())
}

/// Parsea un mensaje de operación "OP <operador> <numero>".
fn parse_op(rest: &str) -> Result<Message, String> {
    let parts: Vec<&str> = rest.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Formato de operacion invalido".to_string());
    }

    let op = parts[0]
        .parse::<Operator>()
        .map_err(|_| "Operacion invalida".to_string())?;

    let num = parts[1]
        .parse::<u16>()
        .map_err(|_| "Numero invalido".to_string())?;
    if num > u8::MAX as u16 {
        return Err("Operando fuera de rango".to_string());
    }

    Ok(Message::Op(Operation {
        op,
        operand: num as u8,
    }))
}

/// Parsea un mensaje de error "ERROR \"motivo\"".
fn parse_error(rest: &str) -> Result<Message, String> {
    if let Some(start) = rest.find('"')
        && let Some(end) = rest.rfind('"')
        && end > start
    {
        let motivo = &rest[start + 1..end];
        return Ok(Message::Err(motivo.to_string()));
    }

    Err("Formato ERROR invalido".to_string())
}

/// Parsea un mensaje de valor "VALUE <numero>".
fn parse_value(rest: &str) -> Result<Message, String> {
    let val_str = rest.trim();
    let v = val_str
        .parse::<i128>()
        .map_err(|_| "VALUE invalido".to_string())?;
    Ok(Message::Value(v))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operator::Operator;

    #[test]
    fn test_parse_get() {
        assert_eq!(parse_message("GET").unwrap(), Message::Get);
    }

    #[test]
    fn test_parse_ok() {
        assert_eq!(parse_message("OK").unwrap(), Message::Ok);
    }

    #[test]
    fn test_parse_op() {
        let msg = parse_message("OP + 5").unwrap();
        assert_eq!(
            msg,
            Message::Op(Operation {
                op: Operator::Add,
                operand: 5
            })
        );
    }

    #[test]
    fn test_parse_op_invalid_operator() {
        assert!(parse_message("OP x 5").is_err());
    }

    #[test]
    fn test_parse_op_invalid_number() {
        assert!(parse_message("OP + 300").is_err());
        assert!(parse_message("OP + abc").is_err());
    }

    #[test]
    fn test_parse_error() {
        let msg = parse_message(r#"ERROR "Algo fallo""#).unwrap();
        assert_eq!(msg, Message::Err("Algo fallo".to_string()));
    }

    #[test]
    fn test_parse_value() {
        let msg = parse_message("VALUE 123").unwrap();
        assert_eq!(msg, Message::Value(123));
    }

    #[test]
    fn test_parse_unknown() {
        assert!(parse_message("XYZ").is_err());
        assert!(parse_message("").is_err());
    }

    #[test]
    fn test_display() {
        let ok = Message::Ok;
        let err = Message::Err("fail".to_string());
        let val = Message::Value(42);

        assert_eq!(ok.to_string(), "OK");
        assert_eq!(err.to_string(), "ERROR \"fail\"");
        assert_eq!(val.to_string(), "VALUE 42");
    }
}
