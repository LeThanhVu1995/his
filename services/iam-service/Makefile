run:
cargo run -p iam-service

check:
cargo check -p iam-service

fmt:
cargo fmt

migrate:
sqlx migrate run --source ./migrations --database-url "$$DATABASE_URL"
