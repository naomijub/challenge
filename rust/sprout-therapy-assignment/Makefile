run-local:
	cargo run --release

run:
	docker-compose up web --build

down:
	docker-compose down

test:
	docker-compose up test

integration:
	cargo test --test mod --no-fail-fast

unit:
	cargo test  --no-fail-fast --lib

test-local: unit integration