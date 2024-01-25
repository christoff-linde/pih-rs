-- Add up migration script here
CREATE UNIQUE INDEX sensor_data_sensor_id_time_idx on sensor_data (sensor_id, time desc);
