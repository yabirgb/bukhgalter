## Contenerizacion para la ejecución de tests

### Objetivo 

Nuestro objetivo es crear un contenedor con lo necesario para que los tests se
ejecuten de la manera más rápida posible usando un contenedor de `docker`.

### Diseño del contenedor

Para elegir la imagen base del contenedor he seguido un procedimiento de
experimentación. En primer lugar necesitamos instalar `rust` y `cargo` en el
contenedor y después necesitamos compilar las dependencias de nuestra aplicación.
He considerado tres imágenes base para mi contenedor

- `rust:1.44.1` que utiliza `debian` internamente
- `rust:1.44.1-slim` que se basa en la anterior pero no contiene todos los
  complementos y solo trae las utilidades básicas.
- `rust:1.44.1-alpine`: Imagen basada en `alpine`

Dentro del `Dockerfile` hacemos los siguiente

    # Elegimos la imagen base para el contenedor
    FROM rust:1.44.1-slim as builder
    LABEL version="1.0" maintainer="Yabir Garcia <yabirgb@gmail.com>" 

    # Instalamos make como dependencia
    RUN apt-get update && apt-get install make --no-install-recommends

    # Creamos un usuario para ejecutar los tests
    ENV APP_USER=test_user
    RUN useradd -ms /bin/bash $APP_USER

    WORKDIR /test
    # creamos una carpeta src
    RUN mkdir src/ && echo "fn main() {println!(\"This shouldn't be in your code\")}" > src/main.rs
    # Copiamos el archivo que contiene las dependencias del proyecto
    COPY ./Cargo.toml ./Cargo.toml
    # Construimos el proyecto en modo debug para descargar las dependencias
    RUN cargo build
    # Eliminamos el código autogenerado por cargo al crear el proyecto
    RUN rm -rf src
    # Eliminamos los ejecutables asociados al código por defecto
    RUN rm ./target/debug/deps/bukhgalter*

    # Le damos permiso al usuario para utilizar cargo
    RUN chown $APP_USER:$APP_USER -R /usr/local/cargo/
    # Cambiamos el usuario
    USER $APP_USER
    # Ejecutamos los tests
    CMD ["make", "test"]

Con esto conseguimos que las dependencias estén descargadas y luego solo
tengamos que compilar los tests con los cambios de nuestro código.

### Problemas encontrados

Al realizar los contenedores me he encontrado un problema con la imagen basada
en `alpine`. Algunas de mis dependencias dependen a su vez de `glibc` y como
`alpine` usa `musl` estas no se han podido compilar en este contenedor. Por este
motivo he decidido continuar con los otros dos contenedores. He considerado
utilizar un contenedor que tenga `musl` pero al final estas imágenes se basan en
`debian` o similares y son equivalentes a las que ya estoy comparando con la
salvedad de que usan `musl`.

### Comparación

Para realizar los tests a la imagen basada en `debian` la he llamado
`bukhgalter_testing_b` y a la versión `slim` la he llamado `bukhgalter_testing_c`.
En primer lugar he comparado el tamaño de las dos imágenes

    bukhgalter_testing_b      latest              fc7438b4bccc        3 hours ago         1.4GB
    bukhgalter_testing_c      latest              53fa76a277d7        5 minutes ago       788MB

donde vemos que la imagen `slim` es la mitad de grande que la basada en
`debian`. No he comparado los tiempos que se tarda en construir las mismas ya
que es una cosa que no se realiza con frecuencia, solo cuando cambian las
dependencias. Sí he comparado el tiempo de ejecución de ambos contenedores en
los que se compilan los tests y se ejecutan los mismos

    docker run -t -v `pwd`:/test bukhgalter_testing_b:latest  0.02s user 0.03s system 0% cpu 15.493 total
    docker run -t -v `pwd`:/test bukhgalter_testing_c:latest  0.03s user 0.02s system 0% cpu 14.277 total

Vemos que por un segundo gana otra vez la imagen `slim`.

### Conclusión

A la vista de los resultados, ya que el contenedor en la versión `slim` tiene un
menor tamaño y ejecuta más rápido considero que es la mejor opción para realizar
los tests de la aplicación.

## Contenerización para desplegado de la aplicación

### Objetivos

Las aplicaciones escritas en `Rust` tienden a crecer en tamaño rápidamente
debido a la compilación de dependencias y el tiempo de compilación de las mismas
es muy grande. 

Por esto uno de los pasos claves cuando queremos contenerizar una aplicación
escrita en `Rust` es evitar la compilación de dependencias con frecuencia.

Otro problema es que si se opta por una imagen de `alpine` como imagen base
puede haber problemas a la hora de compilar si se usan ciertas librerías de 
`C`. 

### Construcción del contenedor

#### Elección de la imagen base

Para elegir la imagen base me he decantado entre dos posibilidades:

- `Debian`: Ya que incluye una buena base de software y de librerías para `C` que serán necesarias para compilar las dependencias del proyecto.
- `Alpine`: Ya que es una versión de `linux` famosa por el pequeño tamaño de los
  contenedores que se generan aunque sí se tienen más problemas a la hora de
  trabajar código de `Rust`.

Ya que `alpine` use `musl`, a la hora de compilar las dependencias será
necesario usar `musl` también para compilar las librerías necesarias. Es por eso
que se ha elegido como imagen para la compilación de las mismas `ekidd/rust-musl-builder:stable` que usa `ubuntu` pero compila con `musl`.

#### Resultado de la construcción

Tras construir los dos archivos `Dockerfile` con las distintas versiones los
contenedores resultantes tienen los siguientes tamaños:

        REPOSITORY                TAG                 IMAGE ID            CREATED             SIZE
        bukhgalter-alpine         latest              56737fdf344f        31 seconds ago      15MB
        bukhgalter-debian         latest              584ac87e2e5b        31 minutes ago      83.2MB

#### Elección del contenedor

A la hora de ejecutar ambos contenedores no he tenido ningún problema que me
haga decantar por uno u otro. Tampoco hay un factor de velocidad que comparar ya
que son contenedores pensados para desplegar la aplicación y con el estado
actual no se puede apreciar ninguna diferencia. A la vista de que el tamaño para
la imagen de `alpine` es mucho menor y, puesto que no hay ningún requisito
técnico que me haga decantarme con uno frente a otro, he decidido usar el
contenedor basado en `alpine`.

## Fuentes 

- [https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/](https://blog.logrocket.com/packaging-a-rust-web-service-using-docker)
- [http://whitfin.io/speeding-up-rust-docker-builds/](http://whitfin.io/speeding-up-rust-docker-builds/)
- [https://docs.github.com/es/free-pro-team@latest/actions/creating-actions/creating-a-docker-container-action](https://docs.github.com/es/free-pro-team@latest/actions/creating-actions/creating-a-docker-container-action)
- [https://github.com/marketplace/actions/build-and-push-docker-images](https://github.com/marketplace/actions/build-and-push-docker-images)