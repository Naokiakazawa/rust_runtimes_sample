FROM rust:latest as builder
COPY ./app-server ./app-server
COPY ./runtimes ./runtimes
COPY ./Cargo.toml .
COPY ./Cargo.lock .
RUN cargo build --release

FROM debian
COPY --from=builder target/release/app-server /usr/local/bin/app-server
EXPOSE 8000
CMD ["app-server"]
