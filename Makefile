DATABASE_DOCKER_CONTAINER=game-community-db

DATABASE_URL=postgres://root:root@localhost:5432/db

DATABASE_NAME=db
DATABASE_USER=root
DATABASE_PASSWORD=root
DATABASE_PORT=5432

.PHONY: i
i:
	cargo install cargo-edit
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add uuid --features "serde v4, v7, fast-rng"
	cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
	cargo add lazy_static
	cargo add jsonwebtoken
# SQLX-CLI
	cargo install sqlx-cli
# SeaORM
	cargo add sea-orm --features sqlx-postgres,runtime-tokio-rustls,macros,debug-print,with-chrono,with-uuid,with-json

# cargo
.PHONY: run
run:
	cargo run

.PHONY: build
build: 
	cargo build

# SQLX
.PHONY: create-migrations
create-migrations:
	sqlx migrate add -r init

.PHONY: migrate-up
migrate-up:
	sqlx migrate run --database-url ${DATABASE_URL}

.PHONY: migrate-down
migrate-down:
	sqlx migrate revert --database-url ${DATABASE_URL}

# Docker
.PHONY: build-no-cache
	docker-compose build --no-cache

.PHONY: up
up:
	docker-compose up -d

.PHONY: down
down:
	docker-compose down

.PHONY: stop
stop:
	docker-compose stop

.PHONY: restart
restart:
	docker-compose restart

.PHONY: clean
clean:
	docker system prune -a --volumes

.PHONY: exec
exec:
	docker-compose exec $(app) /bin/bash

.PHONY: db
db:
	docker exec -it ${DATABASE_DOCKER_CONTAINER} psql -U ${DATABASE_USER} -d ${DATABASE_NAME}

.PHONY: app
app:
	docker exec -it game-community-rust bash

.PHONY: ps
ps:
	docker ps

.PHONY: logs
logs:
	docker-compose logs ${DATABASE_DOCKER_CONTAINER}

