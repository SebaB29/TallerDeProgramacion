use crate::operator::Operator;
use crate::protocol::Operation;
use std::result::Result;

/// Aplica una operación a un valor actual y devuelve el nuevo valor o un error con motivo.
/// Se usa i128 como tipo del acumulador por seguridad.
pub fn apply_operation(current: i128, op: &Operation) -> Result<i128, String> {
    let operand = op.operand as i128;
    match op.op {
        Operator::Add => Ok(current.checked_add(operand).ok_or("Overflow")?),
        Operator::Sub => Ok(current.checked_sub(operand).ok_or("Underflow")?),
        Operator::Mul => Ok(current.checked_mul(operand).ok_or("Overflow")?),
        Operator::Div => {
            if operand == 0 {
                Err("Division por cero".to_string())
            } else {
                Ok(current / operand)
            }
        }
    }
}
