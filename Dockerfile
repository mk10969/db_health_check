FROM rust:1.45.2

WORKDIR /opt/src/app

COPY . .

### $HOME/.cargo/bin/ にinstallされる。
RUN cargo install --path .

### path通っているのでRunできる。
ENTRYPOINT ["rust-pinger"]
