FROM rust:1.72-slim as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src
COPY migrations migrations
RUN cargo build --release
RUN cargo install --locked sqlx-cli --no-default-features --features rustls,postgres
COPY entrypoint.sh .

RUN chmod +x entrypoint.sh

EXPOSE 4000

CMD ["bash", "-c", "/app/entrypoint.sh"]
