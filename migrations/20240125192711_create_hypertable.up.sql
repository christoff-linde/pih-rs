-- Add up migration script here
SELECT  create_hypertable('sensor_data', 'time');