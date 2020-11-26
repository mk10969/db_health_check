FROM rust:1.45.2

WORKDIR /opt/src/app

COPY . .

RUN cargo install --path .

ENTRYPOINT ["rust-pinger"]
