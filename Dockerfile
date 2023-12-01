FROM rust:latest

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY src/ src/

RUN cargo build --release

#remember to change de bind address to 0.0.0.0
EXPOSE 8080

CMD ["./target/release/actix-basic-api"]
