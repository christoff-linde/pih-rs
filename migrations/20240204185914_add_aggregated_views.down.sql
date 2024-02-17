-- Add down migration script here
SELECT remove_continuous_aggregate_policy('conditions_summary_30_seconds');
SELECT remove_continuous_aggregate_policy('conditions_summary_1_minutes');
SELECT remove_continuous_aggregate_policy('conditions_summary_5_minutes');
SELECT remove_continuous_aggregate_policy('conditions_summary_15_minutes');
SELECT remove_continuous_aggregate_policy('conditions_summary_30_minutes');
SELECT remove_continuous_aggregate_policy('conditions_summary_1_hours');
SELECT remove_continuous_aggregate_policy('conditions_summary_6_hours');
SELECT remove_continuous_aggregate_policy('conditions_summary_12_hours');
SELECT remove_continuous_aggregate_policy('conditions_summary_1_days');
SELECT remove_continuous_aggregate_policy('conditions_summary_2_days');
SELECT remove_continuous_aggregate_policy('conditions_summary_7_days');
SELECT remove_continuous_aggregate_policy('conditions_summary_1_weeks');
SELECT remove_continuous_aggregate_policy('conditions_summary_1_months');

DROP MATERIALIZED VIEW IF EXISTS conditions_summary_30_seconds;
DROP MATERIALIZED VIEW IF EXISTS conditions_summary_1_minutes;
DROP MATERIALIZED VIEW IF EXISTS conditions_summary_5_minutes;
DROP MATERIALIZED VIEW IF EXISTS conditions_summary_15_minutes;
DROP MATERIALIZED VIEW IF EXISTS conditions_summary_30_minutes;
DROP MATERIALIZED VIEW IF EXISTS conditions_summary_1_hours;
DROP MATERIALIZED VIEW IF EXISTS conditions_summary_6_hours;
DROP MATERIALIZED VIEW IF EXISTS conditions_summary_12_hours;
DROP MATERIALIZED VIEW IF EXISTS conditions_summary_1_days;
DROP MATERIALIZED VIEW IF EXISTS conditions_summary_2_days;
DROP MATERIALIZED VIEW IF EXISTS conditions_summary_7_days;
DROP MATERIALIZED VIEW IF EXISTS conditions_summary_1_weeks;
DROP MATERIALIZED VIEW IF EXISTS conditions_summary_1_months;