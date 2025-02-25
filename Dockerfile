FROM rust:latest

# WORKDIR /app

#COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Install necessary build dependencies
# RUN apk add --no-cache musl-dev upx

RUN cargo build --release 

Cmd ["/app/target/release/fibbot"]

# Compress the binary using upx
# RUN upx --best target/release/fibbot

# Use a minimal base image for the final image
# FROM scratch

# COPY --from=build-env /app/target/release/fibbot .
