run:
	cargo run -p pharmacy-service
migrate:
	sqlx migrate run
fmt:
	cargo fmt --all
lint:
	cargo clippy -p pharmacy-service -- -D warnings
