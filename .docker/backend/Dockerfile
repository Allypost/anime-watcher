FROM rust:1.68-alpine3.17 AS builder
RUN apk add --no-cache musl-dev
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/app
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
COPY Cargo.* ./
RUN cargo fetch --locked
COPY src/ ./src/
ENV RUSTFLAGS='-C target-feature=+crt-static'
RUN cargo install --path . --target=x86_64-unknown-linux-musl --bins

FROM alpine as runner
ARG bin_name=anime-watcher-backend
COPY --from=builder /usr/local/cargo/bin/${bin_name} /usr/local/bin/
ARG username=app
ARG user_id=1000
ARG group_id=1000
RUN addgroup -g ${group_id} ${username}
RUN adduser -D -u ${user_id} -G ${username} ${username}
USER ${username}
ENTRYPOINT [${bin_name}]
CMD [ "--help" ]