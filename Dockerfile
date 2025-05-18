# installing cargo chef to leverage docker caching
FROM rust:1.86.0-slim-bookworm AS chef

RUN apt update -y && \
    apt install -y pkg-config libssl-dev ca-certificates libpq-dev && \
    cargo install cargo-chef --locked

WORKDIR /builder

# preparing recipe for the current dependencies
FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

# building the actual binary
FROM chef AS builder

COPY --from=planner /builder/recipe.json .

RUN cargo chef cook --release --recipe-path recipe.json

RUN cargo install diesel_cli --no-default-features --features postgres

COPY . .

RUN cargo build --release --workspace --locked

# entering runtime without a distro
FROM debian:bookworm-slim AS runner

RUN apt update -y && \
    apt install -y ca-certificates libpq5 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /runner

COPY migrations ./migrations
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /builder/target/release/schizophrenia-bot /usr/local/bin/schizophrenia-bot

CMD ["/usr/local/bin/schizophrenia-bot"]