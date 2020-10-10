# Build the code first
FROM rust:1.46.0-slim-buster AS builder

RUN rustc --version --verbose
RUN apt update -y && apt install -y pkg-config libssl-dev

WORKDIR /nrl

# We do a non-code build to get all the dependencies. This allows us to benefit from layer caching
COPY Cargo.lock Cargo.toml /nrl/
RUN mkdir /nrl/src
RUN touch /nrl/src/lib.rs
RUN echo "fn main() {}" > /nrl/src/main.rs
RUN cargo build --release

# Then we trash our actual build so that we can correctly build again
RUN find /nrl/target -name "*nrl*" | xargs rm -rf

# Finally, we do a full build of our code
RUN rm -rf /nrl/src
COPY src /nrl/src
COPY migrations /nrl/migrations
RUN cargo build --release


# Next build a container with the build artifact but no code
FROM debian:buster-slim

RUN apt update -y && apt install -y openssl ca-certificates wget

ENV DOCKERIZE_VERSION v0.6.1
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz

WORKDIR /nrl

COPY --from=builder /nrl/target/release/nrl /nrl/nrl
COPY docker/start.sh /nrl

ENV PORT=8000
ENV RUST_LOG=debug

CMD ["/nrl/start.sh"]
