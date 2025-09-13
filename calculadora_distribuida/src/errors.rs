//! Manejo simple de errores de salida
pub fn print_fatal_and_exit(msg: &str) {
    // Imprime el mensaje en STDERR en el formato requerido.
    eprintln!("ERROR \"{}\"", msg);
}
