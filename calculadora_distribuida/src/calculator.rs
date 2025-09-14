use crate::operator::Operator;
use crate::protocol::Operation;
use std::result::Result;

/// Aplica una operación aritmética sobre el valor actual.
///
/// # Parámetros
/// - `current`: el valor actual del estado (i128).
/// - `op`: la operación a aplicar, que contiene el operador y el operando.
///
/// # Retorna
/// - `Ok(i128)` con el nuevo valor si la operación se realizó correctamente.
/// - `Err(String)` con el motivo si ocurrió un error (overflow, underflow, división por cero).
pub fn apply_operation(current: i128, op: &Operation) -> Result<i128, String> {
    let operand = op.operand as i128;
    match op.op {
        Operator::Add => current
            .checked_add(operand)
            .ok_or_else(|| "Overflow".to_string()),
        Operator::Sub => current
            .checked_sub(operand)
            .ok_or_else(|| "Underflow".to_string()),
        Operator::Mul => current
            .checked_mul(operand)
            .ok_or_else(|| "Overflow".to_string()),
        Operator::Div => {
            if operand == 0 {
                Err("Division por cero".to_string())
            } else {
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
        assert_eq!(apply_operation(10, &op).unwrap_err(), "Division por cero");
    }

    #[test]
    fn test_add_overflow() {
        let op = Operation {
            op: Operator::Add,
            operand: 1,
        };
        assert!(
            apply_operation(i128::MAX, &op)
                .unwrap_err()
                .contains("Overflow")
        );
    }

    #[test]
    fn test_sub_underflow() {
        let op = Operation {
            op: Operator::Sub,
            operand: 1,
        };
        assert!(
            apply_operation(i128::MIN, &op)
                .unwrap_err()
                .contains("Underflow")
        );
    }

    #[test]
    fn test_mul_overflow() {
        let op = Operation {
            op: Operator::Mul,
            operand: 2,
        };
        assert!(
            apply_operation(i128::MAX / 2 + 1, &op)
                .unwrap_err()
                .contains("Overflow")
        );
    }
}
