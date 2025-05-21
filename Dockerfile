FROM rustlang/rust:nightly-slim AS builder

WORKDIR /app

COPY . .

RUN cargo build --release && \
    ls -la target/release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/dodo-assignment-rust .

ENV RUST_LOG=info

EXPOSE 3000

CMD ["./dodo-assignment-rust"]
