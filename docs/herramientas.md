# Herramientas usadas en el proyecto

## Lenguaje

Como **lenguaje de programación para el proyecto** se ha elegido
[Rust](https://www.rust-lang.org/). El motivo para su elección es que es un
lenguaje robusto y versátil. Cuenta con un sistema fuerte de tipos y incentiva
una nueva manera de desarrollar software, concentrándose sobre todo en evitar
fallos de memoria. Además tiene la eficiencia como objetivo principal. 

## Task runner

Como gestor de tareas se ha decidido usar `make`. Rust se acompaña normalmente
de la herramienta `cargo` para manejar ciertos aspectos de un proyecto, ya que
incorpora ordenes como `cargo test` para ejecutar tests o `cargo build` para
construir un proyecto. El problema con `cargo` es que crear reglas es una
tarea complicada y que incluye mucha lógica, incluso usando complementos que
intentan solucionar esto. Por este motivo he decidido que usar `make` y crear
reglas en un `makefile` es la mejor solución. 

## Herramienta de tests

Para realizar los tests he optado por utilizar la utilidad de tests integrada en
cargo. Esta herramienta provee de utilidades para realizar tests unitarios
en `Rust`.

Los motivos por los que he optado por usar esta herramienta son:

- Cubre mi necesidad actual en el proyecto para realizar tests unitarios.
- Me permite desarrollar tests de manera rápida.
- Rust incluye una librería de aserciones completa para manejar las situaciones
  que se necesitan en los tests.
- Proporciona información detallada de los errores y el estado de los mismos.
- Integrado dentro de `Cargo` por lo que no se necesitan instalar dependencias
  adicionales.

Además para hacer uso de `fixtures` de forma cómoda he decidido usar
[rstest](https://github.com/la10736/rstest) que funciona como un `plugin`
sobre los tests de `Cargo` y permite el uso de los mismos de forma cómoda.