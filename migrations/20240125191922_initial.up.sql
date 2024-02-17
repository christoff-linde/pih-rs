-- Add up migration script here
CREATE TABLE IF NOT EXISTS sensors
(
    id       SERIAL PRIMARY KEY,
    type     VARCHAR(50),
    location VARCHAR(50)
);

CREATE TABLE IF NOT EXISTS  sensor_data
(
    time        TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    sensor_id   INTEGER,
    temperature DOUBLE PRECISION,
    humidity    DOUBLE PRECISION,
    FOREIGN KEY (sensor_id) REFERENCES sensors (id)
);

SELECT create_hypertable('sensor_data', 'time');

CREATE UNIQUE INDEX sensor_data_sensor_id_time_idx on sensor_data(sensor_id, time desc);