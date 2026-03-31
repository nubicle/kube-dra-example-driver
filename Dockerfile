FROM  rust:1.88-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
  protobuf-compiler \
  musl-tools

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /build
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl --bin example-driver

FROM scratch
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/example-driver /usr/bin/example-driver
