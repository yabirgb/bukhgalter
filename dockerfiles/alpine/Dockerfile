LABEL version="1.0" maintainer="Yabir Garcia <yabirgb@gmail.com>" 
FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --lib bukhgalter
WORKDIR ./bukhgalter
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/bukhgalter*
RUN cargo build --release


FROM alpine:latest

ARG APP=/usr/src/app

#EXPOSE 8000

ENV APP_USER=debt_user

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

COPY --from=builder /home/rust/src/bukhgalter/target/x86_64-unknown-linux-musl/release/bukhgalter ${APP}/bukhgalter

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./bukhgalter"]