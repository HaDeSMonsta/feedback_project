FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir "src"
RUN echo "fn main() {println!(\"If you see this, the build broke\")}" > src/main.rs
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM chef AS proto_cacher
RUN apt update
RUN apt install protobuf-compiler -y

FROM proto_cacher AS builder
COPY . .
COPY --from=cacher /app/target target
RUN cargo build --release

FROM debian:latest
COPY --from=builder /app/target/release/feedback_input /usr/local/bin/feedback_input
CMD [ "feedback_input" ]
