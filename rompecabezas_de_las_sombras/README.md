# El Rompecabezas de las Sombras 🌒

<p align="justify">
Bienvenido a <b>El Rompecabezas de las Sombras</b>, un proyecto que consiste en calcular la longitud total cubierta por las sombras proyectadas por un conjunto de entidades ("flatlanders") en un mundo bidimensional.  
Este ejercicio pone en práctica conceptos de <b>geometría computacional</b>, <b>estructuras de datos</b> y <b>manejo de entrada/salida</b> en Rust.
</p>

## 📜 Tabla de Contenidos

- [Características](#características)
- [Uso](#uso)
- [Ejemplos de Entrada y Salida](#ejemplos-de-entrada-y-salida)
- [Estructura de Archivos](#estructura-de-archivos)
- [Tecnologías](#tecnologías)

## 🌟 Características

- Cálculo de sombras proyectadas en un mundo bidimensional
- Soporte para múltiples entidades con posibles sombras superpuestas
- Lectura de datos desde entrada estándar
- Salida con precisión de punto flotante
- Implementado siguiendo las buenas prácticas de Rust:
  - Sin `unwrap()` ni `expect()`
  - Sin `panic!()`
  - Sin uso de `unsafe`
  - Formateado con `cargo fmt`
  - Documentado con `cargo doc`
  - Tests unitarios e integración incluidos

## 🚀 Uso

Para ejecutar este proyecto:
```bash
cargo run
```
Luego proporciona los datos por entrada estándar:
```bash
<angulo_luz> <n_flatlanders>
<posicion_flatlander_1> <altura_flatlander_1>
<posicion_flatlander_2> <altura_flatlander_2>
```

## 📝 Ejemplos de Entrada y Salida

**Ejemplo 1**

Entrada
```bash
45 2
0 10
5 10
```

Salida
```bash
15.0000000000000
```

**Ejemplo 2**

Entrada
```bash
30 3
50 150
0 100
100 200
```

Salida
```bash
446.4101615137755
```

**Ejemplo 3**

Entrada
```bash
45 3
50 150
0 100
100 200
```

Salida
```bash
300.00000000000006
```

## 📁 Estructura de Archivos

```bash
rompecabezas-sombras/
├── src/
│   ├── flatlander.rs
│   ├── lib.rs
│   ├── main.rs
│   └── program.rs
├── tests/
│   └── integration_test.rs
├── README.md
├── Cargo.toml
└── enunciado.pdf
```

* **src/**: Contiene el código fuente principal del proyecto.
* **tests/**: Incluye tests de integración.
* **Cargo.toml**: Archivo de configuración de dependencias y metadatos del proyecto.
* **enunciado.pdf**: Archivo con el enunciado completo del proyecto.


## 🛠️ Tecnologías

Este proyecto está desarrollado utilizando:
* Rust (última versión estable)
* Biblioteca estándar de Rust (sin crates externos)
