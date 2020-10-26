FROM rust:1.44.1 as builder
LABEL version="1.0" maintainer="Yabir Garcia <yabirgb@gmail.com>" 

# Creamos una libreria vacía y copiamos nuestros archivos de dependencias
RUN USER=root cargo new --lib bukhgalter
WORKDIR ./bukhgalter
COPY ./Cargo.toml ./Cargo.toml
# Compilamos
RUN cargo build --release
# Eliminamos los archivos del src
RUN rm src/*.rs
# Añadimos los archivos locales
ADD . ./
# Borramos los binarios generadors
RUN rm ./target/release/deps/bukhgalter*
# Compilamos la aplicación en modo produción
RUN cargo build --release

# Para el segundo paso usamos una imagen de debian
FROM debian:buster-slim

# Creamos un usuario y especificamos el lugar de la aplicacón
ARG APP=/usr/src/app
#EXPOSE 8000
ENV APP_USER=debt_user
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

# Copiamos al nuevo contenedor el binario generado en el paso anterior
COPY --from=builder /bukhgalter/target/release/bukhgalter ${APP}/bukhgalter
# Le damos permiso de ejecución
RUN chown -R $APP_USER:$APP_USER ${APP}
# Cambiamos el usuario que los ejecuta
USER $APP_USER
WORKDIR ${APP}
# Ejecutamos la aplicación
CMD ["./bukhgalter"]

# docker build -t rust-debian -f ./debian/Dockerfile .