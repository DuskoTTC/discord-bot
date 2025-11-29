# Compile environment
FROM rust:1.91.1-bookworm as builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libopus-dev \
    cmake \
    && rm -rf /var/lib/apt/lists/* 

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {println!(\"If you see this, the build broke\")}" > src/main.rs && \
    cargo build --release 

COPY src ./src

RUN touch src/main.rs && cargo build --release 

# Runtime environment
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    ffmpeg \
    python3 \
    python3-pip \
    libopus0 \
    && rm -rf /var/lib/apt/lists/* 

RUN rm /usr/lib/python3.11/EXTERNALLY-MANAGED || true
RUN pip3 install --no-cache-dir yt-dlp

WORKDIR /app

COPY --from=builder /app/target/release/dttc /app/dttc

CMD ["./dttc"]
