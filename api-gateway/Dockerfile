FROM rust:1.42

WORKDIR /srv/darim-api-gateway
COPY . .

RUN cargo build --release

EXPOSE $PORT
CMD ["./target/release/darim-api-gateway"]
