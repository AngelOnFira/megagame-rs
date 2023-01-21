FROM rust:1 as builder

WORKDIR /app

COPY . /app

# Install libs
RUN apt-get update && apt-get install -y lld clang

# Build release
RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder /app/target/release/megagame-rs /app/megagame-rs

ENTRYPOINT ["/app/megagame-rs"]
