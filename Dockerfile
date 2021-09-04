FROM rust:1.48.0

USER root

WORKDIR /app
ENV TEMPDIR /app
COPY src /app/src
COPY Cargo.toml /app/Cargo.toml

RUN cargo build --release
RUN cargo install --path .

COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

ENV TZ Asia/Tokyo

CMD ["/app/entrypoint.sh"]
