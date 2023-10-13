# App 名稱
ARG APP_NAME=iron_llama
# 要複製進最終映像檔的模型名稱
ARG MODEL_NAME=Taiwan-LLaMa-13b-1.0.ggmlv3.q2_K.bin

# 建置階段，使用 Rust 每夜版
FROM rustlang/rust:nightly-bookworm-slim as build
ARG APP_NAME

# 指定工作位址並將所有東西複製到容器中
WORKDIR /app
COPY . .

# 安裝需要的工具
RUN apt-get update
RUN apt-get install -y pkg-config openssl libssl-dev curl wget

# 加上 WASM target
RUN rustup target add wasm32-unknown-unknown

# 安裝 cargo-binstall, 這可以使安裝其它 cargo 擴充套件如 cargo-leptos 更容易
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin
# 安裝 cargo-leptos
RUN cargo binstall cargo-leptos -y

# 建置 app
RUN cargo leptos build --release -vv

# 最終階段
FROM rustlang/rust:nightly-bookworm-slim as final
ARG APP_NAME
ARG MODEL_NAME

RUN apt-get update && apt-get install -y openssl

WORKDIR /app
# 把模型複製到 model
COPY --from=build /app/$MODEL_NAME model
# 把伺服器的二進制檔複製到 server 
COPY --from=build /app/target/server/release/$APP_NAME server
# 把 包含了 JS/WASM/CSS, etc. 的 /target/site 複製過來
COPY --from=build /app/target/site target/site

# 設定環境變數
ENV MODEL_PATH="/app/model"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"

# 設定使用者
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

# 更改權限
RUN chown -R appuser:appuser /app
RUN chmod -R 755 /app

# 切換使用者
USER appuser

# 暴露端口
EXPOSE 3000

# 跑起來！
CMD ["/app/server"]
