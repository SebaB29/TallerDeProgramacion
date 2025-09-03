/// Representa a un flatlander, definido por su posición en el eje X y su altura.
///
/// Un flatlander proyecta una sombra en el suelo dependiendo de su altura y
/// del ángulo de incidencia de la luz.
#[derive(Debug)]
pub struct Flatlander {
    x: f64,
    altura: f64,
}

impl Flatlander {
    /// Crea un nuevo `Flatlander`.
    ///
    /// # Parámetros
    /// - `x`: posición en el eje X.
    /// - `altura`: altura del flatlander.
    pub fn new(x: f64, altura: f64) -> Self {
        Flatlander { x, altura }
    }

    /// Calcula el intervalo en el eje X que ocupa la sombra del flatlander,
    /// dado un ángulo de luz en grados.
    ///
    /// # Parámetros
    /// - `angulo_luz`: angulo con el que incide la luz.
    ///
    /// Retorna una tupla `(inicio, fin)` que representa el rango de la sombra.
    pub fn intervalo_sombra(&self, angulo_luz: f64) -> (f64, f64) {
        let longitud_sombra = self.longitud_sombra(angulo_luz);
        (self.x, self.x + longitud_sombra)
    }

    /// Calcula la longitud de la sombra del flatlander en función de
    /// su altura y el ángulo de la luz (en grados).
    ///
    /// # Parámetros
    /// - `angulo_luz`: angulo con el que incide la luz.
    ///     
    /// Retorna la longitud de la sombra.
    fn longitud_sombra(&self, angulo_luz: f64) -> f64 {
        self.altura / angulo_luz.to_radians().tan()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longitud_sombra() {
        let f = Flatlander::new(0.0, 10.0);
        let sombra = f.longitud_sombra(45.0);
        assert_eq!(sombra, 10.000000000000002);
    }
}
