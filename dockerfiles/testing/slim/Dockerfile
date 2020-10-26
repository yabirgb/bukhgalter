LABEL version="1.0" maintainer="Yabir Garcia <yabirgb@gmail.com>" 
# Elegimos la imagen base para el contenedor
FROM rust:1.44.1-slim as builder

# Instalamos make como dependencia
RUN apt-get update && apt-get install make

# Nos ponemos a trabajar en la carpeta test
WORKDIR ./test

# creamos una carpeta src
RUN mkdir src/
# y aniadimos un archivo base para poder compilar
RUN echo "fn main() {println!(\"This shouldn't be in your code\")}" > src/main.rs

# Copiamos el archivo que contiene las dependencias del proyecto
COPY ./Cargo.toml ./Cargo.toml

# Construimos el proyecto en modo debug para compilar las dependencias externas
RUN cargo build
# Eliminamos el código autogenerado por cargo al crear el proyecto
RUN rm -rf src
# Eliminamos los ejecutables asociados al código por defecto
RUN rm ./target/debug/deps/bukhgalter*

# Creamos un usuario para ejecutar los tests

ENV APP_USER=test_user

RUN useradd -ms /bin/bash $APP_USER
USER $APP_USER

# Ejecutamos los tests
CMD ["make", "test"]