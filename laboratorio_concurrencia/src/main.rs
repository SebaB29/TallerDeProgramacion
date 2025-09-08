//! Lee operaciones de una lista de archivos, y las aplica.
//!
//! Una vez procesados todos los archivos, imprime el resultado final.
//!
//! ## Uso
//!
//! Para procesar todos los archivos de la carpeta `data/`, ejecutar:
//! ```bash
//! cargo run -- data/*
//! ```
//! El resultado esperado de una ejecución secuencial es 26.
//!
//! ## Ejercicios
//!
//! ### Ejercicio 1
//!
//! Utilizar threads y locks para procesar los archivos de forma concurrente.
//!
//! NOTA: Una ejecución concurrente daría un resultado distinto.
//!
//! ### Ejercicio 2
//!
//! En lugar de utilizar locks para sincronizar el acceso a un recurso compartido, utilizar channels.
//!
//! ### Bonus 1
//!
//! Medir la diferencia de performance entre los 3 enfoques.
mod calculator;
mod operation;
mod modes {
    pub mod concurrent_with_channels;
    pub mod concurrent_with_locks;
    pub mod sequential;
}

use modes::concurrent_with_channels::run_concurrent_with_channels;
use modes::concurrent_with_locks::run_concurrent_with_locks;
use modes::sequential::run_sequential;

pub fn main() {
    // `Args` is an iterator over the program arguments.
    let mut inputs = std::env::args();

    // We skip the first argument, as its traditionally the path to the executable.
    inputs.next();

    let mode = inputs.next().unwrap_or_else(|| "sequential".to_string());

    if mode == "sequential" {
        println!("Running in sequential mode");
        run_sequential(inputs);
    } else if mode == "concurrent_with_locks" {
        println!("Running in concurrent mode with locks");
        run_concurrent_with_locks(inputs);
    } else if mode == "concurrent_with_channels" {
        println!("Running in concurrent mode with channels");
        run_concurrent_with_channels(inputs);
    } else {
        eprintln!("Unknown mode: {}", mode);
    }
}
