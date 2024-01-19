FROM rust:latest as builder
COPY ./app-server .
COPY ./runtimes .
RUN ls -la

COPY ./Cargo.toml .
COPY ./Cargo.lock .
RUN cargo build --release

FROM alpine:3.19
WORKDIR /app-server
COPY --from=builder /usr/app/target/release/app-server .
EXPOSE 8000
CMD ["./app-server"]
