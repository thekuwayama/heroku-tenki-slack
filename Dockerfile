FROM rust:1.48.0

USER root

WORKDIR /app
ENV TEMPDIR /app
COPY src /app/src
COPY Cargo.toml /app/Cargo.toml

RUN cargo build --release
RUN cargo install --path .

ENV TZ Asia/Tokyo

CMD ["heroku_tenki_slack"]
