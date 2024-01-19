FROM rust:latest as builder
RUN apt-get update && apt-get install -y binaryen musl-tools && rm -rf /var/lib/apt/lists/* && rustup target add x86_64-unknown-linux-musl
COPY ./app-server ./app-server
COPY ./runtimes ./runtimes
COPY ./Cargo.toml .
COPY ./Cargo.lock .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder target/x86_64-unknown-linux-musl/release/app-server /app-server
EXPOSE 8000
ENTRYPOINT ["/app-server"]
