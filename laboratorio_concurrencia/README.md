# Procesamiento Concurrente de Archivos ⚡

<p align="justify">
Este proyecto consiste en leer operaciones desde múltiples archivos y aplicarlas de forma secuencial o concurrente.  
Permite explorar conceptos de <b>concurrencia</b>, <b>sincronización de datos compartidos</b> y <b>comunicación entre threads</b> en Rust.
</p>

## 📜 Tabla de Contenidos

- [Características](#características)
- [Uso](#uso)
- [Ejercicios](#ejercicios)
- [Estructura de Archivos](#estructura-de-archivos)
- [Tecnologías](#tecnologías)

## 🌟 Características

- Lectura de operaciones desde múltiples archivos
- Ejecución secuencial o concurrente de operaciones
- Uso de mecanismos de sincronización (`Mutex`/`RwLock`) y canales (`mpsc`)
- Implementado siguiendo buenas prácticas de Rust:
  - Sin `unwrap()` ni `expect()`
  - Sin `panic!()`
  - Sin `unsafe`
  - Formateado con `cargo fmt`
  - Documentado con `cargo doc`
  - Incluye tests unitarios

## 🚀 Uso

Para procesar todos los archivos de la carpeta `data/`:

```bash
cargo run <mode> data/*
```

Los modos disponibles son:
* sequential
* concurrent_with_locks
* concurrent_with_channels

## 🧩 Ejercicios

* **Ejercicio 1**: Utilizar threads y locks para procesar los archivos de forma concurrente.

```
⚠️ Nota: Una ejecución concurrente podría producir un resultado distinto.
```

* **Ejercicio 2**: Reemplazar los locks por canales (mpsc) para coordinar el procesamiento concurrente.

## 📁 Estructura de Archivos

```bash
laboratorio-concurrencia/
├── src/
│   ├── modes/
│   │    ├── concurrent_with_channels.rs
│   │    ├── concurrent_with_locks.rs
│   │    └── sequential.rs
│   ├── calculator.rs
│   ├── main.rs
│   └── operation.rs
├── data/
│   ├── a.txt
│   ├── b.txt
│   └── ...
├── README.md
└── Cargo.toml
```

* **src/**: Contiene el código fuente principal del proyecto.
* **data/**: Archivos de entrada con operaciones a procesar.
* **Cargo.toml**: Archivo de configuración de dependencias y metadatos del proyecto.

## 🛠️ Tecnologías
Este proyecto está desarrollado utilizando:

* Rust (última versión estable)
* Biblioteca estándar de Rust (sin crates externos)