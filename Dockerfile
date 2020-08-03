FROM rust:1.45.1 as builder
WORKDIR /usr/src/app
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM rust:1.45.1-alpine3.12
ENV RUST_LOG=trace
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/vlinder /usr/local/bin/vlinder
CMD ["vlinder"]