# ビルド用のイメージ
FROM rust:1.79 as builder

WORKDIR /app

# 必要なパッケージのインストール
RUN apt-get update && apt-get install -y \
    clang \
    lld \
    libssl-dev \
    pkg-config \
    build-essential

# `sqlx-cli` のインストール
RUN cargo install sqlx-cli

# ソースコードのコピー
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# ビルド
RUN cargo build --release

# 最終イメージ
FROM debian:bookworm-slim

WORKDIR /app

# 必要なライブラリのインストール
RUN apt-get update && apt-get install -y \
    curl \
    libssl3 \
    ca-certificates \
    libclang-dev \
    libpq-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# ビルド済みバイナリのコピー
COPY --from=builder /app/target/release/game-community-postgresql /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/

# 実行コマンド
CMD ["game-community-postgresql"]
