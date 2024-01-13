FROM rust:latest

EXPOSE 8080
WORKDIR /workspace
COPY . .

RUN apt-get update -y && apt-get upgrade -y && apt-get install -y \
  build-essential \
  curl \
  wget \
  psmisc \
  libwebkit2gtk-4.0-dev \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  webkit2gtk-4.0-dev \
  webkit2gtk-driver \
  xvfb

# RUN rustup default nightly
RUN cargo install cargo-watch
RUN cargo build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN trunk build

RUN cargo install tauri-cli
RUN cargo tauri build

CMD ["cargo", "run", "--bin", "server"]
