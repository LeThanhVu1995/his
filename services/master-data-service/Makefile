run:
	cargo run -p master-data-service

test:
	cargo test -p master-data-service

migrate:
	sqlx migrate run

lint:
	cargo clippy -p master-data-service -- -D warnings

fmt:
	cargo fmt --all
