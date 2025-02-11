FROM rustlang/rust:nightly-slim

WORKDIR /app

COPY . .

RUN cargo build --release --bin refinement

CMD ["target/release/refinement"]
