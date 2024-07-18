FROM rust:slim-buster AS builder
RUN update-ca-certificates
WORKDIR /sahomedb
COPY . .
RUN cargo build --release

# Finalize image.
FROM debian:buster-slim
WORKDIR /sahomedb
COPY --from=builder /sahomedb/target/release/sahomedb .
COPY --from=builder /sahomedb/Rocket.toml .
CMD ["./sahomedb"]