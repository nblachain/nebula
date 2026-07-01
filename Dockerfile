FROM rust:1-slim-bookworm AS builder
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo build --release -p nebula-testnet

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates wget \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --create-home --uid 10001 nebula \
    && mkdir -p /data \
    && chown nebula:nebula /data
COPY --from=builder /build/target/release/nebula-testnet /usr/local/bin/nebula-testnet
USER nebula
VOLUME ["/data"]
EXPOSE 9944
HEALTHCHECK --interval=15s --timeout=5s --start-period=20s --retries=4 \
    CMD wget -qO- http://127.0.0.1:9944/health || exit 1
ENTRYPOINT ["nebula-testnet", "--run-rpc", "--rpc-bind", "0.0.0.0:9944", "--data-dir", "/data"]
