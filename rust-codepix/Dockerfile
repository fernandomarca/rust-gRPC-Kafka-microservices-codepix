FROM rust:1.60.0
WORKDIR /rust
ENV PATH="/rust/bin:${PATH}"
RUN apt-get update && apt-get install -y build-essential \
  curl \
  protobuf-compiler \ 
  librdkafka-dev \
  cmake \
  openssl \
  libssl-dev \
  libsasl2-dev \
  pkg-config \
  libzstd-dev

RUN cargo install diesel_cli

CMD ["tail", "-f", "/dev/null"]