FROM rust:alpine AS builder

WORKDIR /workspace
COPY . .

RUN apk update && apk upgrade && apk upgrade --available && apk add \
  alpine-sdk \
  curl \
  wget \
  psmisc \
  webkit2gtk-4.1 \
  libressl-dev \
  gtk+3.0-dev \
  libayatana-appindicator-dev \
  librsvg-dev \
  xvfb

RUN rustup default nightly
RUN cargo install tauri-cli trunk cargo-watch
RUN cargo build
# CMD ["cargo", "run", "--bin", "server"]

FROM scratch
EXPOSE 6900
COPY --from=builder /workspace /workspace
CMD ["./target/debug/server"]
