FROM rust:latest

WORKDIR /app
COPY . .
RUN cargo build --release

ENTRYPOINT ["/app/target/release/fibbot"]
CMD []
