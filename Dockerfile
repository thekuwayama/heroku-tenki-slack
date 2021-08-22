FROM rust:1.48.0

USER root
RUN cargo install --git https://github.com/thekuwayama/heroku-tenki-slack.git --branch main

RUN mkdir -p /opt
ENV TEMPDIR /opt/
ADD entrypoint.sh /opt/entrypoint.sh
RUN chmod +x /opt/entrypoint.sh

CMD ["/opt/entrypoint.sh"]
