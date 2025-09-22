run:
	cargo run -p radiology-service

migrate:
	sqlx migrate run

fmt:
	cargo fmt --all

lint:
	cargo clippy -p radiology-service -- -D warnings

test:
	cargo test -p radiology-service

check:
	cargo check -p radiology-service
