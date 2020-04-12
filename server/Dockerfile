FROM rust:1.42

WORKDIR /srv/patic
COPY . .

RUN cargo install diesel_cli
RUN diesel setup
RUN diesel migration run

RUN cargo build --release

EXPOSE 8080
CMD ["cargo run"]
