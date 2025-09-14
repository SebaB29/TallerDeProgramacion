use std::str::FromStr;

/// Representa los operadores aritméticos soportados por la calculadora.
#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl FromStr for Operator {
    type Err = String;

    /// Convierte una cadena en un `Operator`.
    ///
    /// # Parámetros
    /// - `s`: una cadena que representa el operador (`"+", "-", "*", "/"`).
    ///
    /// # Retorna
    /// - `Ok(Operator)` si la cadena es válida.
    /// - `Err(String)` si la cadena no corresponde a ningún operador.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Mul),
            "/" => Ok(Operator::Div),
            _ => Err(format!("Operacion invalida: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_add() {
        assert_eq!("+".parse::<Operator>().unwrap(), Operator::Add);
    }

    #[test]
    fn test_parse_sub() {
        assert_eq!("-".parse::<Operator>().unwrap(), Operator::Sub);
    }

    #[test]
    fn test_parse_mul() {
        assert_eq!("*".parse::<Operator>().unwrap(), Operator::Mul);
    }

    #[test]
    fn test_parse_div() {
        assert_eq!("/".parse::<Operator>().unwrap(), Operator::Div);
    }

    #[test]
    fn test_parse_invalid() {
        assert!("x".parse::<Operator>().is_err());
        assert!("".parse::<Operator>().is_err());
        assert!("add".parse::<Operator>().is_err());
    }
}
