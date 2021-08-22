FROM rust:1.48.0

USER root

RUN cargo install --git https://github.com/thekuwayama/heroku-tenki-slack.git --branch main
ENTRYPOINT ["/opt/entrypoint.sh"]
