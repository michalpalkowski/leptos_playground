FROM rust:1-alpine AS leptos
RUN apk add --no-cache build-base clang-dev curl

WORKDIR /app
COPY . .

RUN cargo install cargo-leptos@0.2.24

WORKDIR /app
COPY . .

WORKDIR /app
RUN cargo leptos build --release
CMD ["cargo", "leptos", "serve", "--release"]
