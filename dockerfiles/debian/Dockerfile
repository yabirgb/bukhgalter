FROM rust:1.44.1 as builder

RUN USER=root cargo new --lib bukhgalter
WORKDIR ./bukhgalter
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/bukhgalter*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

#EXPOSE 8000

ENV APP_USER=debt_user

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /bukhgalter/target/release/bukhgalter ${APP}/bukhgalter

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./bukhgalter"]

# docker build -t rust-debian -f ./debian/Dockerfile .