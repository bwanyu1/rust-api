FROM debian:bookworm-slim

# --- Build arguments for non-root user ---
ARG USERNAME=rust
ARG UID=1000
ARG GID=1000

# --- Base deps commonly needed by Rust crates ---
RUN set -eux; \
    apt-get update; \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      ca-certificates \
      curl \
      build-essential \
      pkg-config \
      libssl-dev \
      git \
      cmake \
      clang \
    ; \
    rm -rf /var/lib/apt/lists/*

# --- Create non-root user ---
RUN set -eux; \
    groupadd -g ${GID} ${USERNAME}; \
    useradd -m -u ${UID} -g ${GID} -s /bin/bash ${USERNAME}

USER ${USERNAME}
WORKDIR /app

# --- Environment for rustup/cargo ---
ENV HOME=/home/${USERNAME}
ENV CARGO_HOME=/home/${USERNAME}/.cargo \
    RUSTUP_HOME=/home/${USERNAME}/.rustup \
    PATH=/home/${USERNAME}/.cargo/bin:${PATH}

SHELL ["/bin/bash", "-lc"]

# --- Install Rust toolchain (stable) via rustup ---
RUN set -eux; \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable; \
    . "$HOME/.cargo/env"; \
    rustc --version; cargo --version; \
    rustup component add clippy rustfmt

# Default command opens a shell with Rust available
CMD ["bash"]
