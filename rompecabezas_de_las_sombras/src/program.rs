use crate::flatlander::Flatlander;
use std::io;

/// Ejecuta el programa principal.
///
/// - Lee los datos iniciales (ángulo de luz y cantidad de flatlanders).
/// - Procesa la lista de flatlanders desde la entrada estándar.
/// - Calcula los intervalos de sombra y la longitud total cubierta.
/// - Imprime el resultado por salida estándar.
///
/// Si ocurre un error en la entrada, muestra un mensaje de error en `stderr`
/// y aborta la ejecución.
pub fn ejecutar_programa() {
    let datos_iniciales = match procesar_entrada() {
        Ok(datos) => datos,
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        }
    };

    let flatlanders = match procesar_flatlanders(datos_iniciales.1) {
        Ok(f) => f,
        Err(msg) => {
            eprintln!("{}", msg);
            return; // aborta ejecución
        }
    };

    let intervalos = calcular_intervalos(&flatlanders, datos_iniciales.0);
    let longitud_cubierta = calcular_longitud_cubierta(&intervalos);
    println!("{}", longitud_cubierta);
}

/// Procesa la primera línea de entrada para obtener:
/// - El ángulo de la luz en grados.
/// - La cantidad de flatlanders.
///
/// # Returns
/// - `Ok((f64, usize))`: El ángulo y la cantidad de flatlanders.
/// - `Err(String)`: Mensaje de error en caso de fallo.
///
/// # Errores
/// - `"IO"` si ocurre un error de lectura.
/// - Propaga los errores de [`extraer_datos_iniciales`].
fn procesar_entrada() -> Result<(f64, usize), String> {
    let mut entrada = String::new();
    if io::stdin().read_line(&mut entrada).is_err() {
        return Err(String::from("Error: \"IO\"."));
    }

    extraer_datos_iniciales(&entrada)
}

/// Procesa múltiples líneas de entrada para crear una lista de `Flatlander`.
///
/// # Parámetros
/// - `cantidad_flatlanders`: Número de flatlanders que se deben leer.
///
/// # Returns
/// - `Ok(Vec<Flatlander>)`: Lista de flatlanders válidos.
/// - `Err(String)`: Mensaje de error si alguno de los flatlanders es inválido.
///
/// # Errores
/// - `"IO"` si ocurre un error de lectura.
/// - Propaga los errores de [`extraer_datos_flatlander`].
fn procesar_flatlanders(cantidad_flatlanders: usize) -> Result<Vec<Flatlander>, String> {
    let mut flatlanders: Vec<Flatlander> = Vec::new();

    for _ in 0..cantidad_flatlanders {
        let mut datos_flatlander = String::new();
        if io::stdin().read_line(&mut datos_flatlander).is_err() {
            return Err(String::from("Error: \"IO\"."));
        }

        if datos_flatlander.trim().is_empty() {
            return Err(String::from("Error: \"Linea faltante\""));
        }

        match extraer_datos_flatlander(&datos_flatlander) {
            Ok(flatlander) => flatlanders.push(flatlander),
            Err(msg) => return Err(msg),
        }
    }

    Ok(flatlanders)
}

/// Extrae y valida los datos iniciales desde un `&str`.
///
/// # Parámetros
/// - `entrada`: Cadena con dos valores separados por espacio:
///   - Ángulo de luz en grados (`10 <= θ <= 80`)
///   - Número de flatlanders (`1 <= N <= 100000`)
///
/// # Returns
/// - `Ok((f64, usize))`: Ángulo y cantidad de flatlanders.
/// - `Err(String)`: Mensaje de error en caso de datos inválidos.
pub fn extraer_datos_iniciales(entrada: &str) -> Result<(f64, usize), String> {
    let datos: Vec<&str> = entrada.split_whitespace().collect();
    if datos.len() < 2 {
        return Err(String::from("Error: \"Valor faltante\""));
    }

    let angulo: f64 = match datos[0].parse() {
        Ok(a) => a,
        Err(_) => return Err(String::from("Error: \"Numero invalido\"")),
    };

    // validación de rango del ángulo
    if !(10.0..=80.0).contains(&angulo) {
        return Err(String::from("Error: \"Fuera de rango\""));
    }

    let num_flatlanders: usize = match datos[1].parse() {
        Ok(c) => c,
        Err(_) => return Err(String::from("Error: \"Numero invalido\"")),
    };

    // validación de rango de cantidad de flatlanders
    if !(1..=100_000).contains(&num_flatlanders) {
        return Err(String::from("Error: \"Fuera de rango\""));
    }

    Ok((angulo, num_flatlanders))
}

/// Extrae y valida los datos de un `Flatlander` desde un `&str`.
///
/// # Parámetros
/// - `datos`: Cadena con dos valores separados por espacio:
///   - `x`: Posición en el eje (`0 <= x <= 300000`)
///   - `altura`: Altura (`1 <= H <= 1000`)
///
/// # Returns
/// - `Ok(Flatlander)`: Instancia válida de un flatlander.
/// - `Err(String)`: Mensaje de error en caso de datos inválidos.
pub fn extraer_datos_flatlander(datos: &str) -> Result<Flatlander, String> {
    let partes: Vec<&str> = datos.split_whitespace().collect();
    if partes.len() < 2 {
        return Err(String::from("Error: \"Valor faltante\""));
    }

    let x: f64 = match partes[0].parse() {
        Ok(n) => n,
        Err(_) => return Err(String::from("Error: \"Numero invalido\"")),
    };

    if !(0.0..=300_000.0).contains(&x) {
        return Err(String::from("Error: \"Fuera de rango\""));
    }

    let altura: f64 = match partes[1].parse() {
        Ok(n) => n,
        Err(_) => return Err(String::from("Error: \"Numero invalido\"")),
    };

    if !(1.0..=1000.0).contains(&altura) {
        return Err(String::from("Error: \"Fuera de rango\""));
    }

    Ok(Flatlander::new(x, altura))
}

/// Calcula los intervalos de sombra proyectados por una lista de flatlanders.
///
/// # Parámetros
/// - `flatlanders`: Lista de flatlanders.
/// - `angulo_luz`: Ángulo de la luz en grados.
///
/// # Returns
/// - `Vec<(f64, f64)>`: Lista de intervalos `[inicio, fin]` ordenados por el inicio.
pub fn calcular_intervalos(flatlanders: &[Flatlander], angulo_luz: f64) -> Vec<(f64, f64)> {
    let mut intervalos: Vec<(f64, f64)> = Vec::new();
    for flatlander in flatlanders {
        intervalos.push(flatlander.intervalo_sombra(angulo_luz));
    }
    // Ordenamos por inicio
    intervalos.sort_by(|a, b| a.0.total_cmp(&b.0));
    intervalos
}

/// Calcula la longitud total cubierta por una unión de intervalos.
///
/// # Parámetros
/// - `intervalos`: Lista de intervalos `[inicio, fin]`.
///
/// # Returns
/// - `f64`: Longitud total cubierta por los intervalos.
pub fn calcular_longitud_cubierta(intervalos: &[(f64, f64)]) -> f64 {
    let mut longitud_cubierta = 0.0;
    if intervalos.is_empty() {
        return longitud_cubierta;
    }

    let (mut inicio_anterior, mut fin_anterior) = intervalos[0];
    for &(inicio, fin) in &intervalos[1..] {
        // Compruebo si se suerponen
        if inicio <= fin_anterior {
            if fin > fin_anterior {
                fin_anterior = fin;
            }
        } else {
            longitud_cubierta += fin_anterior - inicio_anterior;
            inicio_anterior = inicio;
            fin_anterior = fin;
        }
    }

    longitud_cubierta + (fin_anterior - inicio_anterior)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flatlander::Flatlander;

    #[test]
    fn test_extraer_datos_iniciales_ok() {
        let entrada = "45 3\n";
        let res = extraer_datos_iniciales(entrada);
        assert!(res.is_ok());
        let (angulo, n) = res.unwrap();
        assert_eq!(angulo, 45.0);
        assert_eq!(n, 3);
    }

    #[test]
    fn test_extraer_datos_iniciales_faltante() {
        let entrada = "45\n";
        let res = extraer_datos_iniciales(entrada);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Error: \"Valor faltante\"");
    }

    #[test]
    fn test_extraer_datos_iniciales_angulo_inferior_al_rango() {
        let entrada = "5 3\n"; // ángulo fuera de rango
        let res = extraer_datos_iniciales(entrada);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Error: \"Fuera de rango\"");
    }

    #[test]
    fn test_extraer_datos_iniciales_angulo_superior_al_rango() {
        let entrada = "85 3\n"; // ángulo fuera de rango
        let res = extraer_datos_iniciales(entrada);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Error: \"Fuera de rango\"");
    }

    #[test]
    fn test_extraer_datos_iniciales_cantidad_flatlanders_inferior_al_rango() {
        let entrada = "45 0\n"; // cantidad de flatlanders fuera de rango
        let res = extraer_datos_iniciales(entrada);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Error: \"Fuera de rango\"");
    }

    #[test]
    fn test_extraer_datos_iniciales_cantidad_flatlanders_superior_al_rango() {
        let entrada = "45 100001\n"; // cantidad de flatlanders fuera de rango
        let res = extraer_datos_iniciales(entrada);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Error: \"Fuera de rango\"");
    }

    #[test]
    fn test_extraer_datos_iniciales_numero_invalido() {
        let entrada = "abc 3\n";
        let res = extraer_datos_iniciales(entrada);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Error: \"Numero invalido\"");
    }

    #[test]
    fn test_extraer_datos_flatlander_ok() {
        let entrada = "100 50\n";
        let res = extraer_datos_flatlander(entrada);
        assert!(res.is_ok());
        let f = res.unwrap();
        let (inicio, fin) = f.intervalo_sombra(45.0);
        assert_eq!(inicio, 100.0);
        assert!(fin > inicio);
    }

    #[test]
    fn test_extraer_datos_flatlander_posicion_inferior_al_rango() {
        let entrada = "-10 20\n"; // fuera del rango de x
        let res = extraer_datos_flatlander(entrada);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Error: \"Fuera de rango\"");
    }

    #[test]
    fn test_extraer_datos_flatlander_posicion_superior_al_rango() {
        let entrada = "400000 20\n"; // fuera del rango de x
        let res = extraer_datos_flatlander(entrada);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Error: \"Fuera de rango\"");
    }

    #[test]
    fn test_extraer_datos_flatlander_altura_inferior_al_rango() {
        let entrada = "100 0.5\n"; // fuera del rango de altura
        let res = extraer_datos_flatlander(entrada);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Error: \"Fuera de rango\"");
    }

    #[test]
    fn test_extraer_datos_flatlander_altura_superior_al_rango() {
        let entrada = "100 1500\n"; // fuera del rango de altura
        let res = extraer_datos_flatlander(entrada);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Error: \"Fuera de rango\"");
    }

    #[test]
    fn test_extraer_datos_flatlander_numero_invalido() {
        let entrada = "100 abc\n";
        let res = extraer_datos_flatlander(entrada);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Error: \"Numero invalido\"");
    }

    #[test]
    fn test_calcular_intervalos() {
        let f1 = Flatlander::new(0.0, 10.0);
        let f2 = Flatlander::new(50.0, 20.0);
        let intervalos = calcular_intervalos(&[f1, f2], 45.0);
        assert_eq!(intervalos.len(), 2);
        assert!(intervalos[0].0 <= intervalos[1].0); // ordenados por inicio
    }

    #[test]
    fn test_calcular_longitud_cubierta_superpuesto() {
        let intervalos = vec![(0.0, 10.0), (5.0, 15.0)];
        let res = calcular_longitud_cubierta(&intervalos);
        assert_eq!(res, 15.0);
    }

    #[test]
    fn test_calcular_longitud_cubierta_disjunto() {
        let intervalos = vec![(0.0, 5.0), (10.0, 15.0)];
        let res = calcular_longitud_cubierta(&intervalos);
        assert_eq!(res, 10.0);
    }
}
