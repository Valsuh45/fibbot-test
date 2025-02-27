FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo build --release

# Ensure workspace is correctly mounted
VOLUME /github/workspace

CMD ["/app/target/release/fibbot"]
