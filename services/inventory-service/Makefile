run:
	cargo run -p inventory-service

migrate:
	sqlx migrate run

fmt:
	cargo fmt --all

lint:
	cargo clippy -p inventory-service -- -D warnings
