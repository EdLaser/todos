FROM rust

WORKDIR /usr/src/api

COPY ./todo-api .

CMD ["cargo","run"]