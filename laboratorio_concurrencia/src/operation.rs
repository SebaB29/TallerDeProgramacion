use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub enum Operation {
    Add(u8),
    Sub(u8),
    Mul(u8),
    Div(u8),
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string into tokens separated by whitespace.
        let tokens: Vec<&str> = s.split_whitespace().collect();

        // Try to convert the vector into a statically-sized array of 2 elements, failing otherwise.
        let [operation, operand] = tokens.try_into().map_err(|_| "expected 2 arguments")?;

        // Parse the operand into an u8.
        let operand: u8 = operand.parse().map_err(|_| "operand is not an u8")?;

        match operation {
            "+" => Ok(Operation::Add(operand)),
            "-" => Ok(Operation::Sub(operand)),
            "*" => Ok(Operation::Mul(operand)),
            "/" => Ok(Operation::Div(operand)),
            _ => Err("unknown operation"),
        }
    }
}
