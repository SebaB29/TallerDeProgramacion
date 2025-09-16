# Calculadora Distribuida ⚡

<p align="justify">
Bienvenido a <b>Calculadora Distribuida</b>, un proyecto que implementa un sistema cliente-servidor para realizar operaciones aritméticas de forma concurrente sobre un valor central compartido.  
Este trabajo práctico pone en práctica conceptos de <b>concurrencia</b>, <b>comunicación en red</b> y <b>manejo de sockets</b> en Rust.
</p>

## 📜 Tabla de Contenidos

- [Características](#características)
- [Uso](#uso)
- [Ejemplos de Comunicación](#ejemplos-de-comunicación)
- [Estructura de Archivos](#estructura-de-archivos)
- [Tecnologías](#tecnologías)

## 🌟 Características

- Arquitectura cliente-servidor basada en sockets TCP
- Procesamiento concurrente de múltiples clientes mediante hilos (threads)
- Aplicación de operaciones aritméticas sobre un valor central compartido
- Comunicación basada en mensajes de texto delimitados por salto de línea
- Implementado siguiendo las buenas prácticas de Rust:
  - Sin `unwrap()` ni `expect()`
  - Sin `panic!()`
  - Sin uso de `unsafe`
  - Formateado con `cargo fmt`
  - Documentado con `cargo doc`
  - Tests unitarios e integración incluidos

## 🚀 Uso

El proyecto genera dos binarios: `server` y `client`.

### Servidor
Ejecutar el servidor indicando la dirección y puerto donde escuchará:
```bash
cargo run --bin server <dirección_IP>
```
Cada conexión entrante será manejada por un hilo independiente.

En caso de error irrecuperable, se imprimirá en STDERR con el formato:
```bash
ERROR "<motivo>"
```

### Cliente
Ejecutar el cliente indicando la dirección del servidor y el archivo con las operaciones:
```bash
cargo run --bin client <dirección IP> data/operaciones.txt
```
El cliente enviará las operaciones al servidor y luego imprimirá el valor final de la calculadora.

## 💬 Ejemplos de Comunicación
**Ejemplo 1**
```bash
client : OP + 1
server : OK
client : GET
server : VALUE 1
client : OP * 3
server : OK
client : OP + 2
server : OK
client : GET
server : VALUE 5
```

**Ejemplo 2 (operación inválida)**
```bash
client : OP + 1
server : OK
client : OP % 5
server : ERROR "Operacion invalida"
client : GET
server : VALUE 1
```

## 📁 Estructura de Archivos

```bash
calculadora-distribuida/
├── src/
│   ├── bin/
│   │    ├── client.rs
│   │    └── server.rs
│   ├── calculator.rs
│   ├── lib.rs
│   ├── operator.rs
│   └── protocol.rs
├── data/
│   └── operaciones.txt
├── README.md
├── Cargo.toml
└── enunciado.pdf
```

* **src/**: Contiene el código fuente principal, separado por módulos.
* **data/**: Contiene archivos con operaciones de ejemplo para el cliente.
* **Cargo.toml**: Archivo de configuración de dependencias y metadatos del proyecto.
* **enunciado.pdf**: Archivo con el enunciado completo del proyecto.

## 🛠️ Tecnologías
Este proyecto está desarrollado utilizando:

* Rust (última versión estable)
* Biblioteca estándar de Rust (sin crates externos)