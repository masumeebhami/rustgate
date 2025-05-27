# 🚀 RustGate – A Lightweight API Gateway in Rust

**RustGate** is a high-performance, secure, and extensible API gateway written in Rust, using [Axum](https://docs.rs/axum), [Hyper](https://docs.rs/hyper), and [Tower](https://docs.rs/tower). It’s designed to route and proxy HTTP requests to internal microservices with built-in support for JWT authentication, Prometheus metrics, and rate limiting.

---

## ✨ Features

- 🔁 Reverse proxy with dynamic path-based routing
- 🔐 JWT-based authentication middleware
- 📊 Prometheus-compatible metrics at `/metrics`
- ⚙️ Configuration via `TOML` file
- ⚡ Built on async Rust (Tokio + Hyper 1.6)

---

## 🛠️ Getting Started

### Prerequisites

- Rust 1.76+
- `just` (optional, for command shortcuts)
- Docker (optional)

### Installation

```bash
git clone https://github.com/yourname/rustgate.git
cd rustgate
cargo build --release


## 🧪 Developer Shortcuts with `just`
This project includes a [`justfile`](https://github.com/casey/just) to simplify common workflows.

### Usage
Install `just` (if not already installed):

cargo install just
```

### Available Commands
```bash
just build           # Build Rust project
just run             # Run gateway locally
just docker-build    # Build Docker image
just docker-run      # Run container with config mounted
just compose-up      # Start services with Docker Compose
```