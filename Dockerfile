FROM rust:1.63 as build

RUN USER=root cargo new --bin pialert2spaceapi
WORKDIR /pialert2spaceapi

COPY ./Cargo.lock ./Cargo.toml ./

RUN cargo build --release && \
	rm src/*.rs ./target/release/deps/pialert2spaceapi*

COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get -y install --no-install-recommends libsqlite3-0 && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/*

COPY --from=build /pialert2spaceapi/target/release/pialert2spaceapi .
COPY spaceapi.json .

CMD ["./pialert2spaceapi"]
