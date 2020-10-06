
- Como **lenguaje de programación para el proyecto** se ha elegido
  [Rust](https://www.rust-lang.org/). El motivo para su elección es que es un
  lenguaje robusto y versátil. Cuenta con un sistema fuerte de tipos y incentiva
  una nueva manera de desarrollar software, concentrándose sobre todo en evitar
  fallos de memoria. Además tiene la eficiencia como objetivo principal. 

- Como gestor de tareas se ha decidido usar `make`. Rust se acompaña normalmente
  de la herramienta `cargo` para manejar ciertos aspectos de un proyecto, ya que
  incorpora ordenes como `cargo test` para ejecutar tests o `cargo build` para
  construir un proyecto. El problema con `cargo` es que crear reglas es una
  tarea complicada y que incluye mucha lógica, incluso usando complementos que
  intentan solucionar esto. Por este motivo he decidido que usar make y crear
  reglas en un `makefile` es la mejor solución. 