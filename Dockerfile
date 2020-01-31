FROM rust:1.41 AS builder

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
    export PATH=$PWD/bin:$PATH && \
    cargo install \
        --target x86_64-unknown-linux-musl \
        --root $PWD \
        --path $PWD

FROM alpine

ARG BUILD_TARGET

ENV RUST_LOG=actix_web=info
ENV SERVER_LISTEN_ADDR=0.0.0.0
ENV SERVER_LISTEN_PORT=80

COPY --from=builder /build/bin/$BUILD_TARGET /usr/local/bin/server

EXPOSE 80

ENTRYPOINT ["/usr/local/bin/server"]
