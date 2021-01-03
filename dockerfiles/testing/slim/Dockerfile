# Elegimos la imagen base para el contenedor
FROM rust:1.44.1-slim as builder
LABEL version="1.0" maintainer="Yabir Garcia <yabirgb@gmail.com>" 

# Instalamos make como dependencia
RUN apt-get update && apt-get install make libpq-dev --no-install-recommends -y

# Nos ponemos a trabajar en la carpeta test
WORKDIR ./test

# creamos una carpeta src # y aniadimos un archivo base para poder compilar

RUN mkdir src/ && echo "fn main() {println!(\"This shouldn't be in your code\")}" > src/main.rs

# Copiamos el archivo que contiene las dependencias del proyecto
COPY ./Cargo.toml ./Cargo.toml

# Construimos el proyecto en modo debug para compilar las dependencias externas
RUN cargo build
# Eliminamos el código autogenerado por cargo al crear el proyecto
# Eliminamos los ejecutables asociados al código por defecto
RUN rm -rf src && rm ./target/debug/deps/bukhgalter*

# Ejecutamos los tests
CMD ["make", "test"]

# NOTA: Soy consciente de que uso el usuario root al hacer los tests. En la
# documentación en el archivo de `cotenerizacion` comento este problema