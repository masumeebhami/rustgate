# Build the Rust project
build:
	cargo build --release

# Run the RustGate binary
run:
	cargo run --release

# Build and run Docker container
docker-build:
	docker build -t rustgate .

docker-run:
	docker run -p 8080:8080 -p 9090:9090 -v $(pwd)/config:/app/config rustgate

# Use Docker Compose
compose-up:
	docker-compose up --build

fmt:
    cargo fmt
check:
    cargo check