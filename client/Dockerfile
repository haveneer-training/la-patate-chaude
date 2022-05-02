FROM rust:alpine3.14

WORKDIR /home/app
COPY . .

RUN apk update && \
    apk add musl-dev && \
    cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run"]
