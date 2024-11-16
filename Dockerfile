FROM rust:latest

WORKDIR /usr/src/ferchat-api-rust

COPY . .

RUN cargo install --debug --path .

EXPOSE 8000

CMD ["ferchat-api-rust"]