-- Add up migration script here
CREATE MATERIALIZED VIEW IF NOT EXISTS conditions_summary_30_seconds
            WITH (timescaledb.continuous) AS
SELECT sensor_data.sensor_id,
       time_bucket(INTERVAL '30 seconds', time) as bucket,
       AVG(temperature)                    as avg_temperature,
       MAX(temperature)                    as max_temperature,
       MIN(temperature)                    as min_temperature,
       AVG(humidity)                       as avg_humidity,
       MAX(humidity)                       as max_humidity,
       MIN(humidity)                       as min_humidity
FROM sensor_data
GROUP BY sensor_id, bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('conditions_summary_30_seconds',
                                       start_offset => null,
                                       end_offset => INTERVAL '30 seconds',
                                       schedule_interval => '30 seconds');


CREATE MATERIALIZED VIEW IF NOT EXISTS conditions_summary_1_minutes
            WITH (timescaledb.continuous) AS
SELECT sensor_data.sensor_id,
       time_bucket(INTERVAL '1 minutes', time) as bucket,
       AVG(temperature)                    as avg_temperature,
       MAX(temperature)                    as max_temperature,
       MIN(temperature)                    as min_temperature,
       AVG(humidity)                       as avg_humidity,
       MAX(humidity)                       as max_humidity,
       MIN(humidity)                       as min_humidity
FROM sensor_data
GROUP BY sensor_id, bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('conditions_summary_1_minutes',
                                       start_offset => null,
                                       end_offset => INTERVAL '1 minutes',
                                       schedule_interval => '1 minutes');


CREATE MATERIALIZED VIEW IF NOT EXISTS conditions_summary_5_minutes
    WITH (timescaledb.continuous) AS
SELECT sensor_data.sensor_id,
       time_bucket(INTERVAL '5 minutes', time) as bucket,
       AVG(temperature)                    as avg_temperature,
       MAX(temperature)                    as max_temperature,
       MIN(temperature)                    as min_temperature,
       AVG(humidity)                       as avg_humidity,
       MAX(humidity)                       as max_humidity,
       MIN(humidity)                       as min_humidity
FROM sensor_data
GROUP BY sensor_id, bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('conditions_summary_5_minutes',
                                       start_offset => null,
                                       end_offset => INTERVAL '5 minutes',
                                       schedule_interval => '5 minutes');


CREATE MATERIALIZED VIEW IF NOT EXISTS conditions_summary_15_minutes
            WITH (timescaledb.continuous) AS
SELECT sensor_data.sensor_id,
       time_bucket(INTERVAL '15 minutes', time) as bucket,
       AVG(temperature)                    as avg_temperature,
       MAX(temperature)                    as max_temperature,
       MIN(temperature)                    as min_temperature,
       AVG(humidity)                       as avg_humidity,
       MAX(humidity)                       as max_humidity,
       MIN(humidity)                       as min_humidity
FROM sensor_data
GROUP BY sensor_id, bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('conditions_summary_15_minutes',
                                       start_offset => null,
                                       end_offset => INTERVAL '15 minutes',
                                       schedule_interval => '15 minutes');


CREATE MATERIALIZED VIEW IF NOT EXISTS conditions_summary_30_minutes
            WITH (timescaledb.continuous) AS
SELECT sensor_data.sensor_id,
       time_bucket(INTERVAL '30 minutes', time) as bucket,
       AVG(temperature)                    as avg_temperature,
       MAX(temperature)                    as max_temperature,
       MIN(temperature)                    as min_temperature,
       AVG(humidity)                       as avg_humidity,
       MAX(humidity)                       as max_humidity,
       MIN(humidity)                       as min_humidity
FROM sensor_data
GROUP BY sensor_id, bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('conditions_summary_30_minutes',
                                       start_offset => null,
                                       end_offset => INTERVAL '30 days',
                                       schedule_interval => '5 minutes');


CREATE MATERIALIZED VIEW IF NOT EXISTS conditions_summary_1_hours
            WITH (timescaledb.continuous) AS
SELECT sensor_data.sensor_id,
       time_bucket(INTERVAL '1 hours', time) as bucket,
       AVG(temperature)                    as avg_temperature,
       MAX(temperature)                    as max_temperature,
       MIN(temperature)                    as min_temperature,
       AVG(humidity)                       as avg_humidity,
       MAX(humidity)                       as max_humidity,
       MIN(humidity)                       as min_humidity
FROM sensor_data
GROUP BY sensor_id, bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('conditions_summary_1_hours',
                                       start_offset => null,
                                       end_offset => INTERVAL '30 days',
                                       schedule_interval => '5 minutes');

CREATE MATERIALIZED VIEW IF NOT EXISTS conditions_summary_6_hours
            WITH (timescaledb.continuous) AS
SELECT sensor_data.sensor_id,
       time_bucket(INTERVAL '6 hours', time) as bucket,
       AVG(temperature)                    as avg_temperature,
       MAX(temperature)                    as max_temperature,
       MIN(temperature)                    as min_temperature,
       AVG(humidity)                       as avg_humidity,
       MAX(humidity)                       as max_humidity,
       MIN(humidity)                       as min_humidity
FROM sensor_data
GROUP BY sensor_id, bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('conditions_summary_6_hours',
                                       start_offset => null,
                                       end_offset => INTERVAL '30 days',
                                       schedule_interval => '30 minutes');


CREATE MATERIALIZED VIEW IF NOT EXISTS conditions_summary_12_hours
            WITH (timescaledb.continuous) AS
SELECT sensor_data.sensor_id,
       time_bucket(INTERVAL '12 hours', time) as bucket,
       AVG(temperature)                    as avg_temperature,
       MAX(temperature)                    as max_temperature,
       MIN(temperature)                    as min_temperature,
       AVG(humidity)                       as avg_humidity,
       MAX(humidity)                       as max_humidity,
       MIN(humidity)                       as min_humidity
FROM sensor_data
GROUP BY sensor_id, bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('conditions_summary_12_hours',
                                       start_offset => null,
                                       end_offset => INTERVAL '30 days',
                                       schedule_interval => '1 hours');


CREATE MATERIALIZED VIEW IF NOT EXISTS conditions_summary_1_days
            WITH (timescaledb.continuous) AS
SELECT sensor_data.sensor_id,
       time_bucket(INTERVAL '1 days', time) as bucket,
       AVG(temperature)                    as avg_temperature,
       MAX(temperature)                    as max_temperature,
       MIN(temperature)                    as min_temperature,
       AVG(humidity)                       as avg_humidity,
       MAX(humidity)                       as max_humidity,
       MIN(humidity)                       as min_humidity
FROM sensor_data
GROUP BY sensor_id, bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('conditions_summary_1_days',
                                       start_offset => null,
                                       end_offset => INTERVAL '30 days',
                                       schedule_interval => '1 hours');


CREATE MATERIALIZED VIEW IF NOT EXISTS conditions_summary_2_days
            WITH (timescaledb.continuous) AS
SELECT sensor_data.sensor_id,
       time_bucket(INTERVAL '2 days', time) as bucket,
       AVG(temperature)                    as avg_temperature,
       MAX(temperature)                    as max_temperature,
       MIN(temperature)                    as min_temperature,
       AVG(humidity)                       as avg_humidity,
       MAX(humidity)                       as max_humidity,
       MIN(humidity)                       as min_humidity
FROM sensor_data
GROUP BY sensor_id, bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('conditions_summary_2_days',
                                       start_offset => null,
                                       end_offset => INTERVAL '30 days',
                                       schedule_interval => '1 days');


CREATE MATERIALIZED VIEW IF NOT EXISTS conditions_summary_7_days
            WITH (timescaledb.continuous) AS
SELECT sensor_data.sensor_id,
       time_bucket(INTERVAL '7 days', time) as bucket,
       AVG(temperature)                    as avg_temperature,
       MAX(temperature)                    as max_temperature,
       MIN(temperature)                    as min_temperature,
       AVG(humidity)                       as avg_humidity,
       MAX(humidity)                       as max_humidity,
       MIN(humidity)                       as min_humidity
FROM sensor_data
GROUP BY sensor_id, bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('conditions_summary_7_days',
                                       start_offset => null,
                                       end_offset => INTERVAL '30 days',
                                       schedule_interval => '1 days');


CREATE MATERIALIZED VIEW IF NOT EXISTS conditions_summary_1_weeks
            WITH (timescaledb.continuous) AS
SELECT sensor_data.sensor_id,
       time_bucket(INTERVAL '1 weeks', time) as bucket,
       AVG(temperature)                    as avg_temperature,
       MAX(temperature)                    as max_temperature,
       MIN(temperature)                    as min_temperature,
       AVG(humidity)                       as avg_humidity,
       MAX(humidity)                       as max_humidity,
       MIN(humidity)                       as min_humidity
FROM sensor_data
GROUP BY sensor_id, bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('conditions_summary_1_weeks',
                                       start_offset => null,
                                       end_offset => INTERVAL '30 days',
                                       schedule_interval => '1 days');


CREATE MATERIALIZED VIEW IF NOT EXISTS conditions_summary_1_months
            WITH (timescaledb.continuous) AS
SELECT sensor_data.sensor_id,
       time_bucket(INTERVAL '1 months', time) as bucket,
       AVG(temperature)                    as avg_temperature,
       MAX(temperature)                    as max_temperature,
       MIN(temperature)                    as min_temperature,
       AVG(humidity)                       as avg_humidity,
       MAX(humidity)                       as max_humidity,
       MIN(humidity)                       as min_humidity
FROM sensor_data
GROUP BY sensor_id, bucket
WITH NO DATA;

SELECT add_continuous_aggregate_policy('conditions_summary_1_months',
                                       start_offset => null,
                                       end_offset => INTERVAL '3 months',
                                       schedule_interval => '1 days');

