use crate::operator::Operator;
use crate::protocol::Operation;

/// Aplica una operación aritmética sobre el valor actual (`u8`).
/// Para +, -, * usamos aritmética wrapping (módulo 256).
/// Para / hacemos división entera; si el operando es 0 devolvemos error.
pub fn apply_operation(current: u8, op: &Operation) -> Result<u8, String> {
    let operand = op.operand;

    match op.op {
        Operator::Add => Ok(current.wrapping_add(operand)),
        Operator::Sub => Ok(current.wrapping_sub(operand)),
        Operator::Mul => Ok(current.wrapping_mul(operand)),
        Operator::Div => {
            if operand == 0 {
                Err("division by zero".to_string())
            } else {
                // división entera; quedamos con el comportamiento usual de u8
                Ok(current / operand)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operator::Operator;

    #[test]
    fn test_add_normal() {
        let op = Operation {
            op: Operator::Add,
            operand: 5,
        };
        assert_eq!(apply_operation(10, &op).unwrap(), 15);
    }

    #[test]
    fn test_sub_normal() {
        let op = Operation {
            op: Operator::Sub,
            operand: 3,
        };
        assert_eq!(apply_operation(10, &op).unwrap(), 7);
    }

    #[test]
    fn test_mul_normal() {
        let op = Operation {
            op: Operator::Mul,
            operand: 4,
        };
        assert_eq!(apply_operation(3, &op).unwrap(), 12);
    }

    #[test]
    fn test_div_normal() {
        let op = Operation {
            op: Operator::Div,
            operand: 2,
        };
        assert_eq!(apply_operation(10, &op).unwrap(), 5);
    }

    #[test]
    fn test_div_by_zero() {
        let op = Operation {
            op: Operator::Div,
            operand: 0,
        };
        assert_eq!(apply_operation(10, &op).unwrap_err(), "division by zero");
    }
}
