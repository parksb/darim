FROM rust:1.42

WORKDIR /srv/darim-server
COPY . .

RUN cargo install diesel_cli
RUN diesel setup
RUN diesel migration run

RUN cargo build --release

EXPOSE $PORT
CMD ["./target/release/darim-server"]
