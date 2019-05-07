FROM liuchong/rustup:nightly-musl AS builder

ARG PROJECT=dev
WORKDIR /usr/src/${PROJECT}/

ENV RUSTFLAGS=-Clinker=musl-gcc
RUN apt-get -y update && \
    apt-get install -y ffmpeg && \
    rustup target install x86_64-unknown-linux-musl && \
    cargo install cargo-watch

COPY Cargo.toml Cargo.lock ./

RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release --target=x86_64-unknown-linux-musl && \
    rm -rf src && \
    rm -f target/x86_64-unknown-linux-musl/release/deps/${PROJECT}*

CMD cargo watch -q -x run
