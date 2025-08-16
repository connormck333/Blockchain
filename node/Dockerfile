FROM debian:bullseye-slim AS builder

RUN apt-get update && apt-get install -y \
    curl build-essential pkg-config libssl-dev \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && . $HOME/.cargo/env \
    && rustup default stable

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN useradd -m appuser

COPY --from=builder /usr/src/app/target/release/MockChain /usr/local/bin/mockchain

RUN chown appuser:appuser /usr/local/bin/mockchain

USER appuser

ENTRYPOINT ["mockchain"]