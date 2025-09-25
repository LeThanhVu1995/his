CREATE SCHEMA IF NOT EXISTS rpt;

CREATE MATERIALIZED VIEW IF NOT EXISTS rpt.mv_revenue_daily AS
SELECT date_trunc('day', i.issued_at) AS day,
       sum(i.total_amount) AS revenue,
       count(*) AS invoices
FROM staging_invoices i
GROUP BY 1
WITH NO DATA;
CREATE UNIQUE INDEX IF NOT EXISTS idx_mv_revenue_daily_day ON rpt.mv_revenue_daily(day);

CREATE MATERIALIZED VIEW IF NOT EXISTS rpt.mv_visits_by_dept AS
SELECT e.department_id, d.name AS department_name,
       date_trunc('day', e.started_at) AS day,
       count(*) AS visits
FROM staging_encounters e
LEFT JOIN staging_departments d ON d.id = e.department_id
GROUP BY 1,2,3
WITH NO DATA;
CREATE INDEX IF NOT EXISTS idx_mv_visits_by_dept_day ON rpt.mv_visits_by_dept(day);

CREATE MATERIALIZED VIEW IF NOT EXISTS rpt.mv_abnormal_lab_daily AS
SELECT date_trunc('day', r.reported_at) AS day,
       count(*) FILTER (WHERE r.flag = 'H' OR r.flag = 'L') AS abnormal_count,
       count(*) AS total
FROM staging_lab_results r
GROUP BY 1
WITH NO DATA;
CREATE UNIQUE INDEX IF NOT EXISTS idx_mv_abnormal_lab_day ON rpt.mv_abnormal_lab_daily(day);

CREATE OR REPLACE FUNCTION rpt.refresh_all()
RETURNS void LANGUAGE plpgsql AS UTF8
BEGIN
  REFRESH MATERIALIZED VIEW CONCURRENTLY rpt.mv_revenue_daily;
  REFRESH MATERIALIZED VIEW CONCURRENTLY rpt.mv_visits_by_dept;
  REFRESH MATERIALIZED VIEW CONCURRENTLY rpt.mv_abnormal_lab_daily;
END; UTF8;
