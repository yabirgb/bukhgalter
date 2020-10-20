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

En su momento se consideraron otras alternativas como
[speculate.rs](https://github.com/utkarshkukreti/speculate.rs) que mantiene un
estilo similar a `RSpec`. No me he decidido por esta librearía que parece ser
una de las que más apoyo tiene dentro de la comunidad porque añade dependencias
que hay que compilar y no considero que me aporta nada adicional frente a
utilizar el modulo de `testing` de `cargo`. Además `cargo` se actualiza
frecuentemente junto a `Rust` y es una herramienta más probada.

Un último argumento aunque no de peso para elegir `cargo` a la hora de hacer
`tests` es que se utiliza en multitud de proyectos y es el ejecutor de tests
elegido por mozilla en la inmensa mayoría de sus proyectos.