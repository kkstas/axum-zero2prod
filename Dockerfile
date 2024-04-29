FROM rust:1.77.2
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
ENV APP_ENVIRONMENT production
RUN cargo build --release
ENTRYPOINT ["./target/release/axum-zero2prod"]
