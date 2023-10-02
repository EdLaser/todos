FROM rust

WORKDIR /usr/src/api

COPY . .

RUN diesel setup

RUN diesle migration run

CMD ["cargo","run"]