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

# Convert all .csv files in data/ to .parquet using csv2parquet
convert:
  @for file in data/*.csv; do \
    echo "Converting $file to Parquet"; \
    csv2parquet $file "data/$(basename $file .csv).parquet"; \
  done

  @for file in data/db_dumps/*.csv; do \
    echo "Converting $file to Parquet"; \
    csv2parquet $file "data/db_dumps/$(basename $file .csv).parquet"; \
  done
