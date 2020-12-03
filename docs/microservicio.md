# Microservicio

## Elección del framework

Para la construcción del microservicio he valorado tres frameworks sobre los que
tenía experiencia previa y que son los más utilizados en la comunidad de Rust
para crear REST APIs.

- [Actix](https://actix.rs/)
- [Rocket](https://github.com/SergioBenitez/Rocket)
- [Warp](https://github.com/seanmonstar/warp)

A continuación enumero ventajas y desventajas que he considerado sobre cada uno

### Consideraciones

#### Actix

Pros:

- Muy popular con gran cantidad de ejemplos.
- Buena metodología para describir los endpoints y la integración con la lógica.
- Muy rápido.

Cons:

- Esconde bastante lógica detras de macros y funciones y a veces se hace.
  complicado entender que esta pasando.

#### Rocket

Pros:

- Sigue un diseño mas parecido a frameworks como rails.
- Muy popular con ejemplos.

Cons:

- Requiere utilizar la versión nightly de rust.
- Lento.
- No tiene soporte para funciones `async`.

#### Warp

Pros:

- Una metodología de diseño sencilla.
- Asíncrono.
- Muy rápido.
- Cada vez más popular

Cons:

- Pocos ejemplos pero muy bien documentados.


### Decisión

Finalmente me he decantado por `warp` por ser el candidato más fuerte de los que
he visto y el que creo que más me va a facilitar el desarrollo.

## Diseño

Los endpoints que se van a diseñar han sido documentados en el apartado [URLS](urls.md).

Respecto al diseño de la aplicación se ha estructurado de la siguiente manera:

- Se han creado una serie de [funciones filter](/src/filters) que manejan las peticiones que se reciben.
- Se han creado una serie de [funciones handler](/src/handlers) que manejan la lógica detrás de los filtros.

Respecto a la gestión de como almacenar la información que se recibe y envía se
ha creado un `trait` de Rust. Los traits indican funcionalidades que deben
implementarse de manera completa. En mi caso he creado un trait
[DataManager](/src/models/mod.rs#L12) que expone métodos públicos para obtener y
almacenar la información. A continuación he creado un
[MemoryDataManager](/src/models/mod.rs#L23) que gestiona las cuentas del proyecto en
memoria. De esta maneras desde mis handlers puede gestionar la información sin
tener que manejar la lógica que hay detrás.