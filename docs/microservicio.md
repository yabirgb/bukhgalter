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
he visto y el que creo que más me va a facilitar el desarrollo. Creo que va a
ser especialmente correcto porque fomenta una estructura de código muy limpia y
legible. Además los resultados en temas de rendimiento son excelentes y la
capacidad asíncrona que incorpora de manera nativa en su diseño han tenido mucho
peso en la decisión.

## Diseño

Los endpoints que se van a diseñar han sido documentados en el apartado [URLS](URLS.md).

Respecto al diseño de la aplicación se ha estructurado de la siguiente manera:

- Se han creado una serie de [funciones filter](/src/filters) que manejan las peticiones que se reciben.
- Se han creado una serie de [funciones handler](/src/handlers) que manejan la lógica detrás de los filtros.

De esta manera cuando una petición llega al servidor en primer lugar pasa por el
`Filter` correspondiente y este llama a una función `Handler` que es la que se
encarga de, usando los métodos de las estructuras creadas o las clases de datos,
generar la información que se solicita o actuar respecto a la entrada.
Finalmente el `Handler` devuelve el control al `Filter` que a su vez devuelve al
usuario la respuesta.

Respecto a la gestión de como almacenar la información que se recibe y envía se
ha creado un `trait` de Rust. Los traits indican funcionalidades que deben
implementarse de manera completa. En mi caso he creado un trait
[DataManager](/src/models/mod.rs#L12) que expone métodos públicos para obtener y
almacenar la información. A continuación he creado un
[MemoryDataManager](/src/models/mod.rs#L23) que gestiona las cuentas del proyecto en
memoria. De esta maneras desde mis handlers puede gestionar la información sin
tener que manejar la lógica que hay detrás.

Ha sido muy agradable trabajar con un lenguaje como Rust ya que ha permitido
mediante su sistema de tipos y su habilidad para analizar el código antes de
compilar, encontrar numerosos errores y obtener un código muy legible a la vez
que robusto.

## Testing

Para comprobar el correcto funcionamiento de las funcionalidades de la API se
han desarrollado una serie de `tests` (no requieren de un servidor ejecutándose).
Se han desarrollado tests para cada endpoint en [tests/test_events.rs](/tests/test_events.rs).

En total se han creado 13 tests distintos que prueba los distintos endpoints descritos en el proyecto con distintas entradas, algunas correctas y otras incorrectas, de manera que se siguen todos los casos posibles de entrada que un usuario puede seguir. Estos tests siguen las distintas historias de usuario.

Como se ha comentado los tests se ejecutan utilizando funcionalidades que
incorpora `warp` de manera que no es necesario ejecutar el servidor para poder
probar el correcto funcionamiento del servicio. Esto también es posible gracias
a la separación que se ha seguido en la parte de diseño y que mediante el
desacople permite testear cada modulo de manera independiente.
