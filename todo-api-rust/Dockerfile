FROM rust:slim-buster

RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install libpq-dev

WORKDIR /usr/src/api

COPY . .

RUN cargo build --release

EXPOSE 8000

CMD ["./target/release/main"]