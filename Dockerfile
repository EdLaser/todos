FROM rust

WORKDIR /usr/src/api

COPY ./todo-api .

RUN diesel setup

RUN diesle migration run

CMD ["cargo","run"]