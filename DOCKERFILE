FROM rust:bookworm AS builder
RUN update-ca-certificates
WORKDIR /sahomedb
COPY . .
RUN cargo build --release

# Finalize image.
FROM debian:bookworm
WORKDIR /sahomedb
COPY --from=builder /sahomedb/target/release/sahomedb .
CMD ["./sahomedb"]