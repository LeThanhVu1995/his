run:
	cargo run -p order-service

migrate:
	sqlx migrate run

fmt:
	cargo fmt --all

lint:
	cargo clippy -p order-service -- -D warnings

test:
	cargo test -p order-service

build:
	cargo build -p order-service

clean:
	cargo clean -p order-service
