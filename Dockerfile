FROM rust:1.67 AS builder

WORKDIR /build

RUN apt-get update \
 && apt-get install -y musl-tools \
 && rm -rf /var/lib/apt/lists/* \
 && rustup target add x86_64-unknown-linux-musl

COPY Cargo.* ./

RUN mkdir src \
 && echo "fn main() {}" > src/main.rs \
 && cargo build --target x86_64-unknown-linux-musl --release \
 && rm src/*.rs

COPY src/ ./src/

RUN touch src/*.rs \
 && export PATH=$PWD/bin:$PATH \
 && cargo install \
        --target x86_64-unknown-linux-musl \
        --root $PWD \
        --path $PWD

# -----------------------------------------------------------------------------

FROM scratch

ARG BUILD_TARGET

COPY --from=builder /etc/passwd /etc/group /etc/
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

COPY --from=builder --chown=nobody:nogroup /build/bin/$BUILD_TARGET /bin/server

USER nobody

ENV RUST_LOG=actix_web=info
ENV SERVER_LISTEN_ADDR=0.0.0.0
ENV SERVER_LISTEN_PORT=8080

EXPOSE 8080

ENTRYPOINT ["/bin/server"]
