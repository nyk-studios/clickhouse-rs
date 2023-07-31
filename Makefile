test: start-db
	cargo test -j 1 -- --nocapture && make stop-db

start-db:
	docker-compose up -d && sleep 5

stop-db:
	docker-compose down -v