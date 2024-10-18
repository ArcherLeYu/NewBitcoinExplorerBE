# 使用 Rust 官方镜像作为基础镜像
FROM rust:latest as builder

# 创建一个新的工作目录
WORKDIR /usr/src/bitcoin_explorer_part1

# 复制你的 Cargo 配置文件
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY src ./src

# 构建项目，这里使用 release 配置以优化编译结果
RUN cargo build --release

# 使用 debian:bookworm-slim 镜像作为最终运行环境
FROM debian:bookworm-slim

# 安装必要的运行时库，包括 OpenSSL 3
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /usr/local/bin

# 从构建阶段复制构建好的可执行文件到当前工作目录
COPY --from=builder /usr/src/bitcoin_explorer_part1/target/release/bitcoin_explorer_part1 .

# 暴露需要的端口，这里假设是 8081
EXPOSE 8081

# 运行可执行文件
CMD ["./bitcoin_explorer_part1"]
