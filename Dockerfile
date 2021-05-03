FROM rust:1.51-buster AS build

RUN apt-get update && apt-get install -y \
  libssl-dev;

RUN rustup component add rustfmt

WORKDIR /src/humberto

COPY . .

RUN cargo fmt -- --check

RUN \
    RUST_LOG=debug \
    RUST_BACKTRACE=1 \
    cargo build --release

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
  curl \
  openssl \
  tini;

RUN useradd svc

COPY --from=build /src/humberto/target/release/humberto /

RUN chown -R svc /humberto
USER svc

ENTRYPOINT ["/usr/bin/tini", "--"]

CMD ["/humberto"]
