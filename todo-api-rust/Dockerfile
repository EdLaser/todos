FROM rust

WORKDIR /usr/src/api

COPY . .

RUN cargo build --release

CMD ["./target/release/main"]