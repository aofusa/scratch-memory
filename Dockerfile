FROM rust:1-slim-bullseye AS builder
WORKDIR /app
COPY . /app
RUN RUSTFLAGS= cargo test && cargo build --release

FROM scratch
COPY --from=builder /app/target/release/scratch-memory /hello-world
CMD ["/hello-world"]

