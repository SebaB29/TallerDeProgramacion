#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Sub),
            "*" => Some(Operator::Mul),
            "/" => Some(Operator::Div),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
        }
    }
}
