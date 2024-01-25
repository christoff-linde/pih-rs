# Build and start all docker services
up:
  docker compose up --build -d

# Stop all docker services
down:
  docker compose down

# Start a shell on the API container
shell:
  docker exec -it pih-rs-api bash

# Start a shell on the DB container
shell-db:
  docker exec -it pih-rs-db psql -U postgres

# Run clippy on all code
lint:
  cargo clippy 

# Run the API service (without docker)
run:
  cargo run
