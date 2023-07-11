FROM rust:1.70 AS builder
WORKDIR /usr/src/api
COPY ./todo-api .
RUN cargo install --path .
RUN ls /usr/local/cargo/bin

FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
CMD ["api"]