-- Add up migration script here
CREATE TABLE sensors
(
    id       SERIAL PRIMARY KEY,
    type     VARCHAR(50),
    location VARCHAR(50)
);

CREATE TABLE sensor_data
(
    time        TIMESTAMPTZ NOT NULL,
    sensor_id   INTEGER,
    temperature DOUBLE PRECISION,
    humidity    DOUBLE PRECISION,
    FOREIGN KEY (sensor_id) REFERENCES sensors (id)
);
