FROM rust:1.39

WORKDIR /build

RUN apt-get update && \
    apt-get install -y musl-tools && \
    rm -rf /var/lib/apt/lists/* && \
    rustup target add x86_64-unknown-linux-musl

COPY Cargo.* ./

RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --target x86_64-unknown-linux-musl --release && \
    rm src/*.rs

COPY src/ ./src/

RUN touch src/*.rs && \
    PATH=$PWD/bin:$PATH cargo install --target x86_64-unknown-linux-musl --root $PWD --path $PWD
