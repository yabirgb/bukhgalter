# Partimes de una imagen base
# Elegimos la imagen base para el contenedor
FROM rust:1.44.1-slim as builder
# Nos ponemos a trabajar en la carpeta test
WORKDIR ./test
# Copiamos el archivo que contiene las dependencias del proyecto
COPY ./Cargo.toml ./Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# Construimos el proyecto en modo debug para compilar las dependencias externas
RUN cargo build
# Eliminamos el código autogenerado por cargo al crear el proyecto
RUN rm -rf src
# Eliminamos los ejecutables asociados al código por defecto
RUN rm ./target/debug/deps/bukhgalter*

# Ejecutamos los tests
CMD ["cargo", "test"]