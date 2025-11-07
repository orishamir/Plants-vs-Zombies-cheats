FROM rust:1.90.0-trixie

WORKDIR /tmp

RUN apt-get update
RUN apt-get install -y --no-install-recommends \
    gcc-mingw-w64-i686 g++-mingw-w64-i686 binutils-mingw-w64-i686 \
    pkg-config # optional
RUN rustup target add i686-pc-windows-gnu

WORKDIR /app
