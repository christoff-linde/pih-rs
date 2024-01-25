-- Add up migration script here
CREATE TABLE IF NOT EXISTS sensors
(
    id       SERIAL PRIMARY KEY,
    type     VARCHAR(50),
    location VARCHAR(50)
);

CREATE TABLE IF NOT EXISTS  sensor_data
(
    time        TIMESTAMPTZ NOT NULL,
    sensor_id   INTEGER,
    temperature DOUBLE PRECISION,
    humidity    DOUBLE PRECISION,
    FOREIGN KEY (sensor_id) REFERENCES sensors (id)
);
