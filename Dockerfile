FROM rust:1.88-slim-bookworm AS builder

# Provided by BuildKit: "arm64" on Apple Silicon, "amd64" on x86_64, etc.
ARG TARGETARCH

RUN apt-get update && apt-get install -y \
  protobuf-compiler \
  musl-tools

# Select the static-musl target that matches the platform being built.
RUN case "${TARGETARCH}" in \
      amd64) echo x86_64-unknown-linux-musl  > /tmp/triple ;; \
      arm64) echo aarch64-unknown-linux-musl > /tmp/triple ;; \
      *) echo "unsupported TARGETARCH=${TARGETARCH}" >&2; exit 1 ;; \
    esac; \
    rustup target add "$(cat /tmp/triple)"

WORKDIR /build
COPY . .

RUN triple="$(cat /tmp/triple)"; \
    cargo build --release --target "${triple}" --bin example-driver; \
    cp "target/${triple}/release/example-driver" /example-driver

FROM scratch
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /example-driver /usr/bin/example-driver
