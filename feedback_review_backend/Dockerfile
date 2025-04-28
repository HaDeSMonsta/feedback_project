FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN printf "\n[profile.release]\nlto = true\ncodegen-units = 1\n" >> Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN printf "\n[profile.release]\nlto = true\ncodegen-units = 1\n" >> Cargo.toml
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/feedback_review_backend /usr/local/bin/feedback_review_backend
CMD ["feedback_review_backend"]
