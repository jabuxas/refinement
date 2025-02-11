FROM rustlang/rust:nightly-slim as builder
WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl --bin refinement

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/refinement /refinement
CMD ["/refinement"]
