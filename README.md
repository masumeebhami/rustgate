# 🚀 RustGate – A Lightweight API Gateway in Rust

**RustGate** is a high-performance, secure, and extensible API gateway written in Rust, using [Axum](https://docs.rs/axum), [Tower](https://docs.rs/tower). It routes HTTP traffic to microservices with built-in support for JWT authentication, Prometheus metrics, and dynamic service configuration.

---

## ✨ Features

- 🔁 Reverse proxy with dynamic path-based routing via `/api/{service}/{path..}`
- 🔐 JWT-based authentication middleware (optional with `Authorization` header)
- 📊 Prometheus-compatible metrics exposed at `/metrics`
- ⚙️ Configuration via `TOML` (`config/config.toml`)
- 🧩 Built with async Rust (Tokio + Hyper 1.6)
- 🧪 Developer-friendly tooling with `just`

---

## 🛠️ Getting Started

### 📦 Prerequisites

- Rust 1.76+
- [`just`](https://github.com/casey/just) (optional, for task automation)
- Docker (optional, for microservice mocks or full stack)

---

### 🔧 Configuration

Create a `config/config.toml`:

```toml
[server]
address = "0.0.0.0"
port = 8080

[services]
users = "http://localhost:8001"
payments = "http://localhost:8002"
```

Each incoming request like:

```
GET /api/users/profile
```

Will be proxied to:

```
GET http://localhost:8001/profile
```

---

### 🚀 Run Locally

```bash
cargo run
```

You should see:

```
📊 Metrics server listening on http://0.0.0.0:9090
🚀 RustGate listening on http://0.0.0.0:8080
```

---

### 🔐 Testing Auth-Protected Routes

By default, all routes under `/api/*` require a valid JWT:

```bash
curl -H "Authorization: Bearer <your_jwt>" http://localhost:8080/api/users/test
```

To disable auth temporarily, comment out `jwt_auth` middleware in `router.rs`.

---

## 📊 Metrics

Visit:

```bash
http://localhost:9090/metrics
```

To see Prometheus-compatible metrics.

---

## 🧪 Developer Shortcuts with `just`

Install `just`:

```bash
cargo install just
```

### 🔧 Available Commands

```bash
just build           # Build the Rust project
just run             # Run gateway locally
just docker-build    # Build the Docker image
just docker-run      # Run container with config mounted
just compose-up      # Start services with Docker Compose
```

---

## 🤝 Contributing

PRs and feedback welcome! Feel free to open issues or suggestions.