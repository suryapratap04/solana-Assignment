FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml ./
COPY Cargo.lock ./

COPY src ./src

RUN cargo build --release


FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/solana_fellowship_server /usr/local/bin/solana_fellowship_server

ENV PORT=8080

EXPOSE 8080

CMD ["solana_fellowship_server"]
