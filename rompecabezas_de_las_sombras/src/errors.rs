use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Errores {
    Io,
    ValorFaltante,
    NumeroInvalido,
    FueraDeRango,
    LineaFaltante,
}

impl fmt::Display for Errores {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errores::Io => write!(f, "Error: \"IO\"."),
            Errores::ValorFaltante => write!(f, "Error: \"Valor faltante\""),
            Errores::NumeroInvalido => write!(f, "Error: \"Numero invalido\""),
            Errores::FueraDeRango => write!(f, "Error: \"Fuera de rango\""),
            Errores::LineaFaltante => write!(f, "Error: \"Linea faltante\""),
        }
    }
}
