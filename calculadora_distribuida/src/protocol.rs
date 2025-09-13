use crate::operator::Operator;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    pub op: Operator,
    pub operand: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    Op(Operation),
    Get,
    Ok,
    Err(String),
    Value(i128),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Op(_) => write!(f, "OP ..."),
            Message::Get => write!(f, "GET"),
            Message::Ok => write!(f, "OK"),
            Message::Err(m) => write!(f, "ERROR \"{}\"", m),
            Message::Value(v) => write!(f, "VALUE {}", v),
        }
    }
}

/// Parsea una línea recibida (sin el '\n') como mensaje del protocolo.
/// Retorna Err(String) con motivo si no se puede parsear.
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
        // rest debe ser "<operador> <numero>"
        let parts: Vec<&str> = rest.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Formato de operacion invalido".to_string());
        }
        let op = Operator::from_str(parts[0]).ok_or_else(|| "Operacion invalida".to_string())?;
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
    } else if s.starts_with("ERROR ") {
        // ERROR "motivo"
        if let Some(start) = s.find('"') {
            if let Some(end) = s.rfind('"') {
                if end > start {
                    let motivo = &s[start + 1..end];
                    return Ok(Message::Err(motivo.to_string()));
                }
            }
        }
        Err("Formato ERROR invalido".to_string())
    } else if s.starts_with("VALUE ") {
        let val_str = s["VALUE ".len()..].trim();
        let v = val_str
            .parse::<i128>()
            .map_err(|_| "VALUE invalido".to_string())?;
        Ok(Message::Value(v))
    } else {
        Err("Mensaje desconocido".to_string())
    }
}
