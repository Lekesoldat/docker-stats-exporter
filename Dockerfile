FROM rust:1.77

RUN apt update
RUN apt install docker.io -y

WORKDIR /usr/src/docker-stats-exporter
COPY . .

RUN cargo install --path .

CMD ["docker-stats-exporter"]
